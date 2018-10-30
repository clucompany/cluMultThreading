

use mcore_behavior::portion::th::thread_status::ThreadStatus;
use mcore_behavior::portion::ArcPortionCore;
use std::sync::Arc;
use std::sync::Barrier;
use mcore_behavior::portion::comm::CommPartion;
use std::sync::atomic::Ordering;
use std::time::Duration;

pub fn thread_run(num: usize, flow_queue: usize, core: Arc<ArcPortionCore>, thread: ThreadStatus, wait_start: Option<Arc<Barrier>>) -> impl FnOnce() + 'static {
     move || {
     //let _num = num;
                    let mut success = 0;
                    let mut flow_queue = flow_queue;
                    let mut is_kill = IsKill::None;
                    let wait_count_threads = core._add_count_threads_no_init();

                    {
                         let mut arrel: Vec< CommPartion > = Vec::with_capacity(flow_queue);

                         if let Some(wait_start) = wait_start {
                              wait_start.wait();
                         }
                         //drop(wait_start);

                         'l: loop {
                              {
                                   //Safe thing
                                   let wait = core._add_wait_threads();
                                   let recv = core._lock_recv();

                                   for _a in 0..flow_queue {
                                        if let Ok(a) = recv.try_recv() {
                                             arrel.push(a);
                                             continue;
                                        }
                                        break;
                                   }
                                   if arrel.len() == 0 {
                                        match thread {
                                             ThreadStatus::Master => {
                                                  break 'l;
                                             },

                                             ThreadStatus::Hand => {
                                                  //break 'l;
                                             },

                                             ThreadStatus::Power => {
                                                  //break 'l;
                                             },
                                        }
                                        

                                        match recv.recv_timeout(Duration::from_millis(400)) {
                                             Ok(a) => arrel.push(a),
                                             Err(_e) => {
                                                  break 'l;
                                             },
                                        }
                                        if flow_queue > 1 {
                                             let f = flow_queue-1;
                                             for _a in 0..f {
                                                  if let Ok(a) = recv.try_recv() {
                                                       arrel.push(a);
                                                       continue;
                                                  }
                                                  break;
                                             }
                                        }
                                        
                                   }

                                   drop(wait);
                              }
                              let wait_active_thread = core._add_active_threads();
                              {
                                   let waiting_threads = core.waiting_threads.load(Ordering::SeqCst);
                                   if waiting_threads == 0 {
                                        let count_threads = core.count_threads.load(Ordering::SeqCst);
                                        let active_threads = core.active_threads.load(Ordering::SeqCst);

                                        if count_threads == active_threads {
                                             let mut thread_manager = match core.thread_manager.lock() {
                                                  Ok(a) => a,
                                                  Err(e) => e.into_inner(),
                                             };
                                             thread_manager._async_add_thread(1, &core, ThreadStatus::Power);
                                        }
                                   }
                              }

                              
                              while let Some(recv) = arrel.pop() {
                                   match recv {
                                        CommPartion::Task(mut a) => {
                                             success += 1;
                                             a.run();
                                        },
                                        CommPartion::Kill => {
                                             match is_kill {
                                                  IsKill::None => is_kill = IsKill::Allow,
                                                  _ => {
                                                       let lock_send = core._lock_send();
                                                       let _e = lock_send.send(CommPartion::Kill);
                                                  },
                                             }
                                        },
                                        CommPartion::WaitKill(w) => {
                                             match is_kill {
                                                  IsKill::None => is_kill = IsKill::Barrier(w),
                                                  _ => {
                                                       let lock_send = core._lock_send();
                                                       let _e = lock_send.send(CommPartion::WaitKill(w));
                                                  },
                                             }
                                        },
                                        CommPartion::TransferQueue(mut vec) => {
                                             arrel.append(&mut vec);
                                        },
                                        CommPartion::TransferTask(mut a) => {
                                             while let Some(mut a) = a.pop() {
                                                  success += 1;
                                                  a.run();
                                             }
                                        },

                                        CommPartion::UpFlowQueue(a) => {
                                             flow_queue = a;
                                        }
                                   }
                              }
                              drop(wait_active_thread);
                              match is_kill {
                                   IsKill::None => {},
                                   IsKill::Allow => break 'l,
                                   IsKill::Barrier(w) => {
                                        w.wait();
                                        break 'l;
                                   },
                              }
                         };

                         /*if arrel.len() > 0 {
                              {
                                   let lock_send = core._lock_send();
                                   let _e = lock_send.send(CommPartion::TransferQueue(arrel));
                              }

                              if core.count_threads.load(Ordering::SeqCst) == 1 {
                                   let mut thread_manager = match core.thread_manager.lock() {
                                        Ok(a) => a,
                                        Err(e) => e.into_inner(),
                                   };
                                   thread_manager._add_thread(1, &core, ThreadStatus::Power);
                              }
                         }*/
                         
                    }
                    inf!("Portion: EndThread #{}", num);
                    /*{
                         let lock_thread_manager = core._lock_thread_manager();
                         if let Err(e) = lock_thread_manager.killer.send(success) {
                              err!("Portion Thread: Unable to send stream close information. {:?}", e);
                              return;
                         }
                    }*/

                    drop(wait_count_threads);
     }

}


enum IsKill {
     None,

     Allow,
     Barrier(Arc<Barrier>),
}