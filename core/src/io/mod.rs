//! Input/Output abstraction for memory, ROM, and I/O mapped registers.

use cartridge::Cartridge;
use gpu::Gpu;
use gpu::types;
use super::sound::AudioPlayer;
use super::sound::CpalPlayer;
use super::sound::Sound;
use self::io_map::*;
use self::irq::{Irq, Interrupt};
use self::ram::Ram;
use self::serial::Serial;
use self::timer::Timer;
use self::joypad::Joypad;

mod map;
mod ram;
pub mod io_map;
mod bootrom;
mod irq;

mod timer;
mod serial;
mod joypad;

#[derive(PartialEq, Copy, Clone)]
pub enum GbSpeed {
    Single,
    Double,
}

#[derive(PartialEq)]
enum DMAType {
    OAM,
    GDMA,
    HDMA,
}

pub struct Interconnect {
    /// Cartridge
    cartridge: Cartridge,
    /// I/O ports
    io: Vec<u8>,
    /// Interrupt module
    pub irq: Irq,
    // Work RAM
    iram: Ram,
    // 0-page RAM
    zpage: Ram,
    // GPU
    pub gpu: Gpu,
    // Timer
    timer: Timer,
    // Sound
    sound: Sound,
    // Serial
    serial: Serial,
    // Bootrom is mapped
    bootrom: bool,
    // GB speed mode
    gbspeed: GbSpeed,
    // Speed switch request
    speed_switch_req: bool,
    // Working RAM Bank
    wrambank: usize,
    joypad: Joypad,
    // DMA state
    dma_status: DMAType,
    dma_src: u16,
    dma_dst: u16,
    dma_len: u8,
}

impl Interconnect {
    pub fn new(cartridge: Cartridge, gpu: Gpu) -> Interconnect {
        // creates a new player
        let player = CpalPlayer::get();

        Interconnect {
            cartridge,
            io: vec![0x20; 0x7f],
            irq: Irq::new(),
            iram: Ram::new(0x2000),
            zpage: Ram::new(0x7f),
            gpu,
            timer: Timer::new(),
            sound: Sound::new(Box::new(player.expect("Player is mandatory")) as Box<AudioPlayer>),
            serial: Serial::new(),
            bootrom: true,
            gbspeed: GbSpeed::Single,
            speed_switch_req: false,
            wrambank: 1,
            dma_status: DMAType::OAM,
            joypad: Joypad::new(),
            dma_src: 0,
            dma_dst: 0,
            dma_len: 0xff,
        }
    }

    fn perform_vramdma(&mut self) -> u32 {
        match self.dma_status {
            DMAType::OAM => 0,
            DMAType::GDMA => panic!("TODO: implement GDMA"),
            DMAType::HDMA => panic!("TODO: implement HDMA"),
        }
    }

    pub fn do_cycle(&mut self, ticks: u32) -> u32 {
        let cpudivider = match self.gbspeed {
            GbSpeed::Single => 1,
            GbSpeed::Double => 2,
        };

        // Perform VRAM DMA and compute trick times
        let vramticks = self.perform_vramdma();
        let gputricks = ticks / cpudivider + vramticks;
        let cpu_ticks = ticks + vramticks * cpudivider;

        self.timer.do_cycle(cpu_ticks, &mut self.irq);

        // TODO: Keypad
        
        // GPU cycle
        self.gpu.do_cycle(gputricks);

        // TODO: sound cycle

        // TODO: serial cycle

        return gputricks;
    }

    pub fn switch_speed(&mut self) {
        if self.speed_switch_req {
            if self.gbspeed == GbSpeed::Double {
                self.gbspeed = GbSpeed::Single;
            } else {
                self.gbspeed = GbSpeed::Double;
            }
        }

        self.speed_switch_req = false;
    }

    /// read a byte from the interconnect
    pub fn read_byte(&self, address: u16) -> u8 {
        // ROM
        if let Some(off) = map::in_range(address, map::ROM) {
            // bootrom is still mapped, read from it
            if self.bootrom && off < 0x100 {
                return bootrom::BOOTROM[off as usize];
            }

            // read a byte from the cartridge
            return self.cartridge.read_byte(off);
        }

        // VRAM
        if let Some(off) = map::in_range(address, map::CHAR_RAM) {
            return self.gpu.read_character_ram(off);
        }

        // V_TILE_MAP1
        if let Some(off) = map::in_range(address, map::V_TILE_MAP1) {
            return self.gpu.read_tile_map1(off);
        }

        // V_TILE_MAP2
        if let Some(off) = map::in_range(address, map::V_TILE_MAP2) {
            return self.gpu.read_tile_map2(off);
        }

        // RAM bank
        if let Some(off) = map::in_range(address, map::RAM_BANK) {
            return self.cartridge.ram_byte(off);
        }

        // Internal RAM
        if let Some(off) = map::in_range(address, map::IRAM){
            return self.iram.byte(off);
        }

        // Internal RAM Echo
        if let Some(off) = map::in_range(address, map::IRAM_ECHO) {
            return self.iram.byte(off);
        }

        // Object Attribute Mapping
        if let Some(off) = map::in_range(address, map::OAM) {
            return match self.dma_status {
                DMAType::OAM => 0xff,
                _ => self.gpu.read_oam(off),
            };
        }

        // Empty I/O zone.
        if let Some(off) = map::in_range(address, map::EMPTY_RAM) {
            return 0;
        };

        // IO
        if let Some(off) = map::in_range(address, map::IO) {
            return self.read_io(off);
        }

        // Working RAM Bank Number
        if address == map::WRAMBANK {
            return self.wrambank as u8;
        }

        // Zero Page (High RAM)
        if let Some(off) = map::in_range(address, map::ZERO_PAGE) {
             return self.zpage.byte(off);
        }

        // IE (Interrupt Enable)
        if address == map::IEN {
            return self.irq.get_interrupt_enabled();
        }

        // Infrared (Implementation don't needed)
        // 0xff56 => { 0 }

        panic!("Read from an unrecognized address: {:04x}", address);
    }

