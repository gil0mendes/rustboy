use self::blip_buf::BlipBuf;

extern crate blip_buf;

pub struct SquareChannel {
    enabled: bool,
    blip: BlipBuf,
    has_sweep: bool,
}

impl SquareChannel {
    /// create a new SquareChannel instance
    pub fn new(blip: BlipBuf, with_sweep: bool) -> SquareChannel {
        SquareChannel {
            enabled: false,
            blip: blip,
            has_sweep: with_sweep
        }
    }

    /// check if the channel are enabled
    pub fn on(&self) -> bool {
        self.enabled
    }

    /// write a byte on sound channel
    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            _ => { panic!("TODO SquareChannel write address {:#x}", address) }
        }
    }
}
