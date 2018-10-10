
use mult_core::empty_core::MultEmptyCore;
use mult_core::MultStatic;
use std::sync::ONCE_INIT;
use std::sync::Once;

pub mod macros;

static mut MULT_THREAD: &'static MultStatic = &MultEmptyCore;
static LOGGER_INIT: Once = ONCE_INIT;


#[inline(always)]
pub fn as_mult_thread() -> &'static MultStatic<'static> {
     unsafe { MULT_THREAD }
}

pub fn set_mult_thread(mult: &'static MultStatic) {
     LOGGER_INIT.call_once(|| {
          unsafe {
               MULT_THREAD.destruct();
               MULT_THREAD = mult;
          }
     });
}

#[inline]
pub fn set_boxed_mult_thread(log: Box<MultStatic<'static>>) {
	set_mult_thread( unsafe { &*Box::into_raw(log) } )
}