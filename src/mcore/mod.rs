
mod default;
mod stat;
mod destruct;
mod task;
mod thread;

pub use self::default::*;
pub use self::stat::*;
pub use self::destruct::*;
pub use self::task::*;
pub use self::thread::*;

use std::fmt::Debug;

///Allows you to use the scheduler as a general scheduler. Generalizations with the requirement of Sized are not supported.
pub trait MultStatic<'a>: MultStat + MultTaskManager + MultDestruct + Debug {}

///Allows you to use the scheduler as a local scheduler. Additional advanced scheduling features are also supported.
pub trait MultExtend<'a>: MultThreadManager + MultStat + MultTaskManager + MultDestruct + Debug + Sized {}




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









