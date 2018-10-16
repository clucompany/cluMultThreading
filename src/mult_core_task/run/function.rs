

use std::fmt::Formatter;
use std::fmt::Debug;
use mult_core_task::run::RunTask;
use std::ops::FnMut;
use std::fmt;

pub struct TaskFn<F: Send + FnMut() + Sync>(F);


impl<F: Send + Fn() + Sync> Debug for TaskFn<F> {
     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
          write!(f, "TaskFn(Fn())")
     }
}

impl<F: Send + Fn() + Sync> TaskFn<F> {
     #[inline(always)]
     pub const fn new(f: F) -> Self {
          TaskFn(f)
     }
}

impl<F: Send + Fn() + Sync> RunTask for TaskFn<F> {
     #[inline(always)]
     fn run(&mut self) {
          self.0()
     }
}
