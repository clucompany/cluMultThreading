

use std::marker::PhantomData;
use mcore_behavior::portion::wait_atomic::WaitAtomic;
use std::sync::Barrier;
use mcore_behavior::portion::comm::CommPartion;
use mcore_behavior::portion::th::thread_feedback::EndThreadInfo;
use std::sync::Arc;
use mcore_behavior::portion::ArcPortionCore;
use mcore_behavior::portion::th::thread_status::ThreadStatus;
use std::time::Duration;


#[derive(Debug)]
pub struct PortionThread<'a> {
     _wait: WaitAtomic<'a>,
     num: usize,
     status: ThreadStatus,

     success: usize,
     flow_queue: usize,

     turn: Vec< CommPartion >,
     core: &'a Arc<ArcPortionCore>,
}

impl<'a> PortionThread<'a> {
     pub fn new(num: usize, status: ThreadStatus, core: &'a Arc<ArcPortionCore>, flow_queue: usize, wait_thread: WaitAtomic<'a>) -> Self {
          Self {
               _wait: wait_thread,
               num: num,
               status: status,

               success: 0,
               flow_queue: flow_queue,

               turn: Vec::with_capacity(flow_queue),
               core: core,
          }
     }
}


impl<'a> Iterator for PortionThread<'a> {
     type Item = IterEnd;
     fn next(&mut self) -> Option<Self::Item> {
          let mut is_kill = None;
          
          {
               let wait = self.core._add_wait_threads();
               let recv = self.core._lock_recv();
               for _a in 0..self.flow_queue {
                    if let Ok(a) = recv.try_recv() {
                         self.turn.push(a);
                         continue;
                    }
                    break;
               }
               if self.turn.len() == 0 {
                    match self.status {
                         ThreadStatus::Master => {
                              match recv.recv() {
                                   Ok(a) => {
                                        self.turn.push(a);
                                   },
                                   Err(e) => {
                                        trace!("Unknown Beh. {:?}", e);
                                        return Some(IterEnd::Allow);
                                   },
                              }
                         },
                         ThreadStatus::Hand => {
                              match recv.recv_timeout(Duration::from_secs(3)) {
                                   Ok(a) => {
                                        self.turn.push(a);
                                   },
                                   Err(e) => {
                                        trace!("Unknown Beh. {:?}", e);
                                        return Some(IterEnd::Allow);
                                   },
                              }
                         },
                         ThreadStatus::Power => {
                              match recv.recv_timeout(Duration::from_secs(1)) {
                                   Ok(a) => {
                                        self.turn.push(a);
                                   },
                                   Err(e) => {
                                        trace!("Unknown Beh. {:?}", e);
                                        return Some(IterEnd::Allow);
                                   },
                              }
                         },
                    }

                    match self.flow_queue {
                         0 => {

                         },
                         1 => {
                              
                         },
                         _ => {
                              let f = self.flow_queue - 1;
                              for _a in 0..f {
                                   match recv.try_recv() {
                                        Ok(a) => {
                                             self.turn.push(a);
                                             continue;
                                        },
                                        Err(e) => {
                                             trace!("Unknown Beh. {:?}", e);
                                             is_kill = Some(IterEnd::Allow);
                                             break;
                                        },
                                   }
                              }
                         },
                    }
               }
               drop( wait );
          }

          let wait_active = self.core._add_active_threads();
          {
               let waiting_threads = self.core._get_wait_threads();
               if waiting_threads == 0 {
                    let count_threads = self.core._get_count_threads();
                    let active_threads = self.core._get_active_threads();

                    if count_threads == active_threads {
                         let mut thread_manager = self.core._lock_thread_manager();
                         thread_manager._async_add_thread(1, &self.core, ThreadStatus::Power);
                    }
               }
          }

          

          while let Some(recv) = self.turn.pop() {
               match recv {
                    CommPartion::Task(mut a) => {
                         self.success += 1;
                         a.run();
                    },
                    CommPartion::Kill => {
                         match is_kill {
                              None => is_kill = Some(IterEnd::Allow),
                              _ => {
                                   let lock_send = self.core._lock_send();
                                   let _e = lock_send.send(CommPartion::Kill);
                              },
                         }
                    },
                    CommPartion::WaitKill(w) => {
                         match is_kill {
                              None => is_kill = Some(IterEnd::Barrier(w)),
                              _ => {
                                   let lock_send = self.core._lock_send();
                                   let _e = lock_send.send(CommPartion::WaitKill(w));
                              },
                         }
                    },
                    CommPartion::TransferQueue(mut vec) => {
                         self.turn.append(&mut vec);
                    },
                    CommPartion::TransferTask(mut a) => {
                         while let Some(mut a) = a.pop() {
                              self.success += 1;
                              a.run();
                         }
                    },

                    CommPartion::UpFlowQueue(a) => {
                         self.flow_queue = a;
                    }
               }
          }


          drop(wait_active);

          trace!("{:?}", is_kill);
          is_kill
     }
}


impl<'a> Drop for PortionThread<'a> {
     fn drop(&mut self) {
          trace!("Kill");
          let end: EndThreadInfo = self.into();

          {
               let lock_end = self.core._lock_killer();
               let _e = lock_end.send(end);
          }
     }
}



impl<'a, 'l> Into<EndThreadInfo> for &'a mut PortionThread<'l> {
     #[inline(always)]
     fn into(self) -> EndThreadInfo {
          EndThreadInfo::new(self.num, self.success)
     }
}


#[derive(Debug)]
pub enum IterEnd {
     Allow,
     Barrier(Arc<Barrier>),
}