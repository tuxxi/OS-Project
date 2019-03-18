use super::structures::{ProcessControlBlock, Event, State};
use crate::records::{ProcessData, OSParams};

pub struct OS {
    params: OSParams,
    processes: Vec<ProcessControlBlock>,
    state: OSState,
    master_clock: u32,


}
enum OSState {
    Stopped, Ready, Running, Completed
}
impl OS {
    pub fn new(params: OSParams, processes: Vec<ProcessData>) -> OS {
        let mut procs: Vec<ProcessControlBlock> = vec![];
        for (idx, proc) in processes.iter().enumerate() {
            procs.push(ProcessControlBlock {
                info: proc.clone(),
                id: idx as i32 + 1,     // pid is just index of when we added
                clk: 0,
                state: State::Allocating,
                total_cpu: 0,
                total_ios: 0,
                start_time: 0,
                end_time: 0
            })
        }
        OS {
            params,
            processes: procs,
            state: OSState::Ready,
            master_clock: 0
        }
    }
    pub fn start(&mut self) {
        self.state = OSState::Running;
        println!("Started OS...");
    }
}