    /// read a word from the interconnect
    pub fn read_word(&self, addr: u16) -> u16 {
        (self.read_byte(addr) as u16) | ((self.read_byte(addr + 1) as u16) << 8)
    }

    /// write a byte to the interconnect
    pub fn write_byte(&mut self, address: u16, value: u8) {
        // ROM
        if let Some(off) = map::in_range(address, map::ROM) {
            return self.cartridge.set_rom_byte(off, value);
        }

        // GPU Character RAM
        if let Some(off) = map::in_range(address, map::CHAR_RAM) {
            return self.gpu.write_character_ram(off, value);
        }

        // GPU Tile Map 1
        if let Some(off) = map::in_range(address, map::V_TILE_MAP1) {
            return self.gpu.write_tile_map1(off, value);
        }

        // GPU Tile Map 2
        if let Some(off) = map::in_range(address, map::V_TILE_MAP2) {
            return self.gpu.write_tile_map2(off, value);
        }

        // RAM BANK
        if let Some(off) = map::in_range(address, map::RAM_BANK) {
            return self.cartridge.set_ram_byte(off, value);
        }

        // IRAM
        if let Some(off) = map::in_range(address, map::IRAM) {
            return self.iram.set_byte(off, value);
        }

        // IRAM Echo
        if let Some(off) = map::in_range(address, map::IRAM_ECHO) {
            return self.iram.set_byte(off, value);
        }

        // Object Attribute Mapping
        if let Some(off) = map::in_range(address, map::OAM) {
            panic!("TODO: implement OAM2");
        }

        // IO
        if let Some(off) = map::in_range(address, map::IO) {
            return self.write_io(off, value);
        }

        // Working RAM Bank Number
        if address == map::WRAMBANK {
            return self.wrambank = value as usize;
        }

        // Zero Page (High RAM)
        if let Some(off) = map::in_range(address, map::ZERO_PAGE) {
            return self.zpage.set_byte(off, value);
        }

        // Interrupt Enable
        if address == map::IEN {
            return self.irq.set_interrupt_enabled(value);
        }

        // TODO: organize this code a bit better
        if address == 0xff4f {
            return self.gpu.set_vramBank(value);
        }

        panic!("Unsupported write at ${:04x} = {:02x}", address, value);

        // Infrared (Implementation don't needed)
        // 0xff56 => {}
    }

    /// write a word in memory
    ///
    /// # Arguments
    /// - addr
    /// - value
    pub fn write_word(&mut self, addr: u16, value: u16) {
        self.write_byte(addr, (value & 0xff) as u8);
        self.write_byte(addr + 1, (value >> 8) as u8)
    }

    /// Read a byte from the IO ports
    fn read_io(&self, address: u16) -> u8 {
        match address {
            // Keypad
            0x00 => self.joypad.get_register(),
            // Serial
            0x01 ... 0x02 => self.serial.read_byte(0xff00 | address),
            0x03 => 0,
            // Timer
            0x04 ... 0x07 => self.timer.read_byte(0xff00 | address),
            0x08 ... 0x0e => 0,
            // Interrupt flags
            0x0f => self.irq.get_interrupt_flag(),
            // Sound registers
            0x10 ... 0x3f => self.sound.read_byte(address),
            // Read data from the VRAM
            0x40 ... 0x4f => self.gpu.read_byte(address),
            0x51 ... 0x7f => 0,
            // GPU registers
//            0x40 ... 0x4b => self.gpu.read_byte(address),
            _ => {
                panic!("Read to a IO address not handled: {:#x}", address);
            }
        }
    }

    /// Write a byte to the IO ports
    fn write_io(&mut self, address: u16, value: u8) {
        match address {
            // Keypad
            0x00 => self.joypad.set_register(value),
            // Serial
            0x01 ... 0x02 => self.serial.write_byte(0xff00 | address, value),
            // Timer
            0x04 ... 0x07 => self.timer.write_byte(0xff00 | address, value),
            // Interrupt flags
            0x0f => self.irq.set_interrupt_flag(value),
            // Sound registers
            0x10 ... 0x3f => self.sound.write_byte(address, value),
            0x42 => self.gpu.set_scroll_y(value),
            0x43 => self.gpu.set_scroll_x(value),
            0x47 => self.gpu.set_bg_palette(value),
            0x4a => self.gpu.set_window_y(value),
            0x4b => self.gpu.set_window_x(value),
            _ => {
                panic!("Writing to a IO address not handled: {:#x}", address);
            }
        }
    }

    pub fn screen_buffer(&self) -> &types::ScreenBuffer {
        &self.gpu.back_buffer
    }
}
