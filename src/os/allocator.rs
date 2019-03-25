use self::AllocResult::*;
use crate::os::os::OS;
use crate::os::structures::{MemoryRange, ProcessControlBlock, State};
use crate::records::ProcessData;

use std::collections::HashSet;

// result of allocation attempt
enum AllocResult {
    NoSpace,
    TooBig,
    Allocated(MemoryRange),
}

pub struct Allocator;
impl Allocator {
    /** Allocates processes when there is room */
    pub fn allocate(os: &mut OS) -> bool {
        let mut cycled = false; // did the OS use up a clock cycle by allocating, or was input queue empty?
        loop {
            // allocate until the queue is empty
            if let Some(info) = os.input_queue.pop_front() {
                // try to allocate, and check result of allocation
                match Self::alloc_one(os, &info) {
                    // everything was ok, process allocated
                    Allocated(_) => {
                        println!(
                            "Allocated {} at clock time {}",
                            info.process_name, os.master_clock
                        );
                        cycled = true;
                    }

                    // process too big. don't re add to queue
                    TooBig => {
                        println!(
                            "Flushed {} from input queue: Not enough memory!",
                            info.process_name
                        );
                        break;
                    }

                    // no space this time, try to add next clock cycle.
                    NoSpace => {
                        os.input_queue.push_back(info);
                        break;
                    }
                }
            } else {
                break;
            }
        }
        cycled
    }
    /** Allocates a single process
    @returns:
        false if there is no room for the process in memory
        true if allocation succeeded. */
    fn alloc_one(os: &mut OS, info: &ProcessData) -> AllocResult {
        let memory_range = match Self::check_memory(os, info) {
            Allocated(T) => T,
            NoSpace => return NoSpace,
            TooBig => return TooBig,
        };

        let pid = (os.running_processes.len() + 1) as i32;

        // add process to memory map
        os.memory_map.insert(pid, memory_range.clone());
        // add pid to FIFO scheduling queue
        os.fifo_schedule.push_back(pid);
        // add
        os.running_processes.insert(
            pid,
            ProcessControlBlock {
                info: info.clone(), //grr borrow checker :^(
                pid,
                clk: os.master_clock,
                state: State::Allocating,
                total_cpu: 0,
                total_ios: 0,
                start_time: 0,
                end_time: 0,
                memory_map: memory_range.clone(),
            },
        );
        Allocated(memory_range)
    }

    /** Checks if memory is available for a given process, and returns the available memory range if it is */
    fn check_memory(os: &mut OS, info: &ProcessData) -> AllocResult {
        let proc_mem_size = info.process_memsize / (os.input_params.mem_fix_block_size / 1000);
        let os_mem_max = os.input_params.mem_fix_total_blocks;

        // check if this process will even fit in our total memory
        if proc_mem_size > os_mem_max {
            return TooBig;
        }
        // if nothing is already in memory, we get the first available bytes
        if os.memory_map.is_empty() {
            return Allocated(MemoryRange(1, proc_mem_size));
        }

        // otherwise, check each process's memory range to see if we can fit somewhere

        // fill up vec with all currently allocated memory blocks
        let mut allocated_blocks = HashSet::with_capacity(os_mem_max as usize);
        for range in os.memory_map.values() {
            for i in range.0..=range.1 {
                allocated_blocks.insert(i);
            }
        }

        // check each OS memory block
        let mut prev = 0;
        for block in 0..=os_mem_max {
            if block - prev >= proc_mem_size {
                // we found room for our new process!
                return Allocated(MemoryRange(block - proc_mem_size + 1, block));
            }
            if allocated_blocks.contains(&block) {
                prev = block;
            }
        }
        // didn't find any room
        NoSpace
    }
}
