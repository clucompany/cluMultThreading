

use mult_core::core_destruct::CoreRootDestruct;
use std::marker::PhantomData;
use mult_core::MutMultCore;
use mult_core::UnMutMultCore;
use mult_core_constr::MultCoreConstructor;
use std::sync::Arc;

pub type UnStruct<'a> = EmptyCore<'a>;
pub type MutStruct<'a> = MutEmptyCore<'a>;


#[derive(Debug)]
pub struct EmptyCore<'a> {
     p: PhantomData<&'a ()>
}

#[derive(Debug)]
pub struct MutEmptyCore<'a> {
     p: PhantomData<&'a ()>
}


impl<'a> EmptyCore<'a> {
     pub fn new() -> Self {
          Self {
               p: PhantomData,
          }
     }
}

impl<'a> MutEmptyCore<'a> {
     pub fn new() -> Self {
          Self {
               p: PhantomData,
          }
     }
}


impl<'a> MultCoreConstructor<'a> for UnStruct<'a> {
     type Result = UnStruct<'a>;
     type MutResult = MutStruct<'a>;
     type Arguments = ();

     fn root_destructor(a: Self::Arguments) -> CoreRootDestruct<'a, Self::Result, Self::MutResult> {
          CoreRootDestruct::new(
               Arc::new(Self::constructor(a))
          )
     }
     fn constructor(_a: Self::Arguments) -> Self::Result {
          UnStruct::new()
     }
}



impl<'a> UnMutMultCore<'a, MutStruct<'a>> for UnStruct<'a> {
     fn def_thread_len(&self) -> usize {
          0
     }
     fn lock_core(&'a self) -> MutStruct<'a> {
          MutStruct::new()
     }
}    



impl<'a> MutMultCore<'a, UnStruct<'a>> for MutStruct<'a> {
     
}