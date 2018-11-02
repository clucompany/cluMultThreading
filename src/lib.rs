
#![feature(arbitrary_self_types)]
#![feature(fnbox)]

#![feature(const_fn)]
#![feature(associated_type_defaults)]

#[macro_use]
extern crate cluLog;
#[macro_use]
extern crate enclose;

pub mod mcore;
mod mcore_static;
pub mod mcore_task;
pub mod mcore_destruct;
pub mod mcore_behavior;

pub use self::mcore_static::*;





