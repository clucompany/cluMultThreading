
#[macro_use]
extern crate cluLog;
extern crate cluMultThreading;


use cluMultThreading::mult_core_task::run::RunTask;
use cluMultThreading::mult_core::default::MultDefault;
use cluMultThreading::mult_core_task::run::function::TaskFn;


pub fn main() {
     init_clulog!();
     
     let drop = cluMultThreading::mul_core_behavior::portion::PortionCore::common().unwrap();
     
     let w = drop.wait(TaskFn::new(
          || {
               trace!("Test");
          }
     ));
     println!("1");
     drop(w);

     /*for a in 0..30 {
          let disconn = {
               let (task, disconn) = TaskFn::new(
                    || {
                         trace!("Test");
                    }
               ).wait();
               
               
               drop.task(task.boxed());
          };
     }*/
     inf!("END");

}
