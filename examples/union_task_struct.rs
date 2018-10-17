
#[macro_use]
extern crate cluMultThreading;
use cluMultThreading::mcore_task::RunTask;
use cluMultThreading::mcore_task::EmptyTask;

fn main() {

     let mut all = EmptyTask::new()
          .union(EmptyTask::new())
          .union(EmptyTask::new())
          .union(PrintlnTask("Test"))
          .union(PrintlnTask("Test"))
          .union(ConcatPrintlnTask("1", "2"))
          .union(EmptyTask::new());
     

     all.run();

}


#[derive(Debug)]
pub struct PrintlnTask<'a>(&'a str);

impl<'a> RunTask for PrintlnTask<'a> {
     fn run(&mut self) {
          println!("{}", self.0);
     }
}


#[derive(Debug)]
pub struct ConcatPrintlnTask<'a>(&'a str, &'a str);

impl<'a> RunTask for ConcatPrintlnTask<'a> {
     fn run(&mut self) {
          println!("{}->>{}", self.0, self.1);
     }
}