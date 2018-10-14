
pub mod thmanager;

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


const MIN_THREAD_LEN: usize = 1;


#[derive(Debug)]
pub struct PortionCore(Arc<ArcPortionCore>);

#[derive(Debug)]
pub struct ArcPortionCore {
     thread_manager: Mutex<PortionThreadManager>,

     //Отправка заданий
	send: Mutex<Sender<   Task >>,

	//Получение заданий
	recv: Mutex<Receiver< Task >>, 
}


impl PortionCore {
     #[inline(always)]
     fn _lock_send<'l>(&'l self) -> MutexGuard<'l, Sender<Task>> {
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
          let (sender, receiver) = channel();

          let arc_portion = Arc::new(
               ArcPortionCore {
                    thread_manager: Mutex::new(unsafe{ std::mem::uninitialized() }),

                    send: Mutex::new(sender),

                    recv: Mutex::new(receiver),
               }
          );
          {
               let mut lock_thread_manager = match arc_portion.thread_manager.lock() {
                    Ok(a) => a,
                    Err(e) => e.into_inner(),
               };
               *lock_thread_manager = PortionThreadManager::thread(c, &arc_portion);
          }
          // ************ 
          // It is really safe! 
          // Since in the case of using thread_manager there 
          // is a lock from the inside while the manager is being replaced.
          
          PortionCore(arc_portion)
     }
     
}






impl MultStat for PortionCore {
     #[inline]
     fn count_threads(&self) -> usize {
          *self._lock_thread_manager().as_count_threads()
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

          {
               let threads = thread_manager.as_count_threads();
               inf!("Portion: SetThreads {}->{}", threads, new_count);

               if *threads == new_count {
                    warn!("Runnable, unknown set_count_threads count, len == count");
                    return Ok( SetCountResult::None(new_count) );
               }
               
               if *threads > new_count {
                    let ncount = (*threads)-new_count;

                    match self.del_thread(ncount) {
                         Err(e) => return Err(ErrSetCount::ErrDelThread(e)),
                         Ok(a) => return Ok(SetCountResult::Del(a)),
                    }
               }
          }

          match thread_manager.add_thread(new_count, &self.0) {
               Err(e) => return Err(ErrSetCount::ErrAddThread(e)),
               Ok(a) => return Ok(SetCountResult::Add(a)),
          }
		//Ok( SetCountResult::AddThread( self.add_thread(new_count) ) 
          //Ok( SetCountResult::Add(1) )
     }
}


impl MultTaskManager for PortionCore {
     #[inline(always)]
     fn task(&self, e: Task) -> Result<(), ErrAddTask> {
          if let Err(e) = self._lock_send().send(e) {
               return Err( ErrAddTask::NotReady(e.0) );
          }

          Ok( () )
     }
}

impl MultDestruct for PortionCore {
     fn destruct(&self) {
          inf!("Portion: Destruct");
     }
}


impl MultDefault for PortionCore {}
impl<'a> MultStatic<'a> for PortionCore {}
impl<'a> MultExtend<'a> for PortionCore {}




