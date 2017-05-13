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

pub struct Interconnect {
    /// Cartridge
    cartridge: Cartridge,
    /// I/O ports
    io: Vec<u8>,
    /// Internal RAM
    hram: Vec<u8>,
    /// Interrupt Enable Register
    pub inte: u8,
    /// Interrupt Flag
    pub intf: u8,
    // Work RAM
    iram: Ram,
    // GPU
    gpu: Gpu,
    // Timer
    timer: Timer,
    // Sound
    sound: Sound,
    // Serial
    serial: Serial,
    // Bootrom is mapped
    bootrom: bool
}

impl Interconnect {
    pub fn new(cartridge: Cartridge, gpu: Gpu) -> Interconnect {
        // creates a new player
        let player = CpalPlayer::get();

        Interconnect {
            cartridge: cartridge,
            io: vec![0x20; 0x7f],
            hram: vec![0x20; 0x7f],
            inte: 0,
            intf: 0,
            iram: Ram::new(0x2000),
            gpu: gpu,
            timer: Timer::new(),
            sound: Sound::new(Box::new(player.expect("Player is mandatory")) as Box<AudioPlayer>),
            serial: Serial::new(),
            bootrom: true
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

        // IO
        if let Some(off) = map::in_range(address, map::IO) {
            return self.read_io(off);
        }


        // --- Old Implementation

        match address {
            // Keypad
            0xff00 => panic!("TODO: read keypad"),
            // Serial
            0xff01 ... 0xff02 => self.serial.read_byte(address),
            // Time
            0xff04 ... 0xff07 => self.timer.read_byte(address),
            // Interrupt Flags
            0xff0f => self.intf,
            // Infrared (Implementation don't needed)
            0xff56 => { 0 }
            // High RAM
            0xff80 ... 0xfffe => self.hram[address as usize & 0x007f],
            // IE (Interrupt Enable)
            0xffff => self.inte,
            // invalid address
            _ => { panic!("Read from an unrecognized address: {:#x}", address); }
        }
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

        // --- OLD

        match address {
            // Serial
            0xff01 ... 0xff02 => self.serial.write_byte(address, value),
            // Timer
            0xff04 ... 0xff07 => self.timer.write_byte(address, value),
            // Interrupt Flags
            0xff0f => self.intf = value,
            // Sound
            0xff10 ... 0xff3f => self.sound.write_byte(address, value),
            0xff10 ... 0xff3f => {
                // TODO
            }
            // GPU
            0xff40 ... 0xff4b => self.gpu.write_byte(address, value),
            // Infrared (Implementation don't needed)
            0xff56 => {}
            // High RAM
            0xff80 ... 0xfffe => self.hram[address as usize & 0x007f] = value,
            // Interrupt Enable
            0xffff => self.inte = value,
            _ => { panic!("Write for an unrecognized address: {:#x}", address); }
        }
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
            // 0x10 ... 0x3f => self.sound.write_byte(address, value),
            0x10 ... 0x3f => {
                // TODO
            },
            // GPU registers
            0x40 ... 0x4b => self.gpu.write_byte(address, value),
            _ => {}
        }
    }
}
