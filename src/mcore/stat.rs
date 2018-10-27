

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

impl<'a, A: MultStat> MultStat for &'a A {
	#[inline(always)]
	fn count_threads(&self) -> usize {
		(**self).count_threads()
	}
	
	#[inline(always)]
	fn def_count_threads(&self) -> usize {
          (**self).def_count_threads()
     }

	#[inline(always)]
	fn min_count_threads(&self) -> usize {
		(**self).min_count_threads()
	}

	#[inline(always)]
	fn max_count_threads(&self) -> usize {
		(**self).max_count_threads()
	}
}

impl<'a, A: MultStat> MultStat for &'a mut A {
	#[inline(always)]
	fn count_threads(&self) -> usize {
		(**self).count_threads()
	}
	
	#[inline(always)]
	fn def_count_threads(&self) -> usize {
          (**self).def_count_threads()
     }

	#[inline(always)]
	fn min_count_threads(&self) -> usize {
		(**self).min_count_threads()
	}

	#[inline(always)]
	fn max_count_threads(&self) -> usize {
		(**self).max_count_threads()
	}
}