
use std::ops::Deref;
use mult_core::MultStatic;


#[derive(Debug)]
pub struct MultStaticDestruct;

impl MultStaticDestruct {
     #[inline]
     pub const fn new() -> Self {
          MultStaticDestruct
     }
}


impl Deref for MultStaticDestruct {
     type Target = MultStatic<'static>;

     #[inline(always)]
     fn deref(&self) -> &Self::Target {
          ::mult_core_static::as_mult_thread()
     }
}

impl Drop for MultStaticDestruct {
     #[inline(always)]
     fn drop(&mut self) {
          self.destruct();
     }
}