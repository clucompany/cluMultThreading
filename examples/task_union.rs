
#[macro_use]
extern crate cluMultThreading;
#[macro_use]
extern crate cluLog;

use cluMultThreading::mcore::MultStat;
use cluMultThreading::mcore::MultTaskManager;
use cluMultThreading::mcore::MultDefault;
use cluMultThreading::mcore_task::RunTask;
use cluMultThreading::mcore_task::EmptyTask;

fn main() {
     init_clulog!();
     
     let all = EmptyTask::new()
          .union(EmptyTask::new())
          .union(EmptyTask::new())
          .union(PrintlnTask("Test"))
          .union(PrintlnTask("Test"))
          .union(ConcatPrintlnTask("1", "2"))
          .union(EmptyTask::new())
          .boxed();
     

     let tasker = cluMultThreading::mcore_behavior::PortionCore::common().unwrap();
     for _a in 0..10 {
          let _e = tasker.task(all.clone());
     }
     println!("Count threads: {}", tasker.count_threads());

     drop(tasker);
     trace!("1");
}


#[derive(Debug, Clone)]
pub struct PrintlnTask<'a>(&'a str);

impl<'a> RunTask for PrintlnTask<'a> {
     fn run(&mut self) {
          println!("{}", self.0);
     }
}


#[derive(Debug, Clone)]
pub struct ConcatPrintlnTask<'a>(&'a str, &'a str);

impl<'a> RunTask for ConcatPrintlnTask<'a> {
     fn run(&mut self) {
          println!("{}->>{}", self.0, self.1);
     }
}