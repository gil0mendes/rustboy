//! Input/Output abstraction for memory, ROM, and I/O mapped registers.

use gpu::Gpu;
use self::ram::Ram;
use self::io_map::*;
use super::sound::AudioPlayer;
use super::sound::CpalPlayer;
use super::sound::Sound;
use self::timer::Timer;
use self::serial::Serial;
use cartridge::Cartridge;

mod map;
mod ram;
pub mod io_map;
mod bootrom;

mod timer;
mod serial;

#[derive(PartialEq, Copy, Clone)]
pub enum GbSpeed {
    Single,
    Double,
}

pub struct Interconnect {
    /// Cartridge
    cartridge: Cartridge,
    /// I/O ports
    io: Vec<u8>,
    /// Interrupt Enable Register
    pub inte: u8,
    /// Interrupt Flag
    pub intf: u8,
    // Work RAM
    iram: Ram,
    // 0-page RAM
    zpage: Ram,
    // GPU
    gpu: Gpu,
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
    wrambank: usize
}

impl Interconnect {
    pub fn new(cartridge: Cartridge, gpu: Gpu) -> Interconnect {
        // creates a new player
        let player = CpalPlayer::get();

        Interconnect {
            cartridge: cartridge,
            io: vec![0x20; 0x7f],
            inte: 0,
            intf: 0,
            iram: Ram::new(0x2000),
            zpage: Ram::new(0x7f),
            gpu: gpu,
            timer: Timer::new(),
            sound: Sound::new(Box::new(player.expect("Player is mandatory")) as Box<AudioPlayer>),
            serial: Serial::new(),
            bootrom: true,
            gbspeed: GbSpeed::Single,
            speed_switch_req: false,
            wrambank: 1
        }
    }

    pub fn do_cycle(&mut self, ticks: u32) {
        // TODO: this must use Game Boy Speed and VRAM ticks
        let cpu_ticks = ticks;

        // timer
        self.timer.do_cycle(cpu_ticks);
        self.inte |= self.timer.interrupt;
        self.timer.interrupt = 0;
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
        if let Some(off) = map::in_range(address, map::VRAM) {
            return self.gpu.vram(off);
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
            panic!("TODO: implement OEM");
        }

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
            return self.inte;
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

        // VRAM
        if let Some(off) = map::in_range(address, map::VRAM) {
            return self.gpu.set_vram(off, value);
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
            panic!("TODO: implement OEM");
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
            // TODO implement interrupt
            return self.inte = value;
        }

        panic!("Write for an unrecognized address: {:04x}", address);

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

    // ----------------------------------------------------- [IO Ports]

    /// Read a byte from the IO ports
    fn read_io(&self, address: u16) -> u8 {
        match address {
            // Keypad
            0x00 => panic!("TODO read keypad"),
            // Serial
            0x01 ... 0x02 => self.serial.read_byte(0xff00 | address),
            // Timer
            0x04 ... 0x07 => self.timer.read_byte(0xff00 | address),
            // Interrupt flags
            0x0f => self.intf,
            // Sound registers
            0x10 ... 0x3f => self.sound.read_byte(address),
            0x10 ... 0x3f => {
                // TODO
                0x00
            },
            // GPU registers
            0x40 ... 0x4b => self.gpu.read_byte(address),
            _ => 0x00
        }
    }

    /// Write a byte to the IO ports
    fn write_io(&mut self, address: u16, value: u8) {
        match address {
            // Keypad
            0x00 => panic!("TODO write keypad"),
            // Serial
            0x01 ... 0x02 => self.serial.write_byte(0xff00 | address, value),
            // Timer
            0x04 ... 0x07 => self.timer.write_byte(0xff00 | address, value),
            // Interrupt flags
            0x0f => self.intf = value,
            // Sound registers
            0x10 ... 0x3f => self.sound.write_byte(address, value),
            // GPU registers
            0x40 ... 0x4b => self.gpu.write_byte(address, value),
            _ => {
                panic!("Writing to na IO address not handled: {:04x}", address);
            }
        }
    }
}
