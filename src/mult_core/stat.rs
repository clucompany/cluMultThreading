

use std::fmt::Debug;

pub trait MultStat: Debug {
	fn count_threads(&self) -> usize;
	
	#[inline(always)]
	fn def_count_threads(&self) -> usize {
          0
     }

	#[inline(always)]
	fn min_count_threads(&self) -> usize {
		0
	}

	#[inline(always)]
	fn max_count_threads(&self) -> usize {
		usize::max_value()
	}
}

