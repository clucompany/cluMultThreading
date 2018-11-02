
use mcore::MultThreadManager;
use mcore::ErrSetCount;
use mcore::SetCountResult;
use mcore::ErrDelThread;
use mcore::ErrAddThread;
use mcore::MultDestruct;
use mcore::ErrAddTask;
use mcore_task::Task;
use mcore::MultTaskManager;
use mcore::MultStat;
use std::marker::PhantomData;
use mcore::MultExtend;

#[derive(Debug)]
pub struct RootDestruct<'a, M: MultExtend<'a>>(M, PhantomData<&'a ()>);


impl<'a, M: MultExtend<'a>> RootDestruct<'a, M> {
     #[inline]
     pub const fn new(a: M) -> Self {
          RootDestruct(a, PhantomData)
     }
}

impl<'a, M: MultExtend<'a>> MultStat for RootDestruct<'a, M> {
     #[inline(always)]
     fn count_threads(&self) -> usize {
          self.0.count_threads()
     }
	
	#[inline(always)]
	fn start_count_threads(&self) -> usize {
          self.0.start_count_threads()
     }

	#[inline(always)]
	fn min_count_threads(&self) -> usize {
		self.0.min_count_threads()
	}

	#[inline(always)]
	fn max_count_threads(&self) -> usize {
		self.0.max_count_threads()
	}
}

impl<'a, M: MultExtend<'a>> MultTaskManager for RootDestruct<'a, M> {
     #[inline(always)]
     fn task_array(&self, arr: Vec<Task>) -> Result<(), ErrAddTask> {
		self.0.task_array(arr)
	}

     #[inline(always)]
	fn task(&self, e: Task) -> Result<(), ErrAddTask> {
          self.0.task(e)
     }
}
impl<'a, M: MultExtend<'a>> MultThreadManager for RootDestruct<'a, M> {
     #[inline(always)]
     fn add_thread(&self, count_threads: usize) -> Result<usize, ErrAddThread> {
          self.0.add_thread(count_threads)
     }
	#[inline(always)]
     fn del_thread(&self, count_threads: usize) -> Result<usize, ErrDelThread> {
          self.0.del_thread(count_threads)
     }

     #[inline(always)]
	fn set_count_thread(&self, new_count: usize) -> Result<SetCountResult, ErrSetCount> {
          self.0.set_count_thread(new_count)
     }

     #[inline(always)]
     fn async_add_thread(&self, count_threads: usize) -> Result<usize, ErrAddThread> {
          self.0.async_add_thread(count_threads)
     }
	#[inline(always)]
     fn async_del_thread(&self, count_threads: usize) -> Result<usize, ErrDelThread> {
          self.0.async_del_thread(count_threads)
     }

     #[inline(always)]
	fn async_set_count_thread(&self, new_count: usize) -> Result<SetCountResult, ErrSetCount> {
          self.0.async_set_count_thread(new_count)
     }
}

impl<'a, M: MultExtend<'a>> MultDestruct for RootDestruct<'a, M> {
     #[inline(always)]
     fn destruct(&self) {
          self.0.destruct()
     }
}


impl<'a, M: MultExtend<'a>> Drop for RootDestruct<'a, M> {
     #[inline(always)]
     fn drop(&mut self) {
          self.0.destruct();
     }
}



impl<'a, M: MultExtend<'a>> MultExtend<'a> for RootDestruct<'a, M> {}

