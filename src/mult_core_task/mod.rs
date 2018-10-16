
pub mod run;

use mult_core_task::run::RunTask;

pub type Task = Box<RunTask>;
