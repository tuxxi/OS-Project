use crate::os::os::OS;
use crate::os::structures::{ProcessControlBlock, State, PID};
use crate::records::{IODeviceType, RunInfo};
use itertools::sorted;

pub struct Dispatcher {
    cycles_to_go: i32,
    current_process: Option<PID>,
    current_info: Option<RunInfo>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            cycles_to_go: 0,
            current_process: None,
            current_info: None,
        }
    }
    pub fn dispatch(&mut self, os: &mut OS) {
        // add processes to ready and blocked queues
        for proc in os.running_processes.values() {
            if proc.state == State::Ready && !os.ready_queue.contains(&proc.pid) {
                os.ready_queue.push_back(proc.pid);
            }
            if proc.state == State::Blocked && !os.blocked_queue.contains(&proc.pid) {
                os.blocked_queue.push_back(proc.pid);
            }
        }

        // is the dispatcher currently executing a process right now?
        if let Some(pid) = self.current_process {
            self.exec(os, pid);
        }
        // No process is being executed
        else {
            // pop from ready queue
            if let Some(pid) = os.ready_queue.pop_front() {
                self.exec(os, pid);
                return;
            }
            // pop from blocked queue
            if let Some(pid) = os.blocked_queue.pop_front() {
                self.exec(os, pid);
                return;
            }

            //not ready or blocked, so we just execute in FIFO order
            let next_pid = Self::get_next_pid_FIFO(os);
            self.exec(os, next_pid);
        }
    }

    /** Execute CPU or IO units */
    fn exec(&mut self, os: &mut OS, pid: PID) {
        // set currently executing PID
        os.current_pid = pid;
        self.current_process = Some(pid);

        let proc = os.running_processes.get_mut(&pid).unwrap();
        let clock = os.master_clock;
        // set start time if we haven't already started the process
        if proc.start_time == 0 {
            proc.start_time = clock;
            println!("Started {} at clock time {}", proc.info.process_name, clock);
        }

        proc.state = State::Executing;

        let info = &mut proc.info.run_info;

        // check if we have info to use
        if let Some(info) = self.current_info.clone() {
            self.update_cpu_ios(proc, &info);

            // dirty, ugly hack because I can't figure out how the borrow checker works.
            if info.IO_device_type != (IODeviceType::Unknown) {
                os.blocked_queue.push_back(proc.pid);
            }

        } else {
            // need new run info, pop from runinfo vec.
            if let Some(info) = &info.pop() {
                // update cycles to go
                self.current_info = Some(info.clone());
                self.cycles_to_go = info.CPU_units;
                self.update_cpu_ios(proc, info);
                // dirty, ugly hack because I can't figure out how the borrow checker works.
                if info.IO_device_type != (IODeviceType::Unknown) {
                    os.blocked_queue.push_back(proc.pid);
                }
            }
        }
    }
    fn update_cpu_ios(&mut self, proc: &mut ProcessControlBlock, info: &RunInfo) {
        // if IO dev is not unknown, IO must happen in this record
        if info.IO_device_type != (IODeviceType::Unknown) {
            proc.state = State::Blocked;
            proc.total_ios += info.IO_units;
        }
        // update process total CPU time
        if self.cycles_to_go > 0 {
            proc.total_cpu += 1;
            self.cycles_to_go -= 1;
        } else {
            self.current_process = None;
            proc.state = State::Ready;
        }
    }
    /** get next PID required to execute */
    fn get_next_pid_FIFO(os: &mut OS) -> PID {
        let mut fifo = &mut os.fifo_schedule;
        match fifo.pop_front() {
            Some(T) => {
                // add PID back to queue
                fifo.push_back(T);
                T
            }
            None => 1,
        }
    }
}
