


use mcore::MultStatic;
use mcore_destruct::MultStaticDestruct;
use mcore::MultExtend;
use mcore_destruct::MultRootDestruct;

pub trait MultRawDefault where Self: Sized {
	type NewRes;

     fn new() -> Self::NewRes /*where Self: Sized*/;

     fn thread(c: usize) -> Self::NewRes /*where Self: Sized*/;

     #[inline]
     fn sys() -> Self::NewRes /*where Self: Sized*/ {
          extern crate num_cpus;
          Self::thread(num_cpus::get())
     }
}

impl<'a, A: MultRawDefault<NewRes = T>, T> MultRawDefault for &'a A  where Self: Sized {
	type NewRes = T;

	#[inline(always)]
	fn new() -> T {
		A::new()
	}

	#[inline(always)]
     fn thread(c: usize) -> T {
		A::thread(c)
	}

     #[inline]
     fn sys() -> T {
		A::sys()
	}
}



pub trait MultDefault<T>: MultRawDefault<NewRes = T> where Self: Sized {
	#[inline]
	fn root<'a>() -> MultRootDestruct<'a, T> where T: MultExtend<'a> + Sized {
		MultRootDestruct::new(
			Self::new()
		)
	}
	#[inline]
	fn root_thread<'a>(c: usize) -> MultRootDestruct<'a, T> where T: MultExtend<'a> + Sized {
		MultRootDestruct::new(
			Self::thread(c)
		)
	}
	#[inline]
	fn root_sys<'a>() -> MultRootDestruct<'a, T> where T: MultExtend<'a> + Sized {
		MultRootDestruct::new(
			Self::sys()
		)
	}

	#[inline]
	fn common() -> Option<MultStaticDestruct> where T: 'static + MultStatic<'static> + Sized {
		::mcore_static::set_move_mult_thread(Self::new())
	}

	#[inline]
	fn common_thread(c: usize) -> Option<MultStaticDestruct> where T: 'static + MultStatic<'static> + Sized {
		::mcore_static::set_move_mult_thread(Self::thread(c))
	}

	#[inline]
	fn common_sys() -> Option<MultStaticDestruct> where T: 'static + MultStatic<'static> + Sized {
		::mcore_static::set_move_mult_thread(Self::sys())
	}
}


impl<'l, A: MultDefault<T>, T> MultDefault<T> for &'l A  where Self: Sized {
	#[inline(always)]
	fn root<'a>() -> MultRootDestruct<'a, T> where T: MultExtend<'a> + Sized {
		A::root()
	}
	#[inline(always)]
	fn root_thread<'a>(c: usize) -> MultRootDestruct<'a, T> where T: MultExtend<'a> + Sized {
		A::root_thread(c)
	}
	#[inline(always)]
	fn root_sys<'a>() -> MultRootDestruct<'a, T> where T: MultExtend<'a> + Sized {
		A::root_sys()
	}

	#[inline(always)]
	fn common() -> Option<MultStaticDestruct> where T: 'static + MultStatic<'static> + Sized {
		A::common()
	}

	#[inline(always)]
	fn common_thread(c: usize) -> Option<MultStaticDestruct> where T: 'static + MultStatic<'static> + Sized {
		A::common_thread(c)
	}

	#[inline(always)]
	fn common_sys() -> Option<MultStaticDestruct> where T: 'static + MultStatic<'static> + Sized {
		A::common_sys()
	}
}


