
use std::thread::Thread;
use mult_core_task::run::RunTask;



#[derive(Debug)]
pub struct WaitTask<T: RunTask>(Thread, T);

impl<T: RunTask> WaitTask<T> {
     pub fn new(t: T) -> Self {
          Self::thread(std::thread::current(), t)
     }

     #[inline]
     pub fn impled(t: T) -> impl RunTask {
          Self::new(t)
     }

     #[inline]
     pub fn thread(thread: Thread, t: T) -> Self {
          WaitTask(thread, t)
     }

     #[inline]
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


