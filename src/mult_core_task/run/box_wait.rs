
use std::thread::Thread;
use mult_core_task::run::RunTask;


#[derive(Debug)]
pub struct BoxWaitTask(Thread, Box<RunTask>);

impl BoxWaitTask {
     pub fn new(t: Box<RunTask>) -> (Self, BoxWaitTaskDisconnect) {
          Self::thread(std::thread::current(), t)
     }

     #[inline]
     pub fn thread(thread: Thread, t: Box<RunTask>) -> (Self, BoxWaitTaskDisconnect) {
          ( BoxWaitTask(thread.clone(), t), BoxWaitTaskDisconnect(thread) )
     }

     #[inline(always)]
     pub fn as_run(&mut self) -> &Box<RunTask> {
          &self.1
     }
}


impl RunTask for BoxWaitTask {
     #[inline(always)]
     fn run(&mut self) {
          self.1.run();
          self.0.unpark();
     }
}


#[derive(Debug)]
pub struct BoxWaitTaskDisconnect(Thread);

impl BoxWaitTaskDisconnect {
     #[inline(always)]
     pub fn drop(self) {

     }
}

impl Drop for BoxWaitTaskDisconnect {
     #[inline]
     fn drop(&mut self) {
          std::thread::park();
     }
}
