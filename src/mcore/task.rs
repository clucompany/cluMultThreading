

use mcore_task::Task;
use std::fmt::Debug;


pub trait MultTaskManager: Debug {
	/*
	fn wait<T: 'static>(&self, e: T) -> Result<WaitTaskDisconnect, ErrAddTask> where T: RunTask, Self: Sized {
		let (task, disconnect) = WaitTask::new(e);
		if let Err(e) = self.task(task.boxed()) {
			return Err(e);
		}
		
		Ok( disconnect )
	}
	*/

	fn task_array(&self, arr: Vec<Task>) -> Result<(), ErrAddTask> {
		for a in arr {
			if let Err(e) = self.task(a) {
				return Err(e);
			}
		}

		Ok( () )
	}

	fn task(&self, e: Task) -> Result<(), ErrAddTask>;
}


impl<'a, A: MultTaskManager> MultTaskManager for &'a A {
	#[inline(always)]
	fn task_array(&self, arr: Vec<Task>) -> Result<(), ErrAddTask> {
		(**self).task_array(arr)
	}

	#[inline(always)]
	fn task(&self, e: Task) -> Result<(), ErrAddTask> {
		(**self).task(e)
	}
}

impl<'a, A: MultTaskManager> MultTaskManager for &'a mut A {
	#[inline(always)]
	fn task_array(&self, arr: Vec<Task>) -> Result<(), ErrAddTask> {
		(**self).task_array(arr)
	}

	#[inline(always)]
	fn task(&self, e: Task) -> Result<(), ErrAddTask> {
		(**self).task(e)
	}
}


#[derive(Debug)]
pub enum ErrAddTask {
	NotReady(Task),
	NotArrayReady(Vec<Task>),
	Overflow(Task),
}

