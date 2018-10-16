

pub mod empty;
pub mod union;
pub mod wait;
pub mod box_wait;
pub mod wait_array;
pub mod function;



use mult_core_task::run::wait::WaitTaskDisconnect;
use mult_core_task::run::wait::WaitTask;
use self::union::UnionTask;
use std::fmt::Debug;

pub trait RunTask: Debug + Send + Sync {
     fn run(&mut self);

     #[inline]
     fn boxed(self) -> Box<Self> where Self: Sized {
          Box::new(self)
     }
     
     #[inline]
     fn union<T: RunTask + Sized >(self, t: T) -> UnionTask<Self, T> where Self: Sized {
          UnionTask::new(self, t)
     }

     #[inline]
     fn wait(self) -> (WaitTask<Self>, WaitTaskDisconnect) where Self: Sized {
          WaitTask::new(self)
     }
}




