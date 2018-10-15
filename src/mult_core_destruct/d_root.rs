

use std::marker::PhantomData;
use std::ops::Deref;
use mult_core::MultExtend;

#[derive(Debug)]
pub struct MultRootDestruct<'a, M: MultExtend<'a> + Sized + 'a>(M, PhantomData<&'a ()>);


impl<'a, M: MultExtend<'a> + Sized + 'a> MultRootDestruct<'a, M> {
     #[inline]
     pub fn new(a: M) -> Self {
          MultRootDestruct(a, PhantomData)
     }
}


impl<'a, M: MultExtend<'a> + Sized + 'a> Deref for MultRootDestruct<'a, M> {
     type Target = M;

     #[inline(always)]
     fn deref<'n>(&'n self) -> &'n M {
          &self.0
     }
}

impl<'a, M: MultExtend<'a> + Sized + 'a> Drop for MultRootDestruct<'a, M> {
     #[inline(always)]
     fn drop(&mut self) {
          self.0.destruct();
     }
}

