
mod comm;
mod th;
mod wait_atomic;

use mcore_behavior::portion::th::thread_feedback::EndThreadInfo;
use std::sync::mpsc::SyncSender;
use mcore_behavior::portion::wait_atomic::WaitAtomic;
use mcore_behavior::portion::th::PortionThreadManager;
use std::sync::mpsc::sync_channel;
use mcore_behavior::portion::comm::CommPartion;
use mcore::MultExtend;
use mcore::MultDestruct;
use mcore::MultStatic;
use std::sync::MutexGuard;
use mcore::ErrAddTask;
use mcore::MultTaskManager;
use std::sync::mpsc::channel;
use mcore_task::Task;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use mcore::MultStat;
use mcore::ErrDelThread;
use mcore::MultThreadManager;
use mcore::ErrAddThread;
use mcore::MultDefault;
use mcore::MultRawDefault;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicUsize;
use self::th::thread_status::ThreadStatus;


const MIN_THREAD_LEN: usize = 1;
const KILL_TURN_SEND_MESS: usize = 3;
//0 - default,


#[derive(Debug)]
pub struct PortionCore(Arc<ArcPortionCore>, Receiver<EndThreadInfo>);

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


     killer: Mutex<SyncSender<EndThreadInfo>>,
}


impl PortionCore {
     

     
}

impl ArcPortionCore {
     #[inline(always)]
     pub (crate) fn _add_count_threads<'a>(&'a self) -> WaitAtomic<'a> {
          WaitAtomic::new(&self.count_threads, Ordering::SeqCst)
     }
     #[inline(always)]
     pub (crate) fn _get_count_threads<'a>(&'a self) -> usize {
          self.count_threads.load(Ordering::SeqCst)
     }

     #[inline(always)]
     pub (crate) fn _add_count_threads_no_init<'a>(&'a self) -> WaitAtomic<'a> {
          WaitAtomic::new(&self.count_threads, Ordering::SeqCst)
     }

     #[inline(always)]
     pub (crate) fn _add_wait_threads<'a>(&'a self) -> WaitAtomic<'a> {
          WaitAtomic::new(&self.waiting_threads, Ordering::SeqCst)
     }

     #[inline(always)]
     pub (crate) fn _get_wait_threads<'a>(&'a self) -> usize {
          self.waiting_threads.load(Ordering::SeqCst)
     }

     #[inline(always)]
     pub (crate) fn _add_active_threads<'a>(&'a self) -> WaitAtomic<'a> {
          WaitAtomic::new(&self.active_threads, Ordering::SeqCst)
     }

     #[inline(always)]
     pub (crate) fn _get_active_threads<'a>(&'a self) -> usize {
          self.active_threads.load(Ordering::SeqCst)
     }


     #[inline(always)]
     pub (crate) fn _lock_send<'l>(&'l self) -> MutexGuard<'l, Sender<CommPartion>> {
          match self.send.lock() {
               Ok(a) => a,
               Err(e) => e.into_inner(),
          }
     }

     #[inline(always)]
     pub (crate) fn _lock_recv<'l>(&'l self) -> MutexGuard<'l, Receiver<CommPartion>> {
          match self.recv.lock() {
               Ok(a) => a,
               Err(e) => e.into_inner(),
          }
     }

     #[inline(always)]
     pub (crate) fn _lock_thread_manager<'l>(&'l self) -> MutexGuard<'l, PortionThreadManager> {
          match self.thread_manager.lock() {
               Ok(a) => a,
               Err(e) => e.into_inner(),
          }
     }

     #[inline(always)]
     pub (crate) fn _lock_killer<'l>(&'l self) -> MutexGuard<'l, SyncSender<EndThreadInfo>> {
          match self.killer.lock() {
               Ok(a) => a,
               Err(e) => e.into_inner(),
          }
     }
}


impl MultRawDefault for PortionCore {
     type NewRes = Self;

     #[inline]
     fn new() -> Self::NewRes {
          Self::thread(MIN_THREAD_LEN)
     }

     fn thread(c: usize) -> Self::NewRes {
          let (kill_sender, kill_receiver) = sync_channel(KILL_TURN_SEND_MESS);
          let (sender, receiver) = channel();

          let thread_manager = PortionThreadManager::empty(10);

          let arc_portion = Arc::new(
               ArcPortionCore {
                    thread_manager: Mutex::new(thread_manager),
                    active_threads: AtomicUsize::new(0),
                    all_count_threads: AtomicUsize::new(0),
                    count_threads: AtomicUsize::new(0),
                    waiting_threads: AtomicUsize::new(0),

                    send: Mutex::new(sender),

                    recv: Mutex::new(receiver),

                    killer: Mutex::new(kill_sender),
               }
          );
          if c > 0{
               inf!("AsyncStartAddThread {}", c);
               let mut lock_thread_manager = arc_portion._lock_thread_manager();
               let _e = lock_thread_manager._async_add_thread(c, &arc_portion, ThreadStatus::Master);
          }
          
          PortionCore(arc_portion, kill_receiver)
     }
     
}






impl MultStat for PortionCore {
     #[inline]
     fn count_threads(&self) -> usize {
          self.0.count_threads.load(Ordering::SeqCst)
     }
}


impl MultThreadManager for PortionCore {
     #[inline]
     fn add_thread(&self, count_threads: usize) -> Result<usize, ErrAddThread> {
          //inf!("Portion: AddThread {}", count_threads);
		self.0._lock_thread_manager().add_thread(count_threads, &self.0)
     }
     #[inline]
     fn del_thread(&self, count_threads: usize) -> Result<usize, ErrDelThread> {
          //inf!("Portion: DelThread {}", count_threads);
		self.0._lock_thread_manager().del_thread(count_threads, &self.0)
     }

     #[inline]
     fn async_add_thread(&self, count_threads: usize) -> Result<usize, ErrAddThread> {
          //inf!("Portion: AddThread {}", count_threads);
		self.0._lock_thread_manager().async_add_thread(count_threads, &self.0)
     }
     #[inline]
     fn async_del_thread(&self, count_threads: usize) -> Result<usize, ErrDelThread> {
          //inf!("Portion: DelThread {}", count_threads);
		self.0._lock_thread_manager().async_del_thread(count_threads, &self.0)
     }
}


impl MultTaskManager for PortionCore {
     #[inline(always)]
     fn task(&self, e: Task) -> Result<(), ErrAddTask> {
          let lock_send = self.0._lock_send();
          
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
          let lock_send = self.0._lock_send();
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
               'l: loop {
                    threads = self.count_threads();
                    trace!("{}", threads);
                    if threads == 0 {
                         break;
                    }
                    del_threads += threads;
                    while threads > 0 {
                         match self.1.recv() {
                              Ok(end) => {
                                   println!("{:?}, threads: {}", end, threads);
                                   success += end.success;
                              },
                              Err(e) => {
                                   err!("Portion: Failed to get channel for reporting closed threads. {:?}", e);
                                   
                                   //::std::thread::sleep_ms(4000);
                                   break 'l;
                              },
                         };
                         
                         threads -= 1;
                         


                    }
               }
          }

          let all_count_threads = self.0.all_count_threads.load(Ordering::Relaxed);
          inf!("Portion: End. Threads {}, AllThreads {}, Success {}", 
               del_threads, 
               all_count_threads, 
               success
          );
     }
}


impl<'a> MultDefault<Self> for PortionCore {}
impl<'a> MultStatic<'a> for PortionCore {}
impl<'a> MultExtend<'a> for PortionCore {}


