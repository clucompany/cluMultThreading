

use mult_core_task::ERunTask;
use mult_core_task::union_task::UnionTask;
use std::fmt::Debug;

pub trait RunTask: Debug {
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
     fn erun(self) -> ERunTask where Self: Sized + 'static {
          Self::b_erun(Box::new(self))
     }

     #[inline]
     fn b_erun(self: Box<Self>) -> ERunTask where Self: Sized + 'static {
          ERunTask::RunTask(self)
     }
}


