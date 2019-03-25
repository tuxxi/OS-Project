use crate::os::structures::{MemoryRange, ProcessControlBlock, State, PID};
use crate::records::{OSParams, ProcessData};
use std::collections::{HashMap, VecDeque, HashSet};

//longest clock acceptable
const CLOCK_LIMIT: i32 = 3000;

// version info
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

// result of allocation attempt
enum AllocResult {
    NoSpace,
    TooBig,
    Allocated(MemoryRange)
}

pub struct OS {
    // input data
    input_params: OSParams,
    input_procs: Vec<ProcessData>,
    input_queue: VecDeque<ProcessData>,

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
            self.allocate();
            // dispatch IO and CPU resources to running processes
            self.dispatch();

            // check if we should print info for this cycle
            if self.master_clock % every_n == 0 {
                self.print_info();
            }

            // increment the master clock
            self.master_clock += 1;
        }
    }

    fn dispatch(&mut self) {

    }
    /** Allocates processes when there is room */
    fn allocate(&mut self) {
        loop {
            // allocate until the queue is empty
            if let Some(info) = self.input_queue.pop_front() {
                // try to allocate, and check result of allocation
                match self.alloc_one(&info) {
                    // everything was ok, process allocated
                    AllocResult::Allocated(_) => {
                        println!(
                            "Allocated {} at clock time {}",
                            info.process_name,
                            self.master_clock)
                    },

                    // process too big. don't re add to queue
                    AllocResult::TooBig => {
                        println!(
                            "Flushed {} from input queue: Not enough memory!",
                            info.process_name);
                        break;
                    },

                    // no space this time, try to add next clock cycle.
                    AllocResult::NoSpace => {
                        self.input_queue.push_back(info);
                        break;
                    },
                }
            } else { break }
        }
    }
    /** Allocates a single process
    @returns:
        false if there is no room for the process in memory
        true if allocation succeeded. */
    fn alloc_one(&mut self, info: &ProcessData) -> AllocResult {
        let memory_range = match self.check_memory(info) {
            AllocResult::Allocated(T) => T,
            AllocResult::NoSpace => return AllocResult::NoSpace,
            AllocResult::TooBig => return AllocResult::TooBig,
        };

        let pid = (self.running_processes.len() + 1) as i32;

        self.memory_map.insert(pid, memory_range.clone());
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
                memory_map: memory_range.clone(),
            },
        );
        AllocResult::Allocated(memory_range)
    }

    /** Checks if memory is available for a given process, and returns the available memory range if it is */
    fn check_memory(&self, info: &ProcessData) -> AllocResult {

        let proc_mem_size = info.process_memsize / (self.input_params.mem_fix_block_size / 1000);
        let os_mem_max = self.input_params.mem_fix_total_blocks;

        // check if this process will even fit in our total memory
        if proc_mem_size > os_mem_max {
            return AllocResult::TooBig
        }
        // if nothing is already in memory, we get the first available bytes
        if self.memory_map.is_empty() {
            return AllocResult::Allocated(MemoryRange(1, proc_mem_size))
        }

        // otherwise, check each process's memory range to see if we can fit somewhere

        // fill up vec with all currently allocated memory blocks
        let mut allocated_blocks = HashSet::with_capacity(os_mem_max as usize);
        for range in self.memory_map.values() {
            for i in range.0..=range.1 {
                allocated_blocks.insert(i);
            }
        }

        // check each OS memory block
        let mut prev = 0;
        for block in 0..=os_mem_max {
            if block - prev >= proc_mem_size {  // we found room for our new process!
                 return AllocResult::Allocated(MemoryRange(block - proc_mem_size + 1, block))
            }
            if allocated_blocks.contains(&block) {
                prev = block;
            }
        }
        // didn't find any room
        AllocResult::NoSpace
    }

    /** Print running process info */
    fn print_info(&self) {
        println!(
            "==================================={}===================================",
            self.master_clock,
        );
        for process in self.running_processes.values() {
            println!("{}", process);
        }
        println!(
            "==================================={}===================================",
            self.master_clock,
        );
    }
}
