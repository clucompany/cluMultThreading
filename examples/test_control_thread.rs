
#[macro_use]
extern crate cluLog;
extern crate cluMultThreading;


use cluMultThreading::mcore::MultStat;
use cluMultThreading::mcore::MultThreadManager;
use cluMultThreading::mcore::MultDefault;


pub fn main() {
     init_clulog!();
     
     let tasker = cluMultThreading::mcore_behavior::PortionCore::root();
     trace!("Def threads, {}", tasker.count_threads());
     tasker.add_thread(1).unwrap();
     tasker.set_count_thread(5).unwrap();
     tasker.del_thread(1).unwrap();
     trace!("All threads, {}", tasker.count_threads());
     tasker.set_count_thread(2).unwrap();
     trace!("All threads, {}", tasker.count_threads());
}
