//! Game Boy memory map. Memory ranges are inclusive.

/// ROM
pub const ROM: (u16, u16) = (0x0000, 0x7fff);
/// Video Character RAM
pub const CHAR_RAM: (u16, u16) = (0x8000, 0x97ff);
/// Video Tile Map 1
pub const V_TILE_MAP1: (u16, u16) = (0x9800, 0x9bff);
/// Video Tile Map 2
pub const V_TILE_MAP2: (u16, u16) = (0x9c00, 0x9fff);
/// RAM Bank N
pub const RAM_BANK: (u16, u16) = (0xa000, 0xbfff);
/// Internal RAM
pub const IRAM: (u16, u16) = (0xc000, 0xdfff);
/// Internal RAM echo
pub const IRAM_ECHO: (u16, u16) = (0xe000, 0xfdff);
/// Object Attribute Memory
pub const OAM: (u16, u16) = (0xfe00, 0xfe9f);
/// Empty but unusable for
pub const EMPTY_RAM: (u16, u16) = (0xfea0, 0xfeff);
/// IO ports
pub const IO: (u16, u16) = (0xff00, 0xff4b);
/// Register used to unmap the bootrom, Should not
/// be used by regular games
pub const UNMAP_BOOTROM: u16 = 0xff50;
/// Working RAM Bank Number
pub const WRAMBANK: u16 = 0xff70;
/// Zero page memory (High RAM)
pub const ZERO_PAGE: (u16, u16) = (0xff80, 0xfffe);
/// Interrupt Enable Register
pub const IEN: u16 = 0xffff;

/// Return `Some(offset)` if the given address is in the 
/// inclusive range `range`, Where `offset` is an u16 
/// equal to the offset of `address` within the `range`.
pub fn in_range(address: u16, range: (u16, u16)) -> Option<u16> {
    // destruct the range
    let (first, last) = range;

    // check if the address is in range
    if address >= first && address <= last {
        Some(address - first)
    } else {
        None
    }
}

/// Return the size of `range` in bytes
pub fn range_size(range: (u16, u16)) -> u16 {
    // destruct the range
    let (first, last) = range;

    // compute the range size and return it
    last - first + 1
}
