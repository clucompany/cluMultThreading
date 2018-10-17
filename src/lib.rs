
#![feature(arbitrary_self_types)]
#![feature(fnbox)]

#![feature(min_const_fn)]
#![feature(const_fn)]

#[macro_use]
extern crate cluLog;
#[macro_use]
extern crate enclose;

pub mod mcore;
pub mod mcore_static;
pub mod mcore_task;
pub mod mcore_destruct;
pub mod mcore_behavior;

use mcore::MultStatic;



#[inline(always)]
pub fn as_mult_thread() -> &'static MultStatic<'static> {
     mcore_static::as_mult_thread()
}



