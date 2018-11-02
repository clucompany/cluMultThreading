
use mcore_behavior::MultEmptyCore;
use mcore_destruct::StaticDestruct;
use mcore::MultStatic;
use std::sync::ONCE_INIT;
use std::sync::Once;

mod macros;

pub use self::macros::*;

static mut MULT_THREAD: &'static MultStatic = &MultEmptyCore;
static LOGGER_INIT: Once = ONCE_INIT;


///Get a link to the current scheduler
#[inline(always)]
pub fn as_mult_thread() -> &'static MultStatic<'static> {
     unsafe { MULT_THREAD }
}


///Change the current scheduler. If successful, the destructor returns, the destructor must be destroyed at the end of the program.
pub fn set_mult_thread(mult: &'static MultStatic) -> Option<StaticDestruct> {
     let mut result = None;

     LOGGER_INIT.call_once(|| {
          unsafe {
               MULT_THREAD = mult;

               result = Some(StaticDestruct);
          }
     });

     result
}

///Change the current scheduler. If successful, the destructor returns, the destructor must be destroyed at the end of the program.
pub fn set_box_mult_thread(log: Box<MultStatic<'static>>) -> Option<StaticDestruct> {
     //WHY CLONE?, 
     //if re-initialization does not occur, then additional unsafe will not occur.
     let mut result = None;

     LOGGER_INIT.call_once(|| {
          unsafe {
               MULT_THREAD = &*Box::into_raw(log);

               result = Some(StaticDestruct);
          }
     });

     result
}


///Change the current scheduler. If successful, the destructor returns, the destructor must be destroyed at the end of the program.
pub fn set_move_mult_thread<M: MultStatic<'static> + 'static>(log: M) -> Option<StaticDestruct> {
     let mut result = None;

     LOGGER_INIT.call_once(|| {
          unsafe {
               let log = Box::new(log);
               MULT_THREAD = &*Box::into_raw(log);
               

               result = Some(StaticDestruct);
          }
     });

     result
}

