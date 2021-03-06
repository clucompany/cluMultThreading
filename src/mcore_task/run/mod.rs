

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

///A trait that describes the desired behavior of the task.
pub trait RunTask: Debug + Send + Sync {
     fn run(&mut self);

     #[inline]
     fn boxed(self) -> Box<Self> where Self: Sized {
          Box::new(self)
     }
     
     #[inline]
     fn union<T: RunTask>(self, t: T) -> UnionTask<Self, T> where Self: Sized {
          UnionTask::new(self, t)
     }

     #[inline]
     fn wait<'a>(self) -> (WaitTask<'a, Self>, WaitTaskDisconnect) where Self: 'static + Sized {
          WaitTask::new(self)
     }
}


impl<'a, A: RunTask> RunTask for &'a mut A {
     #[inline(always)]
     fn run(&mut self) {
          (**self).run()
     }
}
