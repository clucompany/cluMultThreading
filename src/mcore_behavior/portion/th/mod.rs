

pub mod thread;
pub mod thread_feedback;
pub mod thread_status;



use std::sync::Barrier;
use mcore_behavior::portion::th::thread_status::ThreadStatus;
use mcore_behavior::portion::comm::CommPartion;
use mcore::ErrDelThread;
use mcore_behavior::portion::ArcPortionCore;
use mcore::ErrAddThread;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use self::thread::PortionThread;
use self::thread::IterEnd;

#[derive(Debug)]
pub struct PortionThreadManager {
     flow_queue: usize,
}


impl PortionThreadManager {
     pub fn empty(flow_queue: usize) -> Self {
          let portion = Self {
               flow_queue: flow_queue,
          };
          /*if c > 0 {
               let _e = portion.add_thread(c, core);
          }*/
          
          portion
     }

     #[inline]
     pub fn add_thread(&mut self, c: usize, core: &Arc<ArcPortionCore>) -> Result<usize, ErrAddThread> {
          inf!("SyncAddThread {}", c);
          if c > 0 {
               return Ok( self._add_thread(c, core, ThreadStatus::Hand) );
          }

          Err( ErrAddThread::Empty(c) )
     }
     #[inline]
     pub fn del_thread(&mut self, c: usize, core: &Arc<ArcPortionCore>) -> Result<usize, ErrDelThread> {
          inf!("SyncDelThread {}", c);
          if c < core.count_threads.load(Ordering::SeqCst) {
               return Ok( self._del_thread(c, core) );
          }

          Err( ErrDelThread::Empty(c) )
     }

     #[inline]
     pub fn async_add_thread(&mut self, c: usize, core: &Arc<ArcPortionCore>) -> Result<usize, ErrAddThread> {
          inf!("AsyncAddThread {}", c);
          if c > 0 {
               return Ok( self._async_add_thread(c, core, ThreadStatus::Hand) );
          }

          Err( ErrAddThread::Empty(c) )
     }
     #[inline]
     pub fn async_del_thread(&mut self, c: usize, core: &Arc<ArcPortionCore>) -> Result<usize, ErrDelThread> {
          inf!("AsyncDelThread {}", c);
          if c < core.count_threads.load(Ordering::SeqCst) {
               return Ok( self._async_del_thread(c, core) );
          }

          Err( ErrDelThread::Empty(c) )
     }

     fn _async_del_thread(&mut self, c: usize, core: &Arc<ArcPortionCore>) -> usize {
          let lock_send = core._lock_send();
          for _a in 1..=c {
               if let Err(_e) = lock_send.send(CommPartion::Kill) {
                    break;
               }
          }
          c
     }
     fn _del_thread(&mut self, c: usize, core: &Arc<ArcPortionCore>) -> usize {
          let barrier = Arc::new(Barrier::new(c+1));
          {
               let lock_send = core._lock_send();
               for n in 1..=c {
                    if let Err(_e) = lock_send.send(CommPartion::WaitKill(barrier.clone())) {
                         return n;
                    }
               }
          }

          barrier.wait();

          c
     }


     pub fn _async_add_thread<'a, 'l>(&mut self, c: usize, core: &'a Arc<ArcPortionCore>, status: ThreadStatus) -> usize {
          core.count_threads.fetch_add(c, Ordering::SeqCst);
          core.all_count_threads.fetch_add(c, Ordering::SeqCst);

          for mut num in 1..=c {
               inf!("AStart thread, #{}, is_additional:{:?}", num, status);
               let flow_queue = self.flow_queue;

               ::std::thread::spawn( enclose!((core, status, flow_queue) move || {
                    let mut thread = PortionThread::new(num, status, &core, flow_queue, core._add_count_threads());

                    let mut a;
                    loop {
                         a = thread.next();
                         match a {
                              None => {},
                              Some(IterEnd::Allow) => break,
                              Some(IterEnd::Barrier(b)) => {
                                   b.wait();
                                   break;
                              },
                         }
                    }
               }));            
          }
          c
     }

     pub fn _add_thread(&mut self, c: usize, core: &Arc<ArcPortionCore>, status: ThreadStatus) -> usize {
          core.count_threads.fetch_add(c, Ordering::SeqCst);
          core.all_count_threads.fetch_add(c, Ordering::SeqCst);

          let barrier = Arc::new(Barrier::new(c+1));

          for mut num in 1..=c {
               let status = status.clone();

               inf!("WStart thread, #{}, is_additional:{:?}", num, status);
               let flow_queue = self.flow_queue;
               
               ::std::thread::spawn( enclose!((core, barrier, status) move || {
                    let mut thread = PortionThread::new(num, status, &core, flow_queue, core._add_count_threads());
                    barrier.wait();
                    drop(barrier);


                    let mut a;
                    loop {
                         a = thread.next();
                         match a {
                              None => {},
                              Some(IterEnd::Allow) => break,
                              Some(IterEnd::Barrier(b)) => {
                                   b.wait();
                                   break;
                              },
                         }
                    }

                    
               }));               
          }
          barrier.wait();
          
          c
     }

     
}
