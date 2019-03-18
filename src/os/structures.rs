use crate::records::ProcessData;

/** The Process Control Block (PCB) for each process
*/
pub struct ProcessControlBlock {
    pub info: ProcessData,      // process info,
    pub id: i32,
    pub clk: i32,               // current CPU clock
    pub state: State,
    pub total_cpu: i32,         // total CPU cycles completed
    pub total_ios: i32,         // total IO cycles completed
    pub start_time: i32,
    pub end_time: i32,
}
pub enum State { Allocating, Ready, Executing, Blocked, Done, Held }

/** An OS event -- either IO completion or timeout.
    .time: time at event creation
    .pid: pid of process that created event
 */
pub enum Event {
    IO {time: i32, pid: i32},
    Timeout {time: i32, pid: i32},
}

impl ProcessControlBlock {
    //TODO
    pub fn allocate(&mut self) {

    }
}
