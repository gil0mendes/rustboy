//! IO Address Map (offset from 0xff00)

/// Input button matrix control
pub const INPUT: u16 = 0x00;


/// Serial data
pub const SB: u16 = 0x01;
/// Serial control
pub const SC: u16 = 0x02;


/// 16.384kHz free-running counter. Writing to it resets it to 0.
pub const DIV: u16 = 0x04;
/// Configurable timer counter
pub const TIMA: u16 = 0x05;
/// Configurable timer modulo (value reloaded in the counter after
/// oveflow)
pub const TMA: u16 = 0x06;
/// Timer control register
pub const TAC: u16 = 0x07;


/// Interrupt Flag register
pub const IF: u16 = 0x0f;


/// Sound channel 1 register 0
pub const NR10: u16 = 0x10;
/// Sound channel 1 register 1
pub const NR11: u16 = 0x11;
/// Sound channel 1 register 2
pub const NR12: u16 = 0x12;
/// Sound channel 1 register 3
pub const NR13: u16 = 0x13;
/// Sound channel 1 register 4
pub const NR14: u16 = 0x14;
/// Sound channel 2 register 1
pub const NR21: u16 = 0x16;
/// Sound channel 2 register 2
pub const NR22: u16 = 0x17;
/// Sound channel 2 register 3
pub const NR23: u16 = 0x18;
/// Sound channel 2 register 4
pub const NR24: u16 = 0x19;
/// Sound channel 1 register 0
pub const NR30: u16 = 0x1a;
/// Sound channel 1 register 1
pub const NR31: u16 = 0x1b;
/// Sound channel 1 register 2
pub const NR32: u16 = 0x1c;
/// Sound channel 1 register 3
pub const NR33: u16 = 0x1d;
/// Sound channel 1 register 4
pub const NR34: u16 = 0x1e;
/// Sound channel 4 register 1
pub const NR41: u16 = 0x20;
/// Sound channel 4 register 2
pub const NR42: u16 = 0x21;
/// Sound channel 4 register 3
pub const NR43: u16 = 0x22;
/// Sound channel 4 register 4
pub const NR44: u16 = 0x23;
/// Sound control: output volume
pub const NR50: u16 = 0x24;
/// Sound control: select output terminal
pub const NR51: u16 = 0x25;
/// Sound control: set global enable and get sound status
pub const NR52: u16 = 0x26;
/// Sound channel 3 sample RAM start
pub const NR3_RAM_START: u16 = 0x30;
/// Sound channel 3 sample RAM end
pub const NR3_RAM_END: u16 = 0x3f;


/// LCD Control
pub const LCDC: u16 = 0x40;
/// LCDC Status + IT selection
pub const LCD_STAT: u16 = 0x41;
/// LCDC Background Y position
pub const LCD_SCY: u16 = 0x42;
/// LCDC Background X position
pub const LCD_SCX: u16 = 0x43;
/// Currently displayed line
pub const LCD_LY: u16 = 0x44;
/// Currently line compare
pub const LCD_LYC: u16 = 0x45;
/// DMA transfer from ROM/RAM to OAM
pub const DMA: u16 = 0x46;
/// Background palette
pub const LCD_BGP: u16 = 0x47;
/// Sprite palette 0
pub const LCD_OBP0: u16 = 0x48;
/// Sprite palette 1
pub const LCD_OBP1: u16 = 0x49;
/// Window Y position
pub const LCD_WY: u16 = 0x4a;
/// Window X position + 7
pub const LCD_WX: u16 = 0x4b;
