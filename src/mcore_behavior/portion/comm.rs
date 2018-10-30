
use std::sync::Arc;
use std::sync::Barrier;
use mcore_task::Task;


#[derive(Debug)]
pub enum CommPartion {
     Task(Task),
     
     Kill,
     WaitKill(Arc<Barrier>),


     TransferQueue(Vec<CommPartion>),
     TransferTask(Vec<Task>),
     

     UpFlowQueue(usize),
}
