


use mcore::MultStatic;
use mcore_destruct::MultStaticDestruct;
use mcore::MultExtend;
use mcore_destruct::MultRootDestruct;

pub trait MultRawDefault {
     fn new() -> Self where Self: Sized;

     fn thread(c: usize) -> Self where Self: Sized;

     #[inline]
     fn sys() -> Self where Self: Sized {
          extern crate num_cpus;
          Self::thread(num_cpus::get())
     }
}

pub trait MultDefault: MultRawDefault  {
	#[inline]
	fn root<'a>() -> MultRootDestruct<'a, Self> where Self: MultExtend<'a> + Sized {
		MultRootDestruct::new(
			Self::new()
		)
	}
	#[inline]
	fn root_thread<'a>(c: usize) -> MultRootDestruct<'a, Self> where Self: MultExtend<'a> + Sized {
		MultRootDestruct::new(
			Self::thread(c)
		)
	}
	#[inline]
	fn root_sys<'a>() -> MultRootDestruct<'a, Self> where Self: MultExtend<'a> + Sized {
		MultRootDestruct::new(
			Self::sys()
		)
	}

	#[inline]
	fn common() -> Option<MultStaticDestruct> where Self: 'static + MultStatic<'static> + Sized {
		::mcore_static::set_move_mult_thread(Self::new())
	}

	#[inline]
	fn common_thread(c: usize) -> Option<MultStaticDestruct> where Self: 'static + MultStatic<'static> + Sized {
		::mcore_static::set_move_mult_thread(Self::thread(c))
	}

	#[inline]
	fn common_sys() -> Option<MultStaticDestruct> where Self: 'static + MultStatic<'static> + Sized {
		::mcore_static::set_move_mult_thread(Self::sys())
	}
}