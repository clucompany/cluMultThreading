
#[macro_use]
extern crate cluLog;
extern crate cluMultThreading;

use cluMultThreading::mcore::MultDefault;
use cluMultThreading::mcore_task::FnTask;
use cluMultThreading::mcore_task::WaitTask;

#[allow(deprecated)]
pub fn main() {
     init_clulog!();

     let drop = cluMultThreading::mcore_behavior::portion::PortionCore::root();
     {
          let w = WaitTask::wait(&drop, FnTask::new(
               || {
                    trace!("Start 1");
                    ::std::thread::sleep_ms(1000);

                    trace!("End 1");
                    flush!();
               }
          )).unwrap();
          let w2 = WaitTask::wait(drop, FnTask::new(
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
