
mod run;
pub use self::run::*;

///Box trait for default tasks.
pub type Task = Box<RunTask>;


