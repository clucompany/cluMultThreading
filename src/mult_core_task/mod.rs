
pub mod run;
pub mod fnbox;

use mult_core_task::run::RunTask;
use mult_core_task::fnbox::DebugFnBox;

pub type Task = ERunTask;

#[derive(Debug)]
pub enum ERunTask {
     BoxFn(DebugFnBox),
     RunTask(Box<RunTask>),
}

impl ERunTask {
     pub fn run(self) {
          match self {
               ERunTask::BoxFn(a) => {
                    a.run();
               },
               ERunTask::RunTask(mut a) => {
                    a.run();
               },
          }
     }
}

