use cartridge::Cartridge;
use cpu::Cpu;
use gpu::Gpu;
use io::Interconnect;

pub struct Machine {
    cpu: Cpu,
    interconnect: Interconnect
}

/// Manage the GameBoy as a whole.
impl Machine {
    pub fn new(cartridge: Cartridge) -> Self {
        let interconnect = Interconnect::new(cartridge, Gpu::new());
        let cpu = Cpu::new();

        Machine {
            cpu,
            interconnect
        }
    }

    pub fn emulate(&mut self) {
        // Process the next CPU instruction
        let cycles = self.cpu.next_trick(&mut self.interconnect);

        // Do the interconnect cycle
        self.interconnect.do_cycle(cycles);
    }

    pub fn screen_buffer(&self) -> Vec<u8> {
        unimplemented!()
    }
}