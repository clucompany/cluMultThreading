


use mult_core::MultStatic;
use mult_core_destruct::d_static::MultStaticDestruct;
use mult_core::MultExtend;
use mult_core_destruct::d_root::MultRootDestruct;
use std::sync::Arc;

pub trait MultRawDefault {
	#[inline]
     fn new() -> Self where Self: Sized;

     fn thread(c: usize) -> Self where Self: Sized;

     #[inline]
     fn sys() -> Self where Self: Sized {
          extern crate num_cpus;
          Self::thread(num_cpus::get())
     }
}

pub trait MultDefault where Self: MultRawDefault {
	#[inline]
	fn root<'a>() -> MultRootDestruct<'a, Self> where Self: MultExtend<'a> + Sized {
		MultRootDestruct::new(
			Arc::new(Self::new())
		)
	}
	#[inline]
	fn root_thread<'a>(c: usize) -> MultRootDestruct<'a, Self> where Self: MultExtend<'a> + Sized {
		MultRootDestruct::new(
			Arc::new(Self::thread(c))
		)
	}
	#[inline]
	fn root_sys<'a>() -> MultRootDestruct<'a, Self> where Self: MultExtend<'a> + Sized {
		MultRootDestruct::new(
			Arc::new(Self::sys())
		)
	}

	#[inline]
	fn common() -> Option<MultStaticDestruct> where Self: 'static + MultStatic<'static> + Sized {
		::mult_core_static::set_move_mult_thread(Self::new())
	}

	#[inline]
	fn common_thread(c: usize) -> Option<MultStaticDestruct> where Self: 'static + MultStatic<'static> + Sized {
		::mult_core_static::set_move_mult_thread(Self::thread(c))
	}

	#[inline]
	fn common_sys() -> Option<MultStaticDestruct> where Self: 'static + MultStatic<'static> + Sized {
		::mult_core_static::set_move_mult_thread(Self::sys())
	}
}