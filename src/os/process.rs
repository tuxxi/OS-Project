use crate::os::memory::MemoryRange;
use crate::records::ProcessData;
use std::cmp::Ordering;
use std::fmt;

pub type PID = i32;

/** The Process Control Block (PCB) for each process */
pub struct ProcessControlBlock {
    pub info: ProcessData, // process info,
    pub pid: PID,
    pub clk: i32, // current CPU clock
    pub state: ProcessState,
    pub total_cpu: i32, // total CPU cycles completed
    pub total_ios: i32, // total IO cycles completed
    pub start_time: i32,
    pub end_time: i32,
    pub memory_map: MemoryRange, // where in memory this process is located
}

impl Ord for ProcessControlBlock {
    fn cmp(&self, other: &ProcessControlBlock) -> Ordering {
        self.pid.cmp(&other.pid)
    }
}

impl PartialOrd for ProcessControlBlock {
    fn partial_cmp(&self, other: &ProcessControlBlock) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for ProcessControlBlock {}

impl PartialEq for ProcessControlBlock {
    fn eq(&self, other: &ProcessControlBlock) -> bool {
        self.pid == other.pid
    }
}

impl fmt::Display for ProcessControlBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pid = self.pid;
        let name = &self.info.process_name;
        let state = &self.state;
        let blocks = &self.memory_map;
        write!(f, "{}\t{}\t{}\t\t\t{:?}", pid, name, blocks, state)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ProcessState {
    Allocating,
    Ready,
    Executing,
    Blocked,
    Done,
    Held,
}
