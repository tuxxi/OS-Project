use super::structures::{Event, ProcessControlBlock, State};
use crate::records::{OSParams, ProcessData};
use std::collections::VecDeque;

//longest clock acceptable
const CLOCK_LIMIT: u32 = 5000;

pub struct OS {
    params: OSParams,
    // running info
    processes: Vec<ProcessControlBlock>,
    state: OSState,
    master_clock: u32,
    current_pid: i32,
    // queues
    ready_queue: VecDeque<i32>,
    blocked_queue: VecDeque<i32>,
}
#[derive(Debug)]
enum OSState {
    Stopped,
    Ready,
    Running,
    Completed,
}
impl OS {
    pub fn new(params: OSParams, processes: Vec<ProcessData>) -> OS {
        OS {
            params,
            processes: processes
                .iter()
                .enumerate()
                .map(|(idx, proc)| ProcessControlBlock {
                    info: proc.clone(),
                    id: idx as i32, // pid is just the index of process in our vector
                    clk: 0,
                    state: State::Allocating,
                    total_cpu: 0,
                    total_ios: 0,
                    start_time: 0,
                    end_time: 0,
                })
                .collect(),
            state: OSState::Ready,
            master_clock: 0,
            current_pid: 0,
            ready_queue: VecDeque::new(),
            blocked_queue: VecDeque::new(),
        }
    }
    pub fn start(&mut self) {
        self.state = OSState::Running;
        println!("Started OS... OS is now {:?}", self.state);
        self.loop_clock();
    }
    fn loop_clock(&mut self) {
        let every_n: u32 = self.params.every_n_units as u32;
        loop {
            // check if we should print
            if self.master_clock % every_n == 0 {
                self.print_info();
            }
            self.master_clock += 1;

            //check if we exceeded the clock limit
            if self.master_clock > CLOCK_LIMIT {
                eprintln!("Error! Likely runaway OS: clock exceeded {} cycles!", CLOCK_LIMIT);
                break;
            }
        }
    }
    //TODO
    /** Allocate a process to the ready queue */
    fn allocate(&mut self) {
        //println!("Allocating {} at time {} (PID #{})")
    }
    //TODO
    /** Print running process info */
    fn print_info(&self) {
        println!("System Clock: {}", self.master_clock);
        println!("Process Info:");
    }
}
