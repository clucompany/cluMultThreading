
#[macro_use]
extern crate cluLog;
extern crate cluMultThreading;


use cluMultThreading::mcore::MultThreadManager;
use cluMultThreading::mcore::MultDefault;


pub fn main() {
     init_clulog!();
     
     let tasker = cluMultThreading::mcore_behavior::portion::PortionCore::root();
     tasker.add_thread(1).unwrap();
     tasker.set_count_thread(5).unwrap();
     tasker.del_thread(1).unwrap();
     
}
