use crate::types::GbKey;

/// P1 register
///
/// Bits are inverted, 0 means pressed
bitflags!(
    struct P1: u8 {
        const P10                   = 1 << 0;   // P10: right or A
        const P11                   = 1 << 1;   // P11: left or B
        const P12                   = 1 << 2;   // P12: up or select
        const P13                   = 1 << 3;   // P13: down or start
        const SELECT_DIRECTIONAL    = 1 << 4;   // P14: select dpad
        const SELECT_BUTTON         = 1 << 5;   // P15: select buttons

        /// When only select bits are writable
        const WRITABLE = P1::SELECT_DIRECTIONAL.bits | P1::SELECT_BUTTON.bits;
    }
);

impl P1 {
    /// Get directional key state
    fn directional(key: &GbKey) -> P1 {
        match *key {
            GbKey::Right => P1::P10,
            GbKey::Left => P1::P11,
            GbKey::Up => P1::P12,
            GbKey::Down => P1::P13,
            _ => P1::empty(),
        }
    }

    /// Get buttons state
    fn buttons(key: &GbKey) -> P1 {
        match *key {
            GbKey::A => P1::P10,
            GbKey::B => P1::P11,
            GbKey::Select => P1::P12,
            GbKey::Start => P1::P13,
            _ => P1::empty(),
        }
    }
}

/// Gameboy joypad.
///
/// Gameboy has a dpad and four buttons: A, B, Select and Start.
pub struct Joypad {
    directional: P1,
    button: P1,
    register: P1,
}

impl Joypad {
    /// Creates a new Joypad instance
    pub fn new() -> Self {
        Self {
            directional: P1::empty(),
            button: P1::empty(),
            register: P1::empty(),
        }
    }

    /// Get register state.
    ///
    /// Invert bits, so 0 means "set".
    pub fn get_register(&self) -> u8 {
        !self.register.bits
    }

    /// Set the joypad register.
    ///
    /// Invert bits before converting into a P1.
    pub fn set_register(&mut self, value: u8) {
        self.register = P1::from_bits_truncate(!value);
        self.update_register();
    }

    /// Updates the register state based on select bits P14-P15 and the
    /// pressed buttons
    pub fn update_register(&mut self) {
        self.register &= P1::WRITABLE;

        if self.register.contains(P1::SELECT_DIRECTIONAL) {
            self.register.insert(self.directional);
        }

        if self.register.contains(P1::SELECT_BUTTON) {
            self.register.insert(self.button);
        }
    }
}