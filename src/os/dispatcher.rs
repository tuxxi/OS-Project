use crate::os::os::OS;
use crate::os::structures::{ProcessControlBlock, State, PID};
use crate::records::{IODeviceType, RunInfo};
use std::collections::HashMap;

pub struct Dispatcher {
    cpus_to_go: HashMap<PID, i32>,
    ios_to_go: HashMap<PID, i32>,
    current_process: Option<PID>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            cpus_to_go: HashMap::new(),
            ios_to_go: HashMap::new(),
            current_process: None,
        }
    }
    pub fn dispatch(&mut self, os: &mut OS) {
        // is the dispatcher currently executing a process right now?
        if let Some(pid) = self.current_process {
            self.exec(os, pid);
        } else {
            Self::get_next_pid_FIFO(os).map(|next_pid| {
                // set currently executing PID
                os.current_pid = next_pid;
                self.current_process = Some(next_pid);

                self.exec(os, next_pid);
            });
        }
        // update IOs for all blocked processes
        self.update_ios(os);
    }

    /** Execute a process */
    fn exec(&mut self, os: &mut OS, pid: PID) {
        if let Some(proc) = os.running_processes.get_mut(&pid) {
            // start the process, if we haven't already started
            if proc.start_time == 0 {
                let clock = os.master_clock;
                proc.start_time = clock;
                println!(
                    "Started {} (PID # {}) at clock time {}",
                    proc.info.process_name, pid, clock
                );
            }

            // check if the dispatcher was previously executing a process, and use that CPU info
            match self.cpus_to_go.get(&pid) {
                Some(_) => {
                    if !os.blocked_queue.contains(&pid) {
                        self.update_cpu(proc);
                    }
                }
                None => {
                    let info_vec = &mut proc.info.run_info;
                    // need new run info, pop from runinfo vec.
                    if let Some(info) = &info_vec.pop() {
                        // update CPU cycles to go
                        self.cpus_to_go.insert(pid, info.CPU_units);

                        // update IO cycles to go
                        if info.IO_units > 0 {
                            println!(
                                "Started IO for process {} (PID # {}) at clock time {}",
                                proc.info.process_name, pid, os.master_clock
                            );
                            self.ios_to_go.insert(pid, info.IO_units);
                        }
                        self.update_cpu(proc);
                    } else {
                        // info is empty, process must have been completed!
                        proc.state = State::Done;
                        println!(
                            "Finished process {} (PID # {}) at clock time {}",
                            proc.info.process_name, pid, os.master_clock
                        );
                        self.current_process = None;
                        os.remove_process(pid);
                    }
                }
            }
        }
    }

    /** update IO cycles completed */
    fn update_ios(&mut self, os: &mut OS) {
        // create list of PIDS we must remove
        let mut to_remove: Vec<PID> = Vec::new();
        for (pid, togo) in self.ios_to_go.iter_mut() {
            let proc = os.running_processes.get_mut(pid).unwrap();
            proc.state = State::Blocked;
            if !os.blocked_queue.contains(&proc.pid) {
                os.blocked_queue.push_back(proc.pid);
            }

            if *togo > 0 {
                proc.total_ios += 1;
                *togo -= 1;
            } else {
                println!(
                    "IO completed for process {} (PID {}) at clock time {}",
                    proc.info.process_name, proc.pid, os.master_clock
                );
                os.blocked_queue.pop_front();
                to_remove.push(proc.pid);
            }
        }

        // removed mrked IOs
        for pid in to_remove {
            self.ios_to_go.remove(&pid);
        }
    }

    /** Update CPU cycles completed */
    fn update_cpu(&mut self, proc: &mut ProcessControlBlock) {
        let togo = self.cpus_to_go.get_mut(&proc.pid).unwrap();
        // update total CPU time for the currently running process
        if *togo > 0 {
            // info block has more cycles to go
            proc.state = State::Executing;
            proc.total_cpu += 1;
            *togo -= 1;
        } else {
            // done executing CPU for this info block
            self.current_process = None;
            self.cpus_to_go.remove(&proc.pid);
            proc.state = State::Ready;
        }
    }

    /** get next PID required to execute
    @returns
    Some(PID) for the next PID in the ready queue
    None if nothing is in the ready queue*/
    fn get_next_pid_FIFO(os: &mut OS) -> Option<PID> {
        let fifo = &mut os.ready_queue;
        fifo.pop_front().and_then(|T| {
            fifo.push_back(T);
            Some(T)
        })
    }
}
