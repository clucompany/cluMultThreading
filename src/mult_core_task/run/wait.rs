

use std::marker::PhantomData;
use mult_core::task::ErrAddTask;
use mult_core::MultExtend;
use std::thread::Thread;
use mult_core_task::run::RunTask;


#[derive(Debug)]
pub struct WaitTask<'a, T: RunTask + 'a>(Thread, T, PhantomData<&'a ()>);


impl<'a, T: RunTask + 'static> WaitTask<'a, T> {
     pub fn wait<'e, E: MultExtend<'e>>(mult: E, task: T) -> Result<WaitTaskDisconnect, ErrAddTask> {
          let (task, disconnect) = WaitTask::new(task);
		if let Err(e) = mult.task(task.boxed()) {
			return Err(e);
		}

          Ok( disconnect )
     }
}

impl<'a, T: RunTask + 'a> WaitTask<'a, T> {
     pub fn new(t: T) -> (Self, WaitTaskDisconnect) {
          Self::thread(std::thread::current(), t)
     }

     #[inline]
     pub fn thread(thread: Thread, t: T) -> (Self, WaitTaskDisconnect) {
          ( WaitTask(thread.clone(), t, PhantomData), WaitTaskDisconnect(thread) )
     }
}


impl<'a, T: RunTask + 'a> RunTask for WaitTask<'a, T> {
     #[inline(always)]
     fn run(&mut self) {
          self.1.run();
          self.0.unpark();
     }
}


#[derive(Debug)]
pub struct WaitTaskDisconnect(Thread);

impl WaitTaskDisconnect {
     #[inline(always)]
     pub fn drop(self) {

     }
}

impl Drop for WaitTaskDisconnect {
     #[inline]
     fn drop(&mut self) {
          std::thread::park();
     }
}
