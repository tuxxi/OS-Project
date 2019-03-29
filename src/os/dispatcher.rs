use crate::os::os::OS;
use crate::os::structures::{ProcessControlBlock, State, PID};
use crate::records::IODeviceType;
use std::collections::{HashMap, VecDeque};

/** A dispatcher event -- IO completion, timeout, start or finish
    .time: time at event creation
    .pid: pid of process that created event
*/
struct Event {
    pub _type: EventType,
    pub time: i32,
    pub pid: PID,
}
enum EventType {
    IO,       // IO completion
    Timeout,  // CPU quantum completion
    Started,  // process started
    Finished, // process finished
}

pub struct Dispatcher {
    cpus_to_go: HashMap<PID, i32>,
    ios_to_go: HashMap<PID, (IODeviceType, i32)>,
    current_process: Option<PID>,
    event_queue: VecDeque<Event>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            cpus_to_go: HashMap::new(),
            ios_to_go: HashMap::new(),
            current_process: None,
            event_queue: VecDeque::new(),
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

        // process event queue
        self.process_events(os);
    }

    /** Execute a process */
    fn exec(&mut self, os: &mut OS, pid: PID) {
        if let Some(proc) = os.running_processes.get_mut(&pid) {
            let clock = os.master_clock;

            // start the process, if we haven't already started
            if proc.start_time == 0 {
                self.event_queue.push_back(Event {
                    _type: EventType::Started,
                    time: clock,
                    pid: clock,
                });
            }

            // check if the dispatcher was previously executing a process, and use that CPU info
            match self.cpus_to_go.get(&pid) {
                Some(_) => {
                    if !os.blocked_queue.contains(&pid) {
                        self.update_cpu(proc, clock);
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
                            self.ios_to_go
                                .insert(pid, (info.IO_device_type, info.IO_units));
                        }
                        self.update_cpu(proc, clock);
                    } else {
                        // info is empty, process must have been completed!
                        self.event_queue.push_back(Event {
                            _type: EventType::Finished,
                            time: os.master_clock,
                            pid,
                        });
                    }
                }
            }
        }
    }

    /** update IO cycles completed */
    fn update_ios(&mut self, os: &mut OS) {
        for (pid, togo) in self.ios_to_go.iter_mut() {
            let proc = os.running_processes.get_mut(pid).unwrap();
            proc.state = State::Blocked;
            if !os.blocked_queue.contains(&proc.pid) {
                os.blocked_queue.push_back(proc.pid);
            }

            if togo.1 > 0 {
                proc.total_ios += 1;
                togo.1 -= 1;
            } else {
                self.event_queue.push_back(Event {
                    _type: EventType::IO,
                    time: os.master_clock,
                    pid: *pid,
                });
            }
        }
    }

    /** Update CPU cycles completed */
    fn update_cpu(&mut self, proc: &mut ProcessControlBlock, clock: i32) {
        let togo = self.cpus_to_go.get_mut(&proc.pid).unwrap();
        // update total CPU time for the currently running process
        if *togo > 0 {
            // info block has more cycles to go
            proc.state = State::Executing;
            proc.total_cpu += 1;
            *togo -= 1;
        } else {
            // done executing CPU for this info block
            self.event_queue.push_back(Event {
                _type: EventType::Timeout,
                time: clock,
                pid: proc.pid,
            })
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

    fn process_events(&mut self, os: &mut OS) {
        // process all events in the queue with drain(..)
        for event in self.event_queue.drain(..) {
            if let Some(proc) = os.running_processes.get_mut(&event.pid) {
                match event._type {
                    EventType::IO => {
                        println!(
                            "IO for process {} (PID {}) completed at clock time {}",
                            proc.info.process_name, event.pid, event.time
                        );
                        self.ios_to_go.remove(&event.pid);
                        os.blocked_queue.pop_front();
                    }
                    EventType::Timeout => {
                        println!(
                            "Process {} (PID # {}) timed out at clock time {}",
                            proc.info.process_name, event.pid, event.time
                        );
                        self.current_process = None;
                        self.cpus_to_go.remove(&event.pid);
                        proc.state = State::Ready;
                    }
                    EventType::Finished => {
                        println!(
                            "Process {} (PID # {}) completed at clock time {}",
                            proc.info.process_name, event.pid, event.time
                        );
                        proc.state = State::Done;
                        self.current_process = None;
                        os.remove_process(event.pid);
                    }
                    EventType::Started => {
                        println!(
                            "Process {} (PID # {}) started at clock time {}",
                            proc.info.process_name, event.pid, event.time
                        );
                        proc.start_time = event.time;
                    }
                }
            }
        }
    }
}
