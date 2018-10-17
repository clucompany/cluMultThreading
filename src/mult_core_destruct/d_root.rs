
use mult_core::thread::MultThreadManager;
use mult_core::thread::ErrSetCount;
use mult_core::thread::SetCountResult;
use mult_core::thread::ErrDelThread;
use mult_core::thread::ErrAddThread;
use mult_core::destruct::MultDestruct;
use mult_core::task::ErrAddTask;
use mult_core_task::Task;
use mult_core::task::MultTaskManager;
use mult_core::stat::MultStat;
use std::marker::PhantomData;
use mult_core::MultExtend;

#[derive(Debug)]
pub struct MultRootDestruct<'a, M: MultExtend<'a> + Sized + 'a>(M, PhantomData<&'a ()>);


impl<'a, M: MultExtend<'a> + Sized + 'a> MultRootDestruct<'a, M> {
     #[inline]
     pub const fn new(a: M) -> Self {
          MultRootDestruct(a, PhantomData)
     }
}


impl<'a, M: MultExtend<'a> + Sized + 'a> MultStat for &'a MultRootDestruct<'a, M> {
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

impl<'a, M: MultExtend<'a> + Sized + 'a> MultTaskManager for &'a MultRootDestruct<'a, M> {
     #[inline(always)]
     fn task_array(&self, arr: Vec<Task>) -> Result<(), ErrAddTask> {
		self.0.task_array(arr)
	}

     #[inline(always)]
	fn task(&self, e: Task) -> Result<(), ErrAddTask> {
          self.0.task(e)
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

impl<'a, M: MultExtend<'a> + Sized + 'a> MultThreadManager for &'a MultRootDestruct<'a, M> {
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

impl<'a, M: MultExtend<'a> + Sized + 'a> MultDestruct for &'a MultRootDestruct<'a, M> {
     #[inline(always)]
     fn destruct(&self) {
          self.0.destruct()
     }
}

impl<'a, M: MultExtend<'a> + Sized + 'a> MultDestruct for MultRootDestruct<'a, M> {
     #[inline(always)]
     fn destruct(&self) {
          self.0.destruct()
     }
}

impl<'a, M: MultExtend<'a> + Sized + 'a> MultExtend<'a> for &'a MultRootDestruct<'a, M> {}
impl<'a, M: MultExtend<'a> + Sized + 'a> MultExtend<'a> for MultRootDestruct<'a, M> {}

impl<'a, M: MultExtend<'a> + Sized + 'a> Drop for MultRootDestruct<'a, M> {
     #[inline(always)]
     fn drop(&mut self) {
          self.0.destruct();
     }
}

