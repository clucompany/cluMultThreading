

use mult_core_task::run::RunTask;
use mult_core_task::run::wait::WaitTaskDisconnect;
use mult_core_task::Task;
use std::fmt::Debug;
use mult_core_task::run::wait::WaitTask;


pub trait MultTaskManager: Debug {

	fn wait<T: 'static>(&self, e: T) -> Result<WaitTaskDisconnect, ErrAddTask> where T: RunTask, Self: Sized {
		let (task, disconnect) = WaitTask::new(e);
		if let Err(e) = self.task(task.boxed()) {
			return Err(e);
		}
		
		Ok( disconnect )
	}

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



#[derive(Debug)]
pub enum ErrAddTask {
	NotReady(Task),
	NotArrayReady(Vec<Task>),
	Overflow(Task),
}

