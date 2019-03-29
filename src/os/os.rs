use crate::os::allocator::Allocator;
use crate::os::dispatcher::Dispatcher;
use crate::os::structures::{MemoryRange, ProcessControlBlock, PID};
use crate::records::{OSParams, ProcessData};

use itertools::sorted;
use std::collections::{HashMap, VecDeque};

// version info
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

pub struct OS {
    // input data
    pub input_params: OSParams,
    pub input_procs: Vec<ProcessData>,
    pub input_queue: VecDeque<ProcessData>,
    pub input_size: i32,
    clock_limit: i32,

    // running info
    pub running_processes: HashMap<PID, ProcessControlBlock>,
    pub master_clock: i32,
    pub current_pid: PID,
    pub memory_map: HashMap<PID, MemoryRange>,

    // queues
    pub blocked_queue: VecDeque<PID>,
    pub ready_queue: VecDeque<PID>,
}

impl OS {
    pub fn new(params: OSParams, processes: Vec<ProcessData>, clock_limit: i32) -> Self {
        let mem_cap = params.mem_fix_total_blocks as usize;
        let num_procs = processes.len();
        Self {
            input_params: params,
            input_procs: processes,
            input_queue: VecDeque::with_capacity(num_procs),
            input_size: num_procs as i32,
            clock_limit,

            running_processes: HashMap::with_capacity(num_procs),
            master_clock: 0,
            current_pid: 0,
            memory_map: HashMap::with_capacity(mem_cap),

            blocked_queue: VecDeque::with_capacity(num_procs),
            ready_queue: VecDeque::with_capacity(num_procs),
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

    /** Remove a process from the OS */
    pub fn remove_process(&mut self, pid: PID) {
        // remove from blocked queue
        for (idx, item) in self.blocked_queue.iter_mut().enumerate() {
            if *item == pid {
                self.blocked_queue.remove(idx);
                break;
            }
        }
        // remove from ready queue
        for (idx, item) in self.ready_queue.iter_mut().enumerate() {
            if *item == pid {
                self.ready_queue.remove(idx);
                break;
            }
        }
        // remove from memory map
        self.memory_map.remove(&pid);
        // remove from running processes table
        self.running_processes.remove(&pid);
    }

    /** Starts the OS clock*/
    fn loop_clock(&mut self) {
        let every_n = self.input_params.every_n_units;
        let mut dispatcher = Dispatcher::new();
        loop {
            // increment the master clock
            self.master_clock += 1;

            //check if we exceeded the clock limit
            if self.master_clock > self.clock_limit {
                eprintln!(
                    "Error! Likely runaway OS: clock exceeded {} cycles!",
                    self.clock_limit
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

            // check if simulation is finished
            if self.running_processes.is_empty() && self.input_queue.is_empty() {
                println!(
                    "OS simulation finished at clock time {}.",
                    self.master_clock
                );
                break;
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
