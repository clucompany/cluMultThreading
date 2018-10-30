

mod empty;
mod union;
mod wait;
mod wait_array;
mod function;

pub use self::empty::*;
pub use self::union::*;
pub use self::wait::*;
pub use self::wait_array::*;
pub use self::function::*;

use std::fmt::Debug;

pub trait RunTask: Debug + Send + Sync {
     fn run(&mut self);
}

impl<'a, A: RunTask> RunTask for &'a mut A {
     #[inline(always)]
     fn run(&mut self) {
          (**self).run()
     }
}


pub trait RunTaskExtension: RunTask + Sized {
     #[inline]
     fn boxed(self) -> Box<Self> {
          Box::new(self)
     }
     
     #[inline]
     fn union<T: RunTask>(self, t: T) -> UnionTask<Self, T> {
          UnionTask::new(self, t)
     }

     #[inline]
     fn wait<'a>(self) -> (WaitTask<'a, Self>, WaitTaskDisconnect) where Self: 'static {
          WaitTask::new(self)
     }
}


impl<'a, R: RunTask> RunTaskExtension for R {

}
