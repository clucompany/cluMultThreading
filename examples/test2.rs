
#[macro_use]
extern crate cluLog;
extern crate cluMultThreading;


use cluMultThreading::mult_core_task::run::RunTask;
use cluMultThreading::mult_core::default::MultDefault;
use cluMultThreading::mult_core_task::run::function::TaskFn;
use cluMultThreading::mult_core_task::run::wait::WaitTask;


pub fn main() {
     init_clulog!();
     
     let drop = cluMultThreading::mul_core_behavior::portion::PortionCore::root();
     
     let w = WaitTask::wait(&drop, TaskFn::new(
          || {
               trace!("Test");
          }
     ));
     let w2 = WaitTask::wait(drop, TaskFn::new(
          || {
               trace!("Test");
          }
     ));
}
