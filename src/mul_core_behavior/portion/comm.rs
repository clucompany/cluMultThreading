
use std::sync::Barrier;
use std::sync::Arc;
use std::thread::Thread;
use mult_core_task::Task;


#[derive(Debug)]
pub enum CommPartion {
     Task(Task),
     
     Kill,
     
     
     TransferQueue(Vec<CommPartion>),

     UpFlowQueue(usize),
}
