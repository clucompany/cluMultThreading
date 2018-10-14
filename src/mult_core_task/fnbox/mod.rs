

use std::fmt::Formatter;
use std::fmt::Debug;
use std::ops::Deref;
use std::boxed::FnBox;
use std::fmt;

pub struct DebugFnBox(Box<FnBox()>);

impl DebugFnBox {
     #[inline(always)]
     pub fn new(f: Box<FnBox()>) -> Self {
          DebugFnBox(f)
     }

     #[inline(always)]
     pub fn run(self) {
          self.0()
     }
}

impl Deref for DebugFnBox {
     type Target = Box<FnBox()>;

     #[inline(always)]
     fn deref<'a>(&'a self) -> &'a Self::Target {
          &self.0
     }
}

impl Debug for DebugFnBox {
     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
          write!(f, "FnBox()")
     }
}

