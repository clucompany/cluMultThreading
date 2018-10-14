
#[macro_use]
extern crate cluLog;
extern crate cluMultThreading;


use cluMultThreading::mult_core::default::MultDefault;


pub fn main() {
     init_clulog!();

     let drop = cluMultThreading::mul_core_behavior::portion::PortionCore::common_thread(10);

     
     //drop(12);
     
}
