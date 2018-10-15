

use mult_core_task::Task;
use mult_core_task::ERunTask;
use mult_core_task::run::RunTask;
use std::boxed::FnBox;
use std::fmt::Debug;
use mult_core_task::fnbox::DebugFnBox;

pub trait MultTaskManager: Debug {
	#[inline]
	fn boxfn(&self, f: Box<FnBox() + Send>) -> Result<(), ErrAddTask> {
		self.task(ERunTask::BoxFn(DebugFnBox::new(f)))
	}
	#[inline]
	fn run_task(&self, f: Box<RunTask>) -> Result<(), ErrAddTask> {
		self.task(ERunTask::RunTask(f))
	}
	
	fn task(&self, e: Task) -> Result<(), ErrAddTask>;
}



#[derive(Debug)]
pub enum ErrAddTask {
	NotReady(Task),
	Overflow(Task),
}

