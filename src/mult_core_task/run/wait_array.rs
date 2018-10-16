

use mult_core_task::run::RunTask;
use std::thread::Thread;
use mult_core_task::Task;


#[derive(Debug)]
pub struct WaitArrayTask<'a>(&'a mut [Box<Task>], Thread);


impl<'a> WaitArrayTask<'a> {
     pub fn new(a: &'a mut [Box<Task>]) -> (Self, WaitArrayTaskDisconnect) {
          Self::thread(std::thread::current(), a)
     }

     #[inline]
     pub fn thread(t: Thread, a: &'a mut [Box<Task>]) -> (Self, WaitArrayTaskDisconnect) {
          ( WaitArrayTask(a, t.clone()), WaitArrayTaskDisconnect(t) )
     }
}

impl<'a> RunTask for WaitArrayTask<'a> {
     #[inline(always)]
     fn run(&mut self) {
          for mut a in self.0.iter_mut() {
               a.run();
          }
          self.1.unpark();
     }
}

#[derive(Debug)]
pub struct WaitArrayTaskDisconnect(Thread);
