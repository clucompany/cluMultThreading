
use mult_core_task::task::RunTask;


#[derive(Debug)]
pub struct EmptyTask;



impl EmptyTask {
     #[inline]
     pub fn new() -> Self {
          EmptyTask
     }
}

impl RunTask for EmptyTask {
     #[inline(always)]
     fn run(&mut self) {
          
     }
}