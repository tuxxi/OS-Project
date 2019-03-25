use crate::os::allocator::Allocator;
use crate::os::dispatcher::Dispatcher;
use crate::os::structures::{MemoryRange, ProcessControlBlock, PID};
use crate::records::{OSParams, ProcessData};

use itertools::sorted;
use std::collections::{HashMap, VecDeque};

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
    pub fifo_schedule: VecDeque<PID>,
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
        let num_procs = processes.len();
        Self {
            input_params: params,
            input_procs: processes,
            input_queue: VecDeque::with_capacity(num_procs),

            running_processes: HashMap::with_capacity(num_procs),
            fifo_schedule: VecDeque::with_capacity(num_procs),
            master_clock: 0,
            current_pid: 0,
            memory_map: HashMap::with_capacity(mem_cap),

            ready_queue: VecDeque::with_capacity(num_procs),
            blocked_queue: VecDeque::with_capacity(num_procs),
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
        let mut dispatcher = Dispatcher::new();
        loop {
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
            // allocate processes, if we allocated, this uses up a clock cycle so we
            if Allocator::allocate(self) {
                continue;
            }
            // dispatch IO and CPU resources to running processes
            dispatcher.dispatch(self);

            // check if we should print info for this cycle
            if self.master_clock % every_n == 0 {
                self.print_info();
            }
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
