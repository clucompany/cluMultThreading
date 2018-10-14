
use mult_core_task::run::RunTask;


#[derive(Debug, Default)]
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