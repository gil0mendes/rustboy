use cartridge::Cartridge;
use cpu::Cpu;
use gpu::Gpu;
use io::Interconnect;

pub struct Machine {
    cpu: Cpu
}

/// Manage the GameBoy as a whole.
impl Machine {
    pub fn new(cartridge: Cartridge) -> Self {
        let interconnect = Interconnect::new(cartridge, Gpu::new());
        let cpu = Cpu::new(interconnect);

        Machine {
            cpu
        }
    }
}