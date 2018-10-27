

use std::fmt::Debug;
use mcore::stat::MultStat;

pub trait MultThreadManager: MultStat + Debug {
	fn add_thread(&self, count_threads: usize) -> Result<usize, ErrAddThread>;
	fn del_thread(&self, count_threads: usize) -> Result<usize, ErrDelThread>;

	fn set_count_thread(&self, new_count: usize) -> Result<SetCountResult, ErrSetCount>;
}


impl<'a, A: MultThreadManager> MultThreadManager for &'a A {
	#[inline(always)]
	fn add_thread(&self, count_threads: usize) -> Result<usize, ErrAddThread> {
		(*self).add_thread(count_threads)
	}

	#[inline(always)]
	fn del_thread(&self, count_threads: usize) -> Result<usize, ErrDelThread> {
		(*self).del_thread(count_threads)
	}

	#[inline(always)]
	fn set_count_thread(&self, new_count: usize) -> Result<SetCountResult, ErrSetCount> {
		(*self).set_count_thread(new_count)
	}
}


impl<'a, A: MultThreadManager> MultThreadManager for &'a mut A {
	#[inline(always)]
	fn add_thread(&self, count_threads: usize) -> Result<usize, ErrAddThread> {
		(**self).add_thread(count_threads)
	}

	#[inline(always)]
	fn del_thread(&self, count_threads: usize) -> Result<usize, ErrDelThread> {
		(**self).del_thread(count_threads)
	}

	#[inline(always)]
	fn set_count_thread(&self, new_count: usize) -> Result<SetCountResult, ErrSetCount> {
		(**self).set_count_thread(new_count)
	}
}


#[derive(Debug)]
pub enum SetCountResult {
	None(usize),
     Add(usize),
     Del(usize),
}


#[derive(Debug)]
pub enum ErrSetCount {
     ErrMinThreads{to: usize, this: usize, min: usize},
	ErrMaxThreads{to: usize, this: usize, max: usize},
	ErrAddThread(ErrAddThread),
	ErrDelThread(ErrDelThread),
}


#[derive(Debug)]
pub enum ErrAddThread {
	Empty(usize),
	ErrMax{new: usize, max: usize},
}

#[derive(Debug)]
pub enum ErrDelThread {
	Empty(usize),
}
