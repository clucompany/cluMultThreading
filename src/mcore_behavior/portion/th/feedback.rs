

use mcore_behavior::portion::th::thread::PortionThread;
use std::sync::Arc;
use std::sync::Barrier;


#[derive(Debug)]
pub enum ThreadFeedBack {
     EndBarrier(Arc<Barrier>),
     EndAlways,
}


#[derive(Debug)]
pub struct EndThreadInfo {
     pub num: usize,
     pub success: usize,
}

impl EndThreadInfo {
     #[inline]
     pub const fn new(num: usize, success: usize) -> Self {
          Self {
               num: num,
               success: success,
          }
     }
}



