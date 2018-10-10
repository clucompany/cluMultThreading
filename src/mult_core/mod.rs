
extern crate num_cpus;


use mult_core_task::DebugFnBox;
use mult_core_task::task::RunTask;
use mult_core_task::ERunTask;
use std::boxed::FnBox;
use std::fmt::Debug;

pub mod core;
pub mod empty_core;


pub trait MultStatic<'a>: MultStat<'a> + MultThreadManager<'a> + MultTaskManager<'a> + MultDestruct<'a> + Debug {}
pub trait MultExtend<'a>: Default + MultStat<'a> + MultThreadManager<'a> + MultTaskManager<'a> + MultDestruct<'a> + Debug {}

pub trait MultStat<'a>: Debug {
	fn def_thread_len(&'a self) -> usize;
	
	fn as_count_threads(&'a self) -> &'a usize;
	fn as_min_count_threads(&'a self) -> &'a usize;
}

pub trait MultThreadManager<'a>: MultStat<'a> + Debug {
	fn add_thread(&'a self, count_threads: usize) -> Result<usize, ErrAddThread>;
	fn del_thread(&'a self, count_threads: usize) -> Result<usize, ErrDelThread>;

	fn set_count_thread(&'a self, new_count: usize) -> Result<SetCountResult, ErrSetCount>;
}

pub trait MultTaskManager<'a>: Debug {
	#[inline]
	fn boxfn(&'a self, f: Box<FnBox()>) -> Result<(), ErrAddDistrib> {
		self.add_erun(ERunTask::BoxFn(DebugFnBox::new(f)))
	}
	#[inline]
	fn task(&'a self, f: Box<RunTask>) -> Result<(), ErrAddDistrib> {
		self.add_erun(ERunTask::RunTask(f))
	}
	
	fn add_erun(&'a self, e: ERunTask) -> Result<(), ErrAddDistrib>;
}

pub trait MultDestruct<'a>: MultStat<'a> + Debug {
	fn destruct(&self);
}


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


#[derive(Debug)]
pub enum ErrAddDistrib {
	NotReady(ERunTask),
	Overflow(ERunTask),
}



#[derive(Debug)]
pub enum SetCountResult {
	None(usize),
     AddThread(Result<usize, ErrAddThread>),
     DelThread(Result<usize, ErrDelThread>),
}

#[derive(Debug)]
pub enum ErrSetCount {
     ErrMinThreads {
		new_count: usize,
		min_threads: usize,
	},
}


#[derive(Debug)]
pub enum ErrAddThread {
	NotAdded(usize),
}

#[derive(Debug)]
pub enum ErrDelThread {
	NotDel(usize),
}
