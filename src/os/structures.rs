use crate::records::ProcessData;
use std::fmt;
use std::cmp::Ordering;

pub type PID = i32;

/** The Process Control Block (PCB) for each process */
pub struct ProcessControlBlock {
    pub info: ProcessData, // process info,
    pub pid: PID,
    pub clk: i32, // current CPU clock
    pub state: State,
    pub total_cpu: i32, // total CPU cycles completed
    pub total_ios: i32, // total IO cycles completed
    pub start_time: i32,
    pub end_time: i32,
    pub memory_map: MemoryRange, // where in memory this process is located
}

impl ProcessControlBlock {
    /** Allocate this process to memory */
    pub fn allocate(&mut self) {}
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

#[derive(Clone)]
pub struct MemoryRange(pub i32, pub i32); // initial and final blocks of memory this process takes up

impl fmt::Display for MemoryRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for i in self.0..=self.1 {
            result.push_str(&(i.to_string() + " "));
        }
        write!(f, "{}", result)
    }
}
#[derive(Debug)]
pub enum State {
    Allocating,
    Ready,
    Executing,
    Blocked,
    Done,
    Held,
}

/** An OS event -- either IO completion or timeout.
    .time: time at event creation
    .pid: pid of process that created event
*/
pub enum Event {
    IO { time: i32, pid: i32 },
    Timeout { time: i32, pid: i32 },
}
