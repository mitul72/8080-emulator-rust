use super::cpu::CPU;
pub struct SpaceInvaderMachine<'a> {
    cpu: &'a mut CPU,

    last_timer: std::time::Instant,
    next_interrupt: u8,
    which_interrupt: u8,

    cycles: u64,
    paused: bool,
}

impl<'a> SpaceInvaderMachine<'a> {
    pub fn new(cpu: &'a mut CPU) -> SpaceInvaderMachine<'a> {
        SpaceInvaderMachine {
            cpu,
            last_timer: std::time::Instant::now(),
            next_interrupt: 0,
            which_interrupt: 1,
            cycles: 0,
            paused: false,
        }
    }

    fn read_memory(&self, address: u16) -> u8 {
        self.cpu.state.memory[address as usize]
    }

    fn key_down(&mut self, key: u8) {
        self.cpu.state.in_port1 |= key;
    }
}
