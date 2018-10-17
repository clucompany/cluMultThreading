
#[macro_use]
extern crate cluLog;
extern crate cluMultThreading;

use cluMultThreading::mult_core::default::MultDefault;
use cluMultThreading::mult_core_task::run::function::TaskFn;
use cluMultThreading::mult_core_task::run::wait::WaitTask;

#[allow(deprecated)]
pub fn main() {
     init_clulog!();

     let drop = cluMultThreading::mul_core_behavior::portion::PortionCore::root();
     {
          let w = WaitTask::wait(&drop, TaskFn::new(
               || {
                    trace!("Start 1");
                    ::std::thread::sleep_ms(1000);

                    trace!("End 1");
                    flush!();
               }
          )).unwrap();
          let w2 = WaitTask::wait(drop, TaskFn::new(
               || {
                    trace!("Start 2");
                    ::std::thread::sleep_ms(800);

                    trace!("End 2");
                    flush!();
               }
          )).unwrap();

          trace!("Ok!");
          trace!("Ok2!");
          flush!();

          w.wait();
          w2.wait();
     }
}
