
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
pub struct MultRootDestruct<'a, M: MultExtend<'a> + Sized + 'a>(M, PhantomData<&'a ()>);


impl<'a, M: MultExtend<'a> + Sized + 'a> MultRootDestruct<'a, M> {
     #[inline]
     pub const fn new(a: M) -> Self {
          MultRootDestruct(a, PhantomData)
     }
}

impl<'a, M: MultExtend<'a> + Sized + 'a> MultStat for MultRootDestruct<'a, M> {
     #[inline(always)]
     fn count_threads(&self) -> usize {
          self.0.count_threads()
     }
	
	#[inline(always)]
	fn def_count_threads(&self) -> usize {
          self.0.def_count_threads()
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

impl<'a, M: MultExtend<'a> + Sized + 'a> MultTaskManager for MultRootDestruct<'a, M> {
     #[inline(always)]
     fn task_array(&self, arr: Vec<Task>) -> Result<(), ErrAddTask> {
		self.0.task_array(arr)
	}

     #[inline(always)]
	fn task(&self, e: Task) -> Result<(), ErrAddTask> {
          self.0.task(e)
     }
}
impl<'a, M: MultExtend<'a> + Sized + 'a> MultThreadManager for MultRootDestruct<'a, M> {
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
}

impl<'a, M: MultExtend<'a> + Sized + 'a> MultDestruct for MultRootDestruct<'a, M> {
     #[inline(always)]
     fn destruct(&self) {
          self.0.destruct()
     }
}


impl<'a, M: MultExtend<'a> + Sized + 'a> Drop for MultRootDestruct<'a, M> {
     #[inline(always)]
     fn drop(&mut self) {
          self.0.destruct();
     }
}



impl<'a, M: MultExtend<'a> + Sized + 'a> MultExtend<'a> for MultRootDestruct<'a, M> {}

