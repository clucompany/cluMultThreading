

use std::thread::Thread;
use mult_core_task::run::RunTask;


#[derive(Debug)]
pub struct WaitTask<T: RunTask>(Thread, T);

impl<T: RunTask> WaitTask<T> {
     pub fn new(t: T) -> (Self, WaitTaskDisconnect) {
          Self::thread(std::thread::current(), t)
     }

     #[inline]
     pub fn thread(thread: Thread, t: T) -> (Self, WaitTaskDisconnect) {
          ( WaitTask(thread.clone(), t), WaitTaskDisconnect(thread) )
     }

     #[inline(always)]
     pub fn as_run(&mut self) -> &RunTask {
          &self.1
     }
}


impl<T: RunTask> RunTask for WaitTask<T> {
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
