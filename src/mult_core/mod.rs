
pub mod default;
pub mod stat;
pub mod destruct;
pub mod task;
pub mod thread;


use mult_core::thread::MultThreadManager;
use mult_core::task::MultTaskManager;
use mult_core::destruct::MultDestruct;
use mult_core::stat::MultStat;
use mult_core::default::MultDefault;
use std::fmt::Debug;

pub trait MultStatic<'a>: MultStat + MultTaskManager + MultDestruct + Debug {}
pub trait MultExtend<'a>: MultDefault + MultThreadManager + MultStat + MultTaskManager + MultDestruct + Debug {}




/*
		let lock_count = self.as_count_thread();
		inf!("Connector threads {}->{}", lock_count, new_count);

		if *lock_count == new_count {
			warn!("Runnable, unknown set_count_threads count, len == count");
			return Ok( SetCountResult::None );
		}
		
		if *lock_count > new_count {
			let ncount = (*lock_count)-new_count;
			return Ok( SetCountResult::DelThread( self.del_thread(new_count) ) );
		}
		Ok( SetCountResult::AddThread( self.add_thread(new_count) ) )
	}
	fn add_thread(&'a mut self, count_threads: usize) -> Result<usize, ErrAddThread>;
	fn del_thread(&'a mut self, count_threads: usize) -> Result<usize, ErrDelThread>;
*/









