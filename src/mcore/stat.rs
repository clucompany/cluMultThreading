

use std::fmt::Debug;

///Providing scheduler statistics.
pub trait MultStat: Debug {
	///Number of threads in the scheduler.
	fn count_threads(&self) -> usize;
	
	///Starting number of threads in the scheduler.
	#[inline(always)]
	fn start_count_threads(&self) -> usize {
          0
     }

	///Minimum number of threads in the scheduler.
	#[inline(always)]
	fn min_count_threads(&self) -> usize {
		0
	}

	///The maximum possible number of threads in the scheduler.
	#[inline(always)]
	fn max_count_threads(&self) -> usize {
		usize::max_value()
	}
}

///Providing scheduler statistics.
impl<'a, A: MultStat> MultStat for &'a A {
	///Number of threads in the scheduler.
	#[inline(always)]
	fn count_threads(&self) -> usize {
		(**self).count_threads()
	}
	
	///Starting number of threads in the scheduler.
	#[inline(always)]
	fn start_count_threads(&self) -> usize {
          (**self).start_count_threads()
     }

	///Minimum number of threads in the scheduler.
	#[inline(always)]
	fn min_count_threads(&self) -> usize {
		(**self).min_count_threads()
	}

	///The maximum possible number of threads in the scheduler.
	#[inline(always)]
	fn max_count_threads(&self) -> usize {
		(**self).max_count_threads()
	}
}

///Providing scheduler statistics.
impl<'a, A: MultStat> MultStat for &'a mut A {
	///Number of threads in the scheduler.
	#[inline(always)]
	fn count_threads(&self) -> usize {
		(**self).count_threads()
	}
	
	///Starting number of threads in the scheduler.
	#[inline(always)]
	fn start_count_threads(&self) -> usize {
          (**self).start_count_threads()
     }

	///Minimum number of threads in the scheduler.
	#[inline(always)]
	fn min_count_threads(&self) -> usize {
		(**self).min_count_threads()
	}

	///The maximum possible number of threads in the scheduler.
	#[inline(always)]
	fn max_count_threads(&self) -> usize {
		(**self).max_count_threads()
	}
}