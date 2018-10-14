
use std::sync::Arc;
use mult_core_task::Task;
use mult_core::destruct::MultDestruct;
use mult_core::task::ErrAddTask;
use mult_core::task::MultTaskManager;
use mult_core::thread::SetCountResult;
use mult_core::thread::ErrSetCount;
use mult_core::thread::ErrDelThread;
use mult_core::thread::MultThreadManager;
use mult_core::thread::ErrAddThread;
use mult_core::stat::MultStat;
use mult_core::default::MultDefault;
use mult_core::default::MultRawDefault;
use mult_core::MultExtend;
use mult_core::MultStatic;


#[derive(Debug)]
pub struct MultEmptyCore;

impl MultEmptyCore {
     
}

impl MultRawDefault for MultEmptyCore {
     #[inline(always)]
     fn new() -> Self {
          MultEmptyCore
     }
     #[inline(always)]
     fn thread(_c: usize) -> Self {
          MultEmptyCore
     }
     #[inline(always)]
     fn sys() -> Self {
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
	fn task(&self, e: Task) -> Result<(), ErrAddTask> {
          Err( ErrAddTask::NotReady(e) )
     }
}

impl MultDestruct for MultEmptyCore {
     #[inline(always)]
	fn destruct(&self) {
          
     }
}

impl MultDefault for MultEmptyCore {}
impl<'a> MultExtend<'a> for MultEmptyCore {}
impl<'a> MultStatic<'a> for MultEmptyCore {}