

use std::sync::Arc;
use std::sync::Condvar;
use std::marker::PhantomData;
use mcore::ErrAddTask;
use mcore::MultExtend;
use std::sync::Mutex;
use mcore_task::run::RunTask;
use mcore_task::run::RunTaskExtension;


#[derive(Debug)]
pub struct WaitTask<'a, T: RunTask + 'a>(
     Arc<(Mutex<bool>, Condvar)>,
     T, 
     PhantomData<&'a ()>,
);


impl<'a, T: RunTask + 'static> WaitTask<'a, T> {
     pub fn wait<'e, E: MultExtend<'e>>(mult: E, task: T) -> Result<WaitTaskDisconnect, ErrAddTask> {
          let (task, disconnect) = WaitTask::new(task);
		if let Err(e) = mult.task(task.boxed()) {
			return Err(e);
		}

          Ok( disconnect )
     }
}

impl<'a, T: RunTask + 'a> WaitTask<'a, T> {
     pub fn new(t: T) -> (Self, WaitTaskDisconnect) {
          let arc = Arc::new((Mutex::new(false), Condvar::new()));

          ( WaitTask(arc.clone(), t, PhantomData), WaitTaskDisconnect(arc) )
     }

     
}


impl<'a, T: RunTask + 'a> RunTask for WaitTask<'a, T> {
     #[inline(always)]
     fn run(&mut self) {
          self.1.run();
          
          {
               let mut lock = match (self.0).0.lock() {
                    Ok(a) => a,
                    Err(e) => e.into_inner(),
               };
               *lock = true;
          }

          (self.0).1.notify_one();
     }
}


#[derive(Debug)]
pub struct WaitTaskDisconnect(Arc<(Mutex<bool>, Condvar)>);

impl WaitTaskDisconnect {
     #[inline(always)]
     pub fn wait(self) {

     }
}

impl Drop for WaitTaskDisconnect {
     #[inline]
     fn drop(&mut self) {
          let mut lock = match (self.0).0.lock() {
               Ok(a) => a,
               Err(e) => e.into_inner(),
          };
          while !*lock {
               lock = (self.0).1.wait(lock).unwrap();
          }
     }
}
