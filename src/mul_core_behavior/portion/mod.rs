
pub mod thmanager;
pub mod comm;

use std::sync::mpsc::sync_channel;
use mul_core_behavior::portion::comm::CommPartion;
use mult_core::MultExtend;
use mult_core::destruct::MultDestruct;
use mult_core::MultStatic;
use mul_core_behavior::portion::thmanager::PortionThreadManager;
use std::sync::MutexGuard;
use mult_core::task::ErrAddTask;
use mult_core::task::MultTaskManager;
use std::sync::mpsc::channel;
use mult_core_task::Task;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use mult_core::stat::MultStat;
use mult_core::thread::ErrSetCount;
use mult_core::thread::SetCountResult;
use mult_core::thread::ErrDelThread;
use mult_core::thread::MultThreadManager;
use mult_core::thread::ErrAddThread;
use mult_core::default::MultDefault;
use mult_core::default::MultRawDefault;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicUsize;


const MIN_THREAD_LEN: usize = 1;
const KILL_TURN_SEND_MESS: usize = 3;
//0 - default,


#[derive(Debug)]
pub struct PortionCore(Arc<ArcPortionCore>, Receiver<usize>);

#[derive(Debug)]
pub struct ArcPortionCore {
     thread_manager: Mutex<PortionThreadManager>,
     
     active_threads: AtomicUsize,
     all_count_threads: AtomicUsize,
     count_threads: AtomicUsize,
     waiting_threads: AtomicUsize,


     //Отправка заданий
	send: Mutex<Sender<   CommPartion >>,

	//Получение заданий
	recv: Mutex<Receiver< CommPartion >>, 
}


impl PortionCore {
     #[inline(always)]
     fn _lock_send<'l>(&'l self) -> MutexGuard<'l, Sender<CommPartion>> {
          match self.0.send.lock() {
               Ok(a) => a,
               Err(e) => e.into_inner(),
          }
     }
     #[inline(always)]
     fn _lock_thread_manager<'l>(&'l self) -> MutexGuard<'l, PortionThreadManager> {
          match self.0.thread_manager.lock() {
               Ok(a) => a,
               Err(e) => e.into_inner(),
          }
     }
}

impl MultRawDefault for PortionCore {
     #[inline]
     fn new() -> Self {
          Self::thread(MIN_THREAD_LEN)
     }

     fn thread(c: usize) -> Self {
          let (kill_sender, kill_receiver) = sync_channel(KILL_TURN_SEND_MESS);
          let (sender, receiver) = channel();

          let thread_manager = PortionThreadManager::empty(10, kill_sender);

          let arc_portion = Arc::new(
               ArcPortionCore {
                    thread_manager: Mutex::new(thread_manager),
                    active_threads: AtomicUsize::new(0),
                    all_count_threads: AtomicUsize::new(0),
                    count_threads: AtomicUsize::new(0),
                    waiting_threads: AtomicUsize::new(0),

                    send: Mutex::new(sender),

                    recv: Mutex::new(receiver),
               }
          );
          {
               let mut lock_thread_manager = match arc_portion.thread_manager.lock() {
                    Ok(a) => a,
                    Err(e) => e.into_inner(),
               };
               
               let _e = lock_thread_manager.add_thread(c, &arc_portion);
          }
          
          PortionCore(arc_portion, kill_receiver)
     }
     
}






impl MultStat for PortionCore {
     #[inline]
     fn count_threads(&self) -> usize {
          //*self._lock_thread_manager().as_count_threads()
          self.0.count_threads.load(Ordering::SeqCst)
     }
}


impl MultThreadManager for PortionCore {
     #[inline]
     fn add_thread(&self, count_threads: usize) -> Result<usize, ErrAddThread> {
          //inf!("Portion: AddThread {}", count_threads);
		self._lock_thread_manager().add_thread(count_threads, &self.0)
     }
     #[inline]
     fn del_thread(&self, count_threads: usize) -> Result<usize, ErrDelThread> {
          //inf!("Portion: DelThread {}", count_threads);
		self._lock_thread_manager().del_thread(count_threads, &self.0)
     }
     #[inline]
     fn set_count_thread(&self, new_count: usize) -> Result<SetCountResult, ErrSetCount> {
          let mut thread_manager = self._lock_thread_manager();
          let threads = self.count_threads();
          {
               //let threads = thread_manager.as_count_threads();
               
               inf!("Portion: SetThreads {}->{}", threads, new_count);

               if threads == new_count {
                    warn!("Runnable, unknown set_count_threads count, len == count");
                    return Ok( SetCountResult::None(new_count) );
               }
               
               if threads > new_count {
                    //let ncount = (threads)-new_count;

                    match thread_manager.del_thread(threads-new_count, &self.0) {
                         Err(e) => return Err(ErrSetCount::ErrDelThread(e)),
                         Ok(a) => return Ok(SetCountResult::Del(a)),
                    }
               }
          }

          match thread_manager.add_thread(new_count-threads, &self.0) {
               Err(e) => return Err(ErrSetCount::ErrAddThread(e)),
               Ok(a) => return Ok(SetCountResult::Add(a)),
          }
     }
}


