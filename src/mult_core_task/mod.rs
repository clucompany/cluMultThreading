
pub mod task;
pub mod empty_task;
pub mod union_task;
pub mod wait_task;

use std::ops::Deref;
use mult_core_task::task::RunTask;
use std::fmt::Formatter;
use std::boxed::FnBox;
use std::fmt::Debug;
use std::fmt;


#[derive(Debug)]
pub enum ERunTask {
     BoxFn(DebugFnBox),
     RunTask(Box<RunTask>),
     //TwoRunTask(Box<RunTask>, Box<RunTask>),
     //ERunTask(Box<ERunTask>, Box<ERunTask>),

     /*WaitUnlock(Thread),
     WaitBoxFnUnlock(Thread, Box<FnBox()>),
     WaitRunTaskUnlock(Thread, Box<RunTask>),
     WaitTwoRunTaskUnlock(Thread, Box<RunTask>, Box<RunTask>),
     WaitERunTaskUnlock(Thread, Box<ERunTask>, Box<ERunTask>),*/

     MultDestruct,
}

impl ERunTask {
     pub fn run(self) {
          match self {
               ERunTask::BoxFn(a) => {
                    a.run();
               },
               ERunTask::RunTask(mut a) => {
                    a.run();
               },
               ERunTask::MultDestruct => {
                    
               },
          }
     }
}


pub struct DebugFnBox(Box<FnBox()>);

impl DebugFnBox {
     #[inline]
     pub fn new(f: Box<FnBox()>) -> Self {
          DebugFnBox(f)
     }

     #[inline(always)]
     pub fn run(self) {
          self.0()
     }
}

impl Deref for DebugFnBox {
     type Target = Box<FnBox()>;

     #[inline(always)]
     fn deref<'a>(&'a self) -> &'a Self::Target {
          &self.0
     }
}

impl Debug for DebugFnBox {
     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
          write!(f, "FnBox()")
     }
}

