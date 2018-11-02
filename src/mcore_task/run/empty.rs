
use mcore_task::run::RunTask;


#[derive(Debug, Default, Clone)]
///A task that does nothing.
pub struct EmptyTask;

impl EmptyTask {
     #[inline(always)]
     pub const fn new() -> Self {
          EmptyTask
     }
}

impl RunTask for EmptyTask {
     #[inline(always)]
     fn run(&mut self) {
          
     }
}