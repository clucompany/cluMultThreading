


//use mult_core::core_empty::MutEmptyCore;
use mult_core::core_destruct::CoreRootDestruct;
use std::sync::MutexGuard;
use std::sync::Mutex;
use mult_core::MutMultCore;
use mult_core::UnMutMultCore;
use std::sync::Arc;

pub type UnStruct<'a> = MultCore<'a>;
pub type MutStruct<'a> = MutexGuard<'a, MultMutCore<'a>>;

#[derive(Debug)]
pub struct MultCore<'a> {
     def_threads: usize,
     
     lock_core: Mutex<MultMutCore<'a>>,
}

#[derive(Debug)]
pub struct MultMutCore<'a> {
     root: &'a MultCore<'a>,
}

impl<'a> MultCore<'a> {
     pub fn arc_destruct(a: usize) -> CoreRootDestruct<'a, Self, MutStruct<'a>> {
          CoreRootDestruct::new(
               Arc::new(
                    Self::threads(a)
               )
          )
     }
     pub fn threads(a: usize) -> Self {
          Self {
               def_threads: a,
               lock_core: Mutex::new(
                    MultMutCore {
                         root: unsafe {::std::mem::uninitialized()},
                    }
               ),
          }
     }
     pub fn threads_impl(a: usize) -> impl UnMutMultCore<'a, MutStruct<'a>> + 'a {
          Self::threads(a)
     }
}





impl<'a> UnMutMultCore<'a, MutStruct<'a>> for UnStruct<'a> {
     fn def_thread_len(&self) -> usize {
          self.def_threads
     }
     fn lock_core(&'a self) -> MutStruct<'a> {
          /*let mut lock = */match self.lock_core.lock() {
               Ok(a) => a,
               Err(e) => e.into_inner(),
          }/*;
          lock.root = self;
          lock*/
     }
}    



impl<'a> MutMultCore<'a, UnStruct<'a>> for MutStruct<'a> {
     
}