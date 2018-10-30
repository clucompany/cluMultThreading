

use std::fmt::Debug;
use mcore::stat::MultStat;

pub trait MultThreadManager: MultStat + Debug {
	fn add_thread(&self, count_threads: usize) -> Result<usize, ErrAddThread>;
	fn del_thread(&self, count_threads: usize) -> Result<usize, ErrDelThread>;
	fn set_count_thread(&self, count_threads: usize) -> Result<SetCountResult, ErrSetCount> {
		//let mut thread_manager = self._lock_thread_manager();
          let threads = self.count_threads();
          {
               //let threads = thread_manager.as_count_threads();
               
               inf!("SetThreads {}->{}", threads, count_threads);

               if threads == count_threads {
                    warn!("Runnable, unknown set_count_threads count, len == count");
                    return Ok( SetCountResult::None(count_threads) );
               }
               
               if threads > count_threads {
                    //let ncount = (threads)-new_count;

                    inf!("SetThreads {}->{}, Del {}", threads, count_threads, threads-count_threads);
                    match self.del_thread(threads-count_threads) {
                         Err(e) => return Err(ErrSetCount::ErrDelThread(e)),
                         Ok(a) => return Ok(SetCountResult::Del(a)),
                    }
               }
          }

          inf!("SetThreads {}->{}, Add {}", threads, count_threads, count_threads-threads);
          match self.add_thread(1+(count_threads-threads)) {
               Err(e) => return Err(ErrSetCount::ErrAddThread(e)),
               Ok(a) => return Ok(SetCountResult::Add(a)),
          }
	}

	fn async_add_thread(&self, count_threads: usize) -> Result<usize, ErrAddThread>;
	fn async_del_thread(&self, count_threads: usize) -> Result<usize, ErrDelThread>;
	fn async_set_count_thread(&self, count_threads: usize) -> Result<SetCountResult, ErrSetCount>{
		//let mut thread_manager = self._lock_thread_manager();
          let threads = self.count_threads();
          {
               //let threads = thread_manager.as_count_threads();
               
               inf!("SetThreads {}->{}", threads, count_threads);

               if threads == count_threads {
                    warn!("Runnable, unknown set_count_threads count, len == count");
                    return Ok( SetCountResult::None(count_threads) );
               }
               
               if threads > count_threads {
                    //let ncount = (threads)-new_count;

                    inf!("SetThreads {}->{}, Del {}", threads, count_threads, threads-count_threads);
                    match self.async_del_thread(threads-count_threads) {
                         Err(e) => return Err(ErrSetCount::ErrDelThread(e)),
                         Ok(a) => return Ok(SetCountResult::Del(a)),
                    }
               }
          }

          inf!("Portion: SetThreads {}->{}, Add {}", threads, count_threads, count_threads-threads);
          match self.async_add_thread(1+(count_threads-threads)) {
               Err(e) => return Err(ErrSetCount::ErrAddThread(e)),
               Ok(a) => return Ok(SetCountResult::Add(a)),
          }
	}
}


impl<'a, A: MultThreadManager> MultThreadManager for &'a A {
	#[inline(always)]
	fn add_thread(&self, count_threads: usize) -> Result<usize, ErrAddThread> {
		(*self).add_thread(count_threads)
	}

	#[inline(always)]
	fn del_thread(&self, count_threads: usize) -> Result<usize, ErrDelThread> {
		(*self).del_thread(count_threads)
	}

	#[inline(always)]
	fn set_count_thread(&self, count_threads: usize) -> Result<SetCountResult, ErrSetCount> {
		(*self).set_count_thread(count_threads)
	}

	#[inline(always)]
	fn async_add_thread(&self, count_threads: usize) -> Result<usize, ErrAddThread> {
		(*self).async_add_thread(count_threads)
	}

	#[inline(always)]
	fn async_del_thread(&self, count_threads: usize) -> Result<usize, ErrDelThread> {
		(*self).async_del_thread(count_threads)
	}

	#[inline(always)]
	fn async_set_count_thread(&self, count_threads: usize) -> Result<SetCountResult, ErrSetCount> {
		(*self).async_set_count_thread(count_threads)
	}
}


impl<'a, A: MultThreadManager> MultThreadManager for &'a mut A {
	#[inline(always)]
	fn add_thread(&self, count_threads: usize) -> Result<usize, ErrAddThread> {
		(**self).add_thread(count_threads)
	}

	#[inline(always)]
	fn del_thread(&self, count_threads: usize) -> Result<usize, ErrDelThread> {
		(**self).del_thread(count_threads)
	}

	#[inline(always)]
	fn set_count_thread(&self, new_count: usize) -> Result<SetCountResult, ErrSetCount> {
		(**self).set_count_thread(new_count)
	}

	#[inline(always)]
	fn async_add_thread(&self, count_threads: usize) -> Result<usize, ErrAddThread> {
		(**self).async_add_thread(count_threads)
	}

	#[inline(always)]
	fn async_del_thread(&self, count_threads: usize) -> Result<usize, ErrDelThread> {
		(**self).async_del_thread(count_threads)
	}

	#[inline(always)]
	fn async_set_count_thread(&self, count_threads: usize) -> Result<SetCountResult, ErrSetCount> {
		(**self).async_set_count_thread(count_threads)
	}
}


#[derive(Debug)]
pub enum SetCountResult {
	None(usize),
     Add(usize),
     Del(usize),
}


#[derive(Debug)]
pub enum ErrSetCount {
     ErrMinThreads{to: usize, this: usize, min: usize},
	ErrMaxThreads{to: usize, this: usize, max: usize},
	ErrAddThread(ErrAddThread),
	ErrDelThread(ErrDelThread),
}


#[derive(Debug)]
pub enum ErrAddThread {
	Empty(usize),
	ErrMax{new: usize, max: usize},
}

#[derive(Debug)]
pub enum ErrDelThread {
	Empty(usize),
}

