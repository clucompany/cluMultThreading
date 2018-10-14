
use mult_core::thread::ErrDelThread;
use mul_core_behavior::portion::ArcPortionCore;
use mul_core_behavior::portion::PortionCore;
use mult_core::thread::ErrAddThread;
use std::sync::Arc;
use std::sync::Barrier;


#[derive(Debug)]
pub struct PortionThreadManager {
     count_threads: usize
}


impl PortionThreadManager {
     pub fn thread(c: usize, core: &Arc<ArcPortionCore>) -> Self {
          let mut portion = Self {
               count_threads: c
          };
          if c > 0 {
               portion.add_thread(c, core);
          }
          
          portion
     }


     #[inline(always)]
     pub fn as_count_threads<'a>(&'a self) -> &'a usize {
          &self.count_threads
     }

     #[inline]
     pub fn add_thread(&mut self, c: usize, core: &Arc<ArcPortionCore>) -> Result<usize, ErrAddThread> {
          inf!("Portion: AddThread {}", c);
          if c > 0 {
               return Ok( self._add_thread(c, core) );
          }

          Err( ErrAddThread::Empty(c) )
     }
     #[inline]
     pub fn del_thread(&mut self, c: usize, core: &Arc<ArcPortionCore>) -> Result<usize, ErrDelThread> {
          inf!("Portion: DelThread {}", c);
          if c >= self.count_threads {
               return Ok( self._del_thread(c, core) );
          }

          Err( ErrDelThread::Empty(c) )
     }

     fn _del_thread(&mut self, c: usize, core: &Arc<ArcPortionCore>) -> usize {
          self.count_threads -= c;
          c
     }

     fn _add_thread(&mut self, c: usize, core: &Arc<ArcPortionCore>) -> usize {
          let barrier = Arc::new(Barrier::new(c));
          
          for mut num in 0..c {
               num += self.count_threads;


               inf!("Portion: Start thread, #{}", num);

               ::std::thread::spawn(move || {
                    
               });

               
          }
          self.count_threads += c;
          c
     }
}

