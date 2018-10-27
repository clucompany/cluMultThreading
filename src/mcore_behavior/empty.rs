
use mcore_task::Task;
use mcore::MultDestruct;
use mcore::ErrAddTask;
use mcore::MultTaskManager;
use mcore::SetCountResult;
use mcore::ErrSetCount;
use mcore::ErrDelThread;
use mcore::MultThreadManager;
use mcore::ErrAddThread;
use mcore::MultStat;
use mcore::MultDefault;
use mcore::MultRawDefault;
use mcore::MultExtend;
use mcore::MultStatic;


#[derive(Debug)]
pub struct MultEmptyCore;

impl MultEmptyCore {
     
}

impl MultRawDefault for MultEmptyCore {
     type NewRes = MultEmptyCore;

     #[inline(always)]
     fn new() -> Self::NewRes {
          MultEmptyCore
     }
     #[inline(always)]
     fn thread(_c: usize) -> Self::NewRes {
          MultEmptyCore
     }
     #[inline(always)]
     fn sys() -> Self::NewRes {
          MultEmptyCore
     }
}


impl MultStat for MultEmptyCore {
     #[inline(always)]
     fn count_threads(&self) -> usize {
          0
     }
     #[inline(always)]
     fn max_count_threads(&self) -> usize {
          0
     }
}

impl MultThreadManager for MultEmptyCore {
     fn add_thread(&self, count_threads: usize) -> Result<usize, ErrAddThread> {
          Ok( count_threads )
     }
	fn del_thread(&self, count_threads: usize) -> Result<usize, ErrDelThread> {
          Ok( count_threads )
     }

	fn set_count_thread(&self, new_count: usize) -> Result<SetCountResult, ErrSetCount> {
          //Err( ErrSetCount::ErrMinThreads{to: new_count, this: 0, min: 0} )
          Ok( SetCountResult::None(new_count) )
     }
}


impl MultTaskManager for MultEmptyCore {
     #[inline]
	fn task(&self, e: Task) -> Result<(), ErrAddTask> {
          Err( ErrAddTask::NotReady(e) )
     }
     #[inline]
     fn task_array(&self, arr: Vec<Task>) -> Result<(), ErrAddTask> {
          Err( ErrAddTask::NotArrayReady(arr) )
     }
}

impl MultDestruct for MultEmptyCore {
     #[inline(always)]
	fn destruct(&self) {
          
     }
}

impl MultDefault<MultEmptyCore> for MultEmptyCore {}
impl<'a> MultExtend<'a> for MultEmptyCore {}
impl<'a> MultStatic<'a> for MultEmptyCore {}