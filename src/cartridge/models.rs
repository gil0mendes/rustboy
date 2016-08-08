//! Cartridge model specific emulation

use super::{Cartridge, ROM_BANK_SIZE};

// --------------------------------------------------------- [Common]

/// Interface to model-specific operations
#[derive(Copy)]
pub struct Model {
  /// Handle ROM write
  pub write_rom: fn(cart: &mut Cartridge, offset: u16, value: u8),
  /// Handle RAM write
  pub write_ram: fn(cart: &mut Cartridge, address: u32, value: u8),
  /// Handle RAM read
  pub read_ram: fn(cart: &Cartridge, address: u32) -> u8,
}

/// Implement clone trait on the Model
impl ::std::clone::Clone for Model {
  fn clone(&self) -> Model {
    Model {
      write_rom: self.write_rom,
      write_ram: self.write_ram,
      read_ram: self.read_ram
    }
  }
}

/// Default implementation of write_ram, suitable for most cartridges
fn write_ram(cart: &mut Cartridge, address: u32, value: u8) {
  if let Some(byte) = cart.ram_byte_absolute_mut(address) {
    *byte = value;
  }
}

/// Default implementation of read_ram, suitable for most cartridges
fn read_ram(cart: &Cartridge, address: u32) -> u8 {
  cart.ram_byte_absolute(address)
}

/// Get the correspondent model for the given cartridge type
pub fn from_id(id: u8) -> Model {
  match id {
    0x00 => mbc0::MODEL,
    0x01 => mbc1::MODEL,
    _ => panic!("Cartridge model {:#x} not implemented", id),
  }
}

// --------------------------------------------------------- [MBC0]

mod mbc0 {
  use super::Model;
  use cartridge::Cartridge;

  fn write_rom(_: &mut Cartridge, offset: u16, value: u8) {
    panic!("Unhandled ROM write: {:04x} {:02x}", offset, value);
  }

  pub static MODEL: Model = Model {
    write_rom: write_rom,
    write_ram: super::write_ram,
    read_ram: super::read_ram
  };
}

mod mbc1 {
  use super::Model;
  use cartridge::{Cartridge, ROM_BANK_SIZE};

  fn write_rom(cart: &mut Cartridge, offset: u16, value: u8) {
    match offset {
      0x0000 ... 0x1fff => {
        // Writing a low nibble 0xa to anywhere in that address 
        // range removes RAM write protect, all other values 
        // enable it.
        cart.set_ram_wp(value & 0xf != 0xa)
      },
      0x2000 ... 0x3fff => {
        // Select a new ROM bank, bits [4:0]
        let cur_bank = cart.rom_bank() & !0x1f;

        // New bank to select
        let bank = cur_bank | (value & 0x1f);

        // select the new rom bank
        set_rom_bank(cart, bank);
      },
      0x4000 ... 0x5fff => {
        if cart.bank_ram() {
          // select a new RAM bank
          cart.set_ram_bank(value & 0x3);
        } else {
          // select a new ROM bank, bits [6:5]
          let cur_bank = cart.rom_bank() & !0x60;

          let bank = cur_bank | ((value << 5) & 0x60);

          // set the new rom bank
          set_rom_bank(cart, bank);
        }
      },
      0x6000 ... 0x7fff => {
        // switch RAM/ROM banking mode
        cart.set_bank_ram(value & 1 != 0)
      },
      _ => panic!("Unhandled ROM write: {:04x} {:02x}", offset, value),
    }
  }

  /// I copy this code from another Game Boy Emulator, some 
  /// games crash if the is not well done
  fn set_rom_bank(cart: &mut Cartridge, bank: u8) {
    // set the select rom bank
    cart.set_rom_bank(bank);

    let bank = if bank & 0x1f != 0 { bank } else { bank | 1 };

    // If the bank overflows we wrap it around. This assumes that
    // MBC1 cart can only have a power of two number of banks.
    let bank = bank & (cart.rom_banks() - 1);

    // Same as super::set_rom_bank: we already have a one bank
    // offset in the CPU address when accessing bankable ROM.
    let bank = (bank as i32) - 1;

    // compute the rom offset
    let rom_offset = ROM_BANK_SIZE * bank;

    // set the rom offset
    cart.set_rom_offset(rom_offset);
  }

  pub static MODEL: Model = Model {
    write_rom: write_rom,
    write_ram: super::write_ram,
    read_ram: super::read_ram,
  };
}

