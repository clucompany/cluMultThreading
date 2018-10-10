
/*

//use mult_core::core_empty::MutEmptyCore;
use mult_core::MutCoreTrait;
use mult_core::CoreTrait;
use mult_core_constr::CoreConstructor;
use mult_core::core_destruct::CoreRootDestruct;
use std::sync::MutexGuard;
use std::sync::Mutex;

use std::sync::Arc;

pub type UnStruct<'a> = DefMultCore<'a>;
pub type MutStruct<'a> = MutexGuard<'a, DefMutMultCore<'a>>;

#[derive(Debug)]
pub struct DefMultCore<'a> {
     def_threads: usize,
     
     lock_core: Mutex<DefMutMultCore<'a>>,
}

#[derive(Debug)]
pub struct DefMutMultCore<'a> {
     root: &'a DefMultCore<'a>,
}

impl<'a> DefMultCore<'a> {
}


impl<'a> CoreConstructor<'a> for UnStruct<'a> {
     type Result = UnStruct<'a>;
     type LockResult = MutStruct<'a>;
     type Arguments = usize;

     fn constructor(a: Self::Arguments) -> Self::Result {
          Self::Result {
               def_threads: a,
               lock_core: Mutex::new(
                    DefMutMultCore {
                         root: unsafe {::std::mem::uninitialized()},
                    }
               ),
          }
     }
}



impl<'a> CoreTrait<'a> for UnStruct<'a> {
     type Lock = MutStruct<'a>;
     
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



impl<'a> MutCoreTrait<'a> for MutStruct<'a> {
     type Root = UnStruct<'a>;
     
}*/