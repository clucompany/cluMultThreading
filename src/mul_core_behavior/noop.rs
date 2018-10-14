

use std::sync::MutexGuard;
use mult_core::task::ErrAddDistrib;
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
use std::sync::Barrier;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;


const DEF_THREAD_LEN: usize = 1;
const MIN_THREAD_LEN: usize = 1;


#[derive(Debug)]
pub struct PortionCore {
     
     count_threads: Arc<AtomicUsize>,   

     //Отправка заданий
	send: Mutex<Sender<   Task >>,

	//Получение заданий
	recv: Mutex<Receiver< Task >>, 
}

impl PortionCore {
     fn _add_thread(&self, count_threads: usize) -> Result<usize, ErrAddThread> {
          let barrier = Arc::new(Barrier::new(count_threads));
          
          for num_thread in 0..count_threads {
               println!("{}", num_thread);


               
          }

          /*for mut self_num in 0..count_threads {
			self_num += *count_lock;
			println!("{}", num_thread);

          }*/

          let threads = self.count_threads.fetch_add(count_threads, Ordering::SeqCst);
          inf!("Threads {}->{}", threads, count_threads);

          Ok( count_threads )
     }
     #[inline(always)]
     fn _lock_send<'l>(&'l self) -> MutexGuard<'l, Sender<Task>> {
          match self.send.lock() {
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
          Self {
               send: Mutex::new(sender),
               recv: Mutex::new(receiver),

               count_threads: Arc::new(AtomicUsize::new(c))
          }
     }
     
}






impl MultStat for PortionCore {
     fn count_threads(&self) -> usize {
          0
     }
}


impl MultThreadManager for PortionCore {
     #[inline]
     fn add_thread(self, count_threads: usize) -> Result<usize, ErrAddThread> {
          inf!("Runnable add threads {}", count_threads);
		self._add_thread(count_threads)
     }
     #[inline]
     fn del_thread(&self, count_threads: usize) -> Result<usize, ErrDelThread> {
          Ok( count_threads )
     }
     #[inline]
     fn set_count_thread(&self, new_count: usize) -> Result<SetCountResult, ErrSetCount> {
          let lock_count = &0;
		inf!("Connector threads {}->{}", lock_count, new_count);

		if *lock_count == new_count {
			warn!("Runnable, unknown set_count_threads count, len == count");
			return Ok( SetCountResult::None(new_count) );
		}
		
		if *lock_count > new_count {
			let ncount = (*lock_count)-new_count;

               match self.del_thread(new_count) {
                    Err(e) => return Err(ErrSetCount::ErrDelThread(e)),
                    Ok(a) => return Ok(SetCountResult::Del(a)),
               }
		}
          match self.add_thread(new_count) {
               Err(e) => return Err(ErrSetCount::ErrAddThread(e)),
               Ok(a) => return Ok(SetCountResult::Add(a)),
          }
		//Ok( SetCountResult::AddThread( self.add_thread(new_count) ) )
          //Err( ErrSetCount::ErrMinThreads{ new_count: new_count, min_threads: 0 } )
     }
}


impl MultTaskManager for PortionCore {
     #[inline(always)]
     fn task(&self, e: Task) -> Result<(), ErrAddDistrib> {
          if let Err(e) = self._lock_send().send(e) {
               return Err( ErrAddDistrib::NotReady(e.0) );
          }

          Ok( () )
     }
}



impl MultDefault for PortionCore {}





