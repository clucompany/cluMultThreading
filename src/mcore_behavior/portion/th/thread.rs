

use mcore_behavior::portion::comm::CommPartion;
use mcore_behavior::portion::th::feedback::EndThreadInfo;
use mcore_behavior::portion::th::feedback::ThreadFeedBack;
use std::sync::Arc;
use mcore_behavior::portion::ArcPortionCore;
use mcore_behavior::portion::th::thread_status::ThreadStatus;



#[derive(Debug)]
pub struct PortionThread {
     num: usize,
     status: ThreadStatus,

     succes: usize,
     flow_queue: usize,

     turn: Vec< CommPartion >,
     core: Arc<ArcPortionCore>,
}

impl PortionThread {
     pub fn new(num: usize, status: ThreadStatus, core: Arc<ArcPortionCore>, flow_queue: usize) -> Self {
          Self {
               num: num,
               status: status,

               succes: 0,
               flow_queue: flow_queue,

               turn: Vec::with_capacity(5),
               core: core,
          }
     }
}


impl Iterator for PortionThread {
     type Item = ();
     fn next(&mut self) -> Option<Self::Item> {
          let wait = self.core._add_wait_threads();
          let recv = self.core._lock_recv();

          for _a in 0..flow_queue {
               if let Ok(a) = recv.try_recv() {
                    arrel.push(a);
                    continue;
               }
               break;
          }

          
          None
     }
}


impl Drop for PortionThread {
     fn drop(&mut self) {
          let end: EndThreadInfo = self.into();

          {
               let lock_end = self.core._lock_killer();
               let _e = lock_end.send(end);
          }
     }
}



impl<'a> Into<EndThreadInfo> for &'a mut PortionThread {
     #[inline(always)]
     fn into(self) -> EndThreadInfo {
          EndThreadInfo::new(self.num, self.succes)
     }
}
