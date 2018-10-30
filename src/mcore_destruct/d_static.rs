
use std::ops::Deref;
use mcore::MultStatic;
use mcore::MultDestruct;
use mcore::ErrAddTask;
use mcore_task::Task;
use mcore::MultTaskManager;
use mcore::MultStat;


#[derive(Debug)]
pub struct StaticDestruct;

impl StaticDestruct {
     #[inline]
     pub const fn new() -> Self {
          StaticDestruct
     }

     #[inline(always)]
     fn as_self(&self) -> &'static MultStatic<'static> {
          ::mcore_static::as_mult_thread()
     }
}


impl Deref for StaticDestruct {
     type Target = MultStatic<'static>;

     #[inline(always)]
     fn deref(&self) -> &Self::Target {
          self.as_self()
     }
}


impl MultStat for StaticDestruct {
     #[inline(always)]
     fn count_threads(&self) -> usize {
          self.as_self().count_threads()
     }
	
	#[inline(always)]
	fn def_count_threads(&self) -> usize {
          self.as_self().def_count_threads()
     }

	#[inline(always)]
	fn min_count_threads(&self) -> usize {
		self.as_self().min_count_threads()
	}

	#[inline(always)]
	fn max_count_threads(&self) -> usize {
		self.as_self().max_count_threads()
	}
}

impl MultTaskManager for StaticDestruct {
     #[inline(always)]
     fn task_array(&self, arr: Vec<Task>) -> Result<(), ErrAddTask> {
		self.as_self().task_array(arr)
	}

     #[inline(always)]
	fn task(&self, e: Task) -> Result<(), ErrAddTask> {
          self.as_self().task(e)
     }
}

impl<'a> MultDestruct for StaticDestruct {
     #[inline(always)]
     fn destruct(&self) {
          self.as_self().destruct()
     }
}

impl<'a> MultStatic<'a> for StaticDestruct {}

impl Drop for StaticDestruct {
     #[inline(always)]
     fn drop(&mut self) {
          self.as_self().destruct();
     }
}
