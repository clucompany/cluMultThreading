


use mcore_task::run::RunTask;

#[derive(Debug)]
///A task combines any two tasks.
pub struct UnionTask<T: RunTask, T2: RunTask>(T, T2);


impl<T: RunTask, T2: RunTask> UnionTask<T, T2> {
     #[inline]
     pub const fn new(t: T, t2: T2) -> Self {
          UnionTask(t, t2)
     }
     #[inline]
     pub const fn impled(t: T, t2: T2) -> impl RunTask {
          Self::new(t, t2)
     }

     #[inline]
     pub const fn as_one<'a>(&'a mut self) -> &'a RunTask {
          &self.0
     }
     #[inline]
     pub const fn as_two<'a>(&'a mut self) -> &'a RunTask {
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

impl<'a, T: RunTask + Clone, T2: RunTask + Clone> Clone for UnionTask<T, T2> {
     #[inline]
     fn clone(&self) -> Self {
          Self::new(self.0.clone(), self.1.clone())
     }
}

