
use std::sync::mpsc::SyncSender;
use mcore_behavior::portion::comm::CommPartion;
use mcore::ErrDelThread;
use mcore_behavior::portion::ArcPortionCore;
use mcore::ErrAddThread;
use std::sync::Arc;
use std::time::Duration;
use std::sync::atomic::Ordering;



#[derive(Debug)]
pub struct PortionThreadManager {
     killer: SyncSender<usize>,

     //all_threads: usize,
     //active_threads: usize,
     
     //count_threads: usize,
     flow_queue: usize,

     //all_success: usize,
}


impl PortionThreadManager {
     pub fn empty(flow_queue: usize, kill: SyncSender<usize>) -> Self {
          let portion = Self {
               killer: kill,

               //all_threads: 0,
               
               //count_threads: 0,
               flow_queue: flow_queue,

               //all_success: 0,
          };
          /*if c > 0 {
               let _e = portion.add_thread(c, core);
          }*/
          
          portion
     }

     #[inline]
     pub fn add_thread(&mut self, c: usize, core: &Arc<ArcPortionCore>) -> Result<usize, ErrAddThread> {
          inf!("Portion: AddThread {}", c);
          if c > 0 {
               return Ok( self._add_thread(c, core, false) );
          }

          Err( ErrAddThread::Empty(c) )
     }
     #[inline]
     pub fn del_thread(&mut self, c: usize, core: &Arc<ArcPortionCore>) -> Result<usize, ErrDelThread> {
          inf!("Portion: DelThread {}", c);
          if c <= core.count_threads.load(Ordering::SeqCst) {
               return Ok( self._del_thread(c, core) );
          }

          Err( ErrDelThread::Empty(c) )
     }

     fn _del_thread(&mut self, c: usize, core: &Arc<ArcPortionCore>) -> usize {
          //self.count_threads -= c;
          let lock = match core.send.lock() {
               Ok(a) => a,
               Err(e) => e.into_inner(),
          };
          for _a in 1..=c {
               //println!("1");
               if let Err(_e) = lock.send(CommPartion::Kill) {
                    break;
               }
          }
          //println!("End. c:{}, {}", c, core.count_threads.load(Ordering::SeqCst));
          c
     }


     fn _add_thread(&mut self, c: usize, core: &Arc<ArcPortionCore>, is_additional: bool) -> usize {
          core.count_threads.fetch_add(c, Ordering::SeqCst);
          core.all_count_threads.fetch_add(c, Ordering::SeqCst);
          //self.all_threads += c;
          
          for mut num in 1..=c {
               //num += self.count_threads;


               inf!("Portion: Start thread, #{}", num);
               let flow_queue = self.flow_queue;
               //let active = core.active_threads.clone();
               ::std::thread::spawn(enclose!((core => core) move || {
                    //let _num = num;
                    let mut success = 0;
                    let mut flow_queue = flow_queue;
                    

                    {
                         let mut arrel: Vec< CommPartion > = Vec::with_capacity(flow_queue);

                         'l: loop {
                              {
                                   core.waiting_threads.fetch_add(1, Ordering::SeqCst);
                                   let lock = match core.recv.lock() {
                                        Ok(a) => a,
                                        Err(e) => e.into_inner()
                                   };

                                   for _a in 0..flow_queue {
                                        if let Ok(a) = lock.try_recv() {
                                             arrel.push(a);
                                             continue;
                                        }
                                        break;
                                   }
                                   if arrel.len() == 0 {
                                        if is_additional {
                                             break 'l;
                                        }
                                        //break 'l;
                                        

                                        match lock.recv_timeout(Duration::from_millis(400)) {
                                             Ok(a) => arrel.push(a),
                                             Err(_e) => {
                                                  core.waiting_threads.fetch_sub(1, Ordering::SeqCst);
                                                  break 'l;
                                             },
                                        }
                                        if flow_queue > 1 {
                                             let f = flow_queue-1;
                                             for _a in 0..f {
                                                  if let Ok(a) = lock.try_recv() {
                                                       arrel.push(a);
                                                       continue;
                                                  }
                                                  break;
                                             }
                                        }
                                        
                                   }
                                   core.waiting_threads.fetch_sub(1, Ordering::SeqCst);
                              }
                              {
                                   core.active_threads.fetch_add(1, Ordering::SeqCst);

                                   let waiting_threads = core.waiting_threads.load(Ordering::SeqCst);
                                   if waiting_threads == 0 {
                                        let count_threads = core.count_threads.load(Ordering::SeqCst);
                                        let active_threads = core.active_threads.load(Ordering::SeqCst);

                                        if count_threads == active_threads {
                                             let mut thread_manager = match core.thread_manager.lock() {
                                                  Ok(a) => a,
                                                  Err(e) => e.into_inner(),
                                             };
                                             thread_manager._add_thread(1, &core, true);
                                        }
                                   }
                              }
                              if Self::run_task(&mut success, &mut flow_queue, &mut arrel) {
                                   break 'l;
                              }

                              
                              {
                                   core.active_threads.fetch_sub(1, Ordering::SeqCst);
                              }
                         };

                         if arrel.len() > 0 {
                              let lock = match core.send.lock() {
                                   Ok(a) => a,
                                   Err(e) => e.into_inner(),
                              };
                              let _e = lock.send(CommPartion::TransferQueue(arrel));

                              if core.count_threads.load(Ordering::SeqCst) == 1 {
                                   let mut thread_manager = match core.thread_manager.lock() {
                                        Ok(a) => a,
                                        Err(e) => e.into_inner(),
                                   };
                                   thread_manager._add_thread(1, &core, true);
                              }
                         }
                         {
                              
                              //thread_manager.all_success += success;
                              //thread_manager.count_threads -= 1;
                              //thread_manager.active_threads -= 1;

                              core.count_threads.fetch_sub(1, Ordering::SeqCst);
                         }
                         
                    }
                    inf!("Portion: EndThread #{}", num);
                    let thread_manager = match core.thread_manager.lock() {
                         Ok(a) => a,
                         Err(e) => e.into_inner(),
                    };
                    if let Err(e) = thread_manager.killer.send(success) {
                         err!("Portion Thread: Unable to send stream close information. {:?}", e);
                         return;
                    }
                    
               }));
               //self.vec_threads.push(join);
               
          }
          //std::thread::park();
          //barrier.wait();

          c
     }

     #[inline(always)]
     fn run_task(success: &mut usize, flow_queue: &mut usize, arrel: &mut Vec<CommPartion>) -> bool {
          while let Some(recv) = arrel.pop() {
               match recv {
                    CommPartion::Task(mut a) => {
                         *success += 1;
                         a.run();
                    },
                    CommPartion::Kill => {
                         return true;
                    },
                    CommPartion::TransferQueue(mut vec) => {
                         if Self::run_task(success, flow_queue, &mut vec) {
                              return true;
                         }
                    },
                    CommPartion::TransferTask(mut a) => {
                         while let Some(mut a) = a.pop() {
                              *success += 1;
                              a.run();
                         }
                    },

                    CommPartion::UpFlowQueue(a) => {
                         *flow_queue = a;
                    }
               }
          }
          false
     }
}
