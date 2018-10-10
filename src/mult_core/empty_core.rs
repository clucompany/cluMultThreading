
use mult_core_task::ERunTask;
use mult_core::MultExtend;
use mult_core::MultDestruct;
use mult_core::ErrAddDistrib;
use mult_core::MultTaskManager;
use mult_core::ErrSetCount;
use mult_core::SetCountResult;
use mult_core::ErrDelThread;
use mult_core::ErrAddThread;
use mult_core::MultThreadManager;
use mult_core::MultStat;
use mult_core::MultStatic;


#[derive(Debug, Default)]
pub struct MultEmptyCore;


impl<'a> MultStat<'a> for MultEmptyCore {
     fn def_thread_len(&self) -> usize {
          0
     }
	
	fn as_count_threads(&self) -> &usize {
          &0
     }
	fn as_min_count_threads(&self) -> &usize {
          &0
     }
}

impl<'a> MultThreadManager<'a> for MultEmptyCore {
     fn add_thread(&self, count_threads: usize) -> Result<usize, ErrAddThread> {
          Ok( count_threads )
     }
	fn del_thread(&self, count_threads: usize) -> Result<usize, ErrDelThread> {
          Ok( count_threads )
     }

	fn set_count_thread(&self, new_count: usize) -> Result<SetCountResult, ErrSetCount> {
          Err( ErrSetCount::ErrMinThreads{ new_count: new_count, min_threads: 0 } )
     }
}


impl<'a> MultTaskManager<'a> for MultEmptyCore {
	fn add_erun(&self, e: ERunTask) -> Result<(), ErrAddDistrib> {
          Err( ErrAddDistrib::NotReady(e) )
     }
}

impl<'a> MultDestruct<'a> for MultEmptyCore {
     #[inline(always)]
	fn destruct(&self) {
          
     }
}


impl<'a> MultExtend<'a> for MultEmptyCore {}
impl<'a> MultStatic<'a> for MultEmptyCore {}