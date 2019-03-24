use crate::os::structures::{MemoryRange, ProcessControlBlock, State, PID};
use crate::records::{OSParams, ProcessData};
use std::collections::{HashMap, VecDeque};

//longest clock acceptable
const CLOCK_LIMIT: i32 = 5000;

// version info
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

pub struct OS {
    // input data
    input_params: OSParams,
    input_procs: Vec<ProcessData>,

    // running info
    running_processes: HashMap<PID, ProcessControlBlock>,
    master_clock: i32,
    current_pid: PID,
    memory_map: HashMap<PID, MemoryRange>,

    // queues
    ready_queue: VecDeque<PID>,
    blocked_queue: VecDeque<PID>,
}

impl OS {
    pub fn new(params: OSParams, processes: Vec<ProcessData>) -> OS {
        let mem_cap = params.mem_fix_total_blocks as usize;
        OS {
            input_params: params,
            input_procs: processes,
            running_processes: HashMap::new(),
            master_clock: 0,
            current_pid: 0,
            memory_map: HashMap::with_capacity(mem_cap),
            ready_queue: VecDeque::new(),
            blocked_queue: VecDeque::new(),
        }
    }
    /** Start the OS Simulation */
    pub fn start(&mut self) {
        println!(
            "Started OS Simulation version {}.",
            VERSION.unwrap_or("(unknown)")
        );
        //TODO: remove me
        self.allocate();
        self.loop_clock();
    }

    /** Starts the OS clock*/
    fn loop_clock(&mut self) {
        let every_n = self.input_params.every_n_units;
        loop {
            // TODO: allocate, etc etc

            // check if we should print info for this cycle
            if self.master_clock % every_n == 0 {
                self.print_info();
            }

            // increment the master clock
            self.master_clock += 1;

            //check if we exceeded the clock limit
            if self.master_clock > CLOCK_LIMIT {
                eprintln!(
                    "Error! Likely runaway OS: clock exceeded {} cycles!",
                    CLOCK_LIMIT
                );
                break;
            }
        }
    }

    /** Checks if memory is available for a given process */
    fn check_memory(&mut self, pcb: &ProcessControlBlock) {}

    /** Allocates processes to memory */
    fn allocate(&mut self) {
        // TODO: check memory and allocate different process other than just first one.
        let info = &self.input_procs[0];
        let pid = (self.running_processes.len() + 1) as i32;
        let memory_max = info.process_memsize / (self.input_params.mem_fix_block_size / 1000);
        self.running_processes.insert(
            pid,
            ProcessControlBlock {
                info: info.clone(), //grr borrow checker :^(
                pid,
                clk: self.master_clock,
                state: State::Allocating,
                total_cpu: 0,
                total_ios: 0,
                start_time: self.master_clock,
                end_time: -1,
                memory_map: MemoryRange(0, memory_max),
            },
        );
    }

    /** Print running process info */
    fn print_info(&self) {
        println!(
            "==================================={}===================================",
            self.master_clock
        );
        for (_, process) in self.running_processes.iter() {
            println!("{}", process);
        }
    }
}
