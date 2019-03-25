use crate::os::structures::{MemoryRange, ProcessControlBlock, PID};
use crate::records::{OSParams, ProcessData};
use crate::os::allocator::Allocator;
use crate::os::dispatcher::Dispatcher;

use std::collections::{HashMap, VecDeque};
use itertools::sorted;

//longest clock acceptable
const CLOCK_LIMIT: i32 = 3000;

// version info
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");


pub struct OS {
    // input data
    pub input_params: OSParams,
    pub input_procs: Vec<ProcessData>,
    pub input_queue: VecDeque<ProcessData>,

    // running info
    pub running_processes: HashMap<PID, ProcessControlBlock>,
    pub master_clock: i32,
    pub current_pid: PID,
    pub memory_map: HashMap<PID, MemoryRange>,

    // queues
    pub ready_queue: VecDeque<PID>,
    pub blocked_queue: VecDeque<PID>,
}

impl OS {
    pub fn new(params: OSParams, processes: Vec<ProcessData>) -> Self {
        let mem_cap = params.mem_fix_total_blocks as usize;
        OS {
            input_params: params,
            input_procs: processes,
            input_queue: VecDeque::new(),

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
        for proc in &self.input_procs {
            self.input_queue.push_back(proc.clone())
        }

        self.loop_clock();
    }

    /** Starts the OS clock*/
    fn loop_clock(&mut self) {
        let every_n = self.input_params.every_n_units;
        loop {
            //check if we exceeded the clock limit
            if self.master_clock > CLOCK_LIMIT {
                eprintln!(
                    "Error! Likely runaway OS: clock exceeded {} cycles!",
                    CLOCK_LIMIT
                );
                break;
            }
            // allocate processes
            Allocator::allocate( self);
            // dispatch IO and CPU resources to running processes
            Dispatcher::dispatch(self);

            // check if we should print info for this cycle
            if self.master_clock % every_n == 0 {
                self.print_info();
            }

            // increment the master clock
            self.master_clock += 1;
        }
    }

    /** Print running process info */
    fn print_info(&self) {
        println!(
            "==================================={}===================================",
            self.master_clock,
        );
        for process in sorted(self.running_processes.values()) {
            println!("{}", process);
        }
        println!(
            "==================================={}===================================",
            self.master_clock,
        );
    }
}
