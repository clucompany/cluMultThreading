

use std::fmt::Formatter;
use std::fmt::Debug;
use mcore_task::run::RunTask;
use std::ops::FnMut;
use std::fmt;

///The task allows to execute Fn on the side of the scheduler.
pub struct FnTask<F: Send + FnMut() + Sync>(F);


impl<F: Send + Fn() + Sync> Debug for FnTask<F> {
     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
          write!(f, "FnTask(Fn())")
     }
}

impl<F: Send + Fn() + Sync> FnTask<F> {
     #[inline(always)]
     pub const fn new(f: F) -> Self {
          FnTask(f)
     }
}

impl<F: Send + Fn() + Sync> RunTask for FnTask<F> {
     #[inline(always)]
     fn run(&mut self) {
          self.0()
     }
}