impl MultTaskManager for PortionCore {
     #[inline(always)]
     fn task(&self, e: Task) -> Result<(), ErrAddTask> {
          let lock_send = self._lock_send();
          
          /*{
               let waiting_threads = self.0.waiting_threads.load(Ordering::SeqCst);
               if waiting_threads == 0 {
                    //inf!("Lock threadm, count: {}, active: {}", self.count_threads(), self.0.active_threads.load(Ordering::Relaxed));
                    let count_threads = self.count_threads();
                    match count_threads {
                         0 => {
                              let mut thread_manager = self._lock_thread_manager();
                              thread_manager.add_thread(1, &self.0);
                         },
                         _ => {
                              let active = self.0.active_threads.load(Ordering::SeqCst);
                              trace!("{:?}, count:{}, active:{}, wait:{}", e, count_threads, active, waiting_threads);
                              if count_threads == active {
                                   let mut thread_manager = self._lock_thread_manager();
                                   thread_manager.add_thread(1, &self.0);
                              }
                         },
                    }
               }else {
                    trace!("{:?}, wait:{}", e, waiting_threads);
               }
          }*/
          if let Err(e) = lock_send.send(CommPartion::Task(e)) {
               if let CommPartion::Task(e) = e.0 {
                    return Err( ErrAddTask::NotReady(e) );
               }
          }
          
          

          Ok( () )
     }

     fn task_array(&self, arr: Vec<Task>) -> Result<(), ErrAddTask> {
          let lock_send = self._lock_send();
          if let Err(e) = lock_send.send(CommPartion::TransferTask(arr)) {
               if let CommPartion::Task(e) = e.0 {
                    return Err( ErrAddTask::NotReady(e) );
               }
          }

          Ok( () )
     }
}

impl MultDestruct for PortionCore {
     fn destruct(&self) {
          inf!("Portion: Destruct");

          let mut success = 0;
          let mut del_threads = 0;
          {
               let mut threads;
               loop {
                    threads = self.count_threads();
                    if threads == 0 {
                         break;
                    }
                    del_threads += threads;
                    while threads > 0 {
                         match self.1.recv() {
                              Ok(succ) => success += succ,
                              Err(e) => {
                                   err!("Portion: Failed to get channel for reporting closed threads. {:?}", e);
                                   ::std::thread::sleep_ms(4000);
                                   return;
                              },
                         };
                         
                         threads -= 1;
                         


                    }
               }
          }

          //let mut err_del_threads = 0;
          /*{
               let mut join;
               loop {
                    {
                         let mut lock_thread_manager = self._lock_thread_manager();
                         
                         match lock_thread_manager.vec_threads.pop() {
                              None => break,
                              Some(th) => join = th,
                         }
                    }
                    match join.join() {
                         Ok(a) => success += a,
                         _ => err_del_threads += 1,
                    }
                    del_threads += 1;
                    /*if threads == 0 {
                         break;
                    }
                    let barrier = Arc::new(Barrier::new(threads + 1));

                    {
                         let lock_send = self._lock_send();
                         for a in 0..threads {
                              lock_send.send(CommPartion::EndRoot(barrier.clone()));
                         }
                    }

                    barrier.wait();
                    println!("12");
                    del_threads += threads;*/
               }
          }*/
          
          /*{
               let mut threads;
               let mut is_one = false;
               loop {
                    {
                         let mut lock_thread_manager = self._lock_thread_manager();
                         threads = *lock_thread_manager.as_count_threads();    
                         //lock_thread_manager.flag_kills += threads;
                    }
                    match threads {
                         0 => break,
                         1 => {
                              if !is_one {
                                   let mut send = self._lock_send();
                                   let _e = send.send(CommPartion::EndKillThread(::std::thread::current()));
                                   is_one = true;
                              }else {
                                   let mut send = self._lock_send();
                                   let _e = send.send(CommPartion::KillThread);
                              }
                         },
                         _ => {
                              let mut send = self._lock_send();

                              if is_one == false {
                                   threads -= 1;
                                   for _a in 0..threads {
                                        let _e = send.send(CommPartion::KillThread);
                                   }
                                   let _e = send.send(CommPartion::EndKillThread(::std::thread::current()));

                                   is_one = true;
                              }else {
                                   for _a in 0..threads {
                                        let _e = send.send(CommPartion::KillThread);
                                   }
                              }
                         },
                    }
                    ::std::thread::park();
                    del_threads += threads;
               }
          }*/
          //let lock_thread_manager = self._lock_thread_manager();
          let all_count_threads = self.0.all_count_threads.load(Ordering::Relaxed);
          inf!("Portion: End. Threads {}, AllThreads {}, Success {}", 
               del_threads, 
               all_count_threads, 
               success
          );
     }
}


impl MultDefault for PortionCore {}
impl<'a> MultStatic<'a> for PortionCore {}
impl<'a> MultExtend<'a> for PortionCore {}


