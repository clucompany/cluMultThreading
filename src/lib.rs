
#![feature(arbitrary_self_types)]
#![feature(fnbox)]

#![feature(min_const_fn)]
#![feature(const_fn)]

#[macro_use]
extern crate cluLog;

pub mod mult_core;
pub mod mult_core_static;
pub mod mult_core_task;
pub mod mult_core_destruct;
pub mod mul_core_behavior;

use mult_core::MultStatic;



#[inline(always)]
pub fn as_mult_thread() -> &'static MultStatic<'static> {
     mult_core_static::as_mult_thread()
}



