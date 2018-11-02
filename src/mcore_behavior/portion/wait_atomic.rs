
use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicUsize;

#[derive(Debug)]
pub struct WaitAtomic<'a,>(&'a AtomicUsize, Ordering);

impl<'a> WaitAtomic<'a> {
     #[inline(always)]
     pub fn new(atomic: &'a AtomicUsize, ordering: Ordering) -> Self {

          atomic.fetch_add(1, ordering);

          WaitAtomic (atomic, ordering)
     }
}

impl<'a> Drop for WaitAtomic<'a> {
     #[inline(always)]
     fn drop(&mut self) {
          self.0.fetch_sub(1, self.1);
     }
}


#[derive(Debug)]
pub struct WaitAtomicNoInit<'a,>(&'a AtomicUsize, Ordering);

impl<'a> WaitAtomicNoInit<'a> {
     #[inline(always)]
     pub fn new(atomic: &'a AtomicUsize, ordering: Ordering) -> Self {

          //atomic.fetch_add(1, ordering);

          WaitAtomicNoInit (atomic, ordering)
     }
}

impl<'a> Drop for WaitAtomicNoInit<'a> {
     #[inline(always)]
     fn drop(&mut self) {
          self.0.fetch_sub(1, self.1);
     }
}



