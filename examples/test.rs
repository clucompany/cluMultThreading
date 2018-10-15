
#[macro_use]
extern crate cluLog;
extern crate cluMultThreading;


use cluMultThreading::mult_core::thread::MultThreadManager;
use cluMultThreading::mult_core::default::MultDefault;


pub fn main() {
     init_clulog!();
     
     let tasker = cluMultThreading::mul_core_behavior::portion::PortionCore::root();
     tasker.add_thread(1);
     tasker.set_count_thread(1);
     tasker.add_thread(1);
     //println!("{:?}", tasker.del_thread(9));
     
}
