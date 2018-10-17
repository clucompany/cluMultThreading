
use mcore_task::Task;


#[derive(Debug)]
pub enum CommPartion {
     Task(Task),
     
     Kill,
     TransferQueue(Vec<CommPartion>),
     TransferTask(Vec<Task>),
     

     UpFlowQueue(usize),
}
