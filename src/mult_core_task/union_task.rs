


use mult_core_task::task::RunTask;

#[derive(Debug)]
pub struct UnionTask<T: RunTask, T2: RunTask>(T, T2);


impl<T: RunTask, T2: RunTask> UnionTask<T, T2> {
     #[inline]
     pub fn new(t: T, t2: T2) -> Self {
          UnionTask(t, t2)
     }
     #[inline]
     pub fn impled(t: T, t2: T2) -> impl RunTask {
          Self::new(t, t2)
     }

     #[inline]
     pub fn as_one<'a>(&'a mut self) -> &'a RunTask {
          &self.0
     }
     #[inline]
     pub fn as_two<'a>(&'a mut self) -> &'a RunTask {
          &self.1
     }
}

impl<T: RunTask, T2: RunTask> RunTask for UnionTask<T, T2> {
     #[inline(always)]
     fn run(&mut self) {
          self.0.run();
          self.1.run();
     }
}


