use super::Process;

const MAX_NUMBER_OF_PROCESSES: usize = 4;

pub struct Kernel {
    processes: [Option<Process>; MAX_NUMBER_OF_PROCESSES],
}

impl Kernel {
    pub fn new() -> Self {
        Kernel {
            processes: [None, None, None, None],
        }
    }

    pub fn load_process(&mut self, process: Process) -> Result<(), ()> {
        let index: usize = match self.processes.iter().position(|p| p.is_none()) {
            Some(n) => n,
            None => return Err(()),
        };
        self.processes[index].replace(process);
        Ok(())
    }

    pub fn run_scheduling(&mut self) -> ! {
        let len = self.processes.num();
        let mut next = 0;
        loop {
            let next_process = self.processes[next].as_mut();
            if next_process.is_none() {
                unimplemented!();
            }
            next_process.map(|p| { p.exec(); });
            next = (next + 1) % len;
        }
    }
}

trait Num {
    fn num(&self) -> usize;
}

impl Num for [Option<Process>] {
    fn num(&self) -> usize {
        match self.iter().position(|p| p.is_none()) {
            Some(n) => n,
            None => MAX_NUMBER_OF_PROCESSES,
        }
    }
}