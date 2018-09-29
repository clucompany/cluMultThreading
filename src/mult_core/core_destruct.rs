


use std::fmt::Debug;
use std::ops::Deref;
use std::marker::PhantomData;
use std::sync::Arc;
use mult_core::MutMultCore;
use mult_core::UnMutMultCore;

#[derive(Debug)]
pub struct CoreRootDestruct<'a, U: UnMutMultCore<'a, M> + 'a, M: 'a + MutMultCore<'a, U>>(Arc<U>, PhantomData<&'a M>);


impl<'a, U: UnMutMultCore<'a, M> + 'a, M: MutMultCore<'a, U> + 'a> CoreRootDestruct<'a, U, M> {
     #[inline]
     pub fn new(a: Arc<U>) -> Self {
          CoreRootDestruct( a, PhantomData )
     }
}


impl<'a, U: UnMutMultCore<'a, M> + 'a, M: MutMultCore<'a, U> + 'a> Deref for CoreRootDestruct<'a, U, M> {
     type Target = Arc<U>;

     fn deref(&self) -> &Self::Target {
          &self.0
     }
}


impl<'a, U: UnMutMultCore<'a, M> + 'a, M: MutMultCore<'a, U> + 'a> Drop for CoreRootDestruct<'a, U, M> {
     fn drop(&mut self) {

     }
}