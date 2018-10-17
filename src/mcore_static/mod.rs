
use mcore_behavior::empty::MultEmptyCore;
use mcore_destruct::MultStaticDestruct;
use mcore::MultStatic;
use std::sync::ONCE_INIT;
use std::sync::Once;

pub mod macros;

static mut MULT_THREAD: &'static MultStatic = &MultEmptyCore;
static LOGGER_INIT: Once = ONCE_INIT;


#[inline(always)]
pub fn as_mult_thread() -> &'static MultStatic<'static> {
     unsafe { MULT_THREAD }
}

pub fn set_mult_thread(mult: &'static MultStatic) -> Option<MultStaticDestruct> {
     let mut result = None;

     LOGGER_INIT.call_once(|| {
          unsafe {
               MULT_THREAD = mult;

               result = Some(MultStaticDestruct);
          }
     });

     result
}

#[inline]
pub fn set_box_mult_thread(log: Box<MultStatic<'static>>) -> Option<MultStaticDestruct> {
     //WHY CLONE?, 
     //if re-initialization does not occur, then additional unsafe will not occur.
     let mut result = None;

     LOGGER_INIT.call_once(|| {
          unsafe {
               MULT_THREAD = &*Box::into_raw(log);

               result = Some(MultStaticDestruct);
          }
     });

     result
}

#[inline]
pub fn set_move_mult_thread<M: 'static + MultStatic<'static>>(log: M) -> Option<MultStaticDestruct> {
     //WHY CLONE?, 
     //if re-initialization does not occur, then additional unsafe will not occur.
     let mut result = None;

     LOGGER_INIT.call_once(|| {
          unsafe {
               let log = Box::new(log);
               MULT_THREAD = &*Box::into_raw(log);
               

               result = Some(MultStaticDestruct);
          }
     });

     result
}