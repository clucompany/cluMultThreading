


use mcore::MultStatic;
use mcore_destruct::StaticDestruct;
use mcore::MultExtend;
use mcore_destruct::RootDestruct;

pub trait MultRawDefault: Sized {
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
	fn root<'a>() -> RootDestruct<'a, T> where T: MultExtend<'a> + Sized {
		RootDestruct::new(
			Self::new()
		)
	}
	#[inline]
	fn root_thread<'a>(c: usize) -> RootDestruct<'a, T> where T: MultExtend<'a> + Sized {
		RootDestruct::new(
			Self::thread(c)
		)
	}
	#[inline]
	fn root_sys<'a>() -> RootDestruct<'a, T> where T: MultExtend<'a> + Sized {
		RootDestruct::new(
			Self::sys()
		)
	}

	#[inline]
	fn common() -> Option<StaticDestruct> where T: 'static + MultStatic<'static> + Sized {
		::mcore_static::set_move_mult_thread(Self::new())
	}

	#[inline]
	fn common_thread(c: usize) -> Option<StaticDestruct> where T: 'static + MultStatic<'static> + Sized {
		::mcore_static::set_move_mult_thread(Self::thread(c))
	}

	#[inline]
	fn common_sys() -> Option<StaticDestruct> where T: 'static + MultStatic<'static> + Sized {
		::mcore_static::set_move_mult_thread(Self::sys())
	}
}


impl<'l, A: MultDefault<T>, T> MultDefault<T> for &'l A  where Self: Sized {
	#[inline(always)]
	fn root<'a>() -> RootDestruct<'a, T> where T: MultExtend<'a> + Sized {
		A::root()
	}
	#[inline(always)]
	fn root_thread<'a>(c: usize) -> RootDestruct<'a, T> where T: MultExtend<'a> + Sized {
		A::root_thread(c)
	}
	#[inline(always)]
	fn root_sys<'a>() -> RootDestruct<'a, T> where T: MultExtend<'a> + Sized {
		A::root_sys()
	}

	#[inline(always)]
	fn common() -> Option<StaticDestruct> where T: 'static + MultStatic<'static> + Sized {
		A::common()
	}

	#[inline(always)]
	fn common_thread(c: usize) -> Option<StaticDestruct> where T: 'static + MultStatic<'static> + Sized {
		A::common_thread(c)
	}

	#[inline(always)]
	fn common_sys() -> Option<StaticDestruct> where T: 'static + MultStatic<'static> + Sized {
		A::common_sys()
	}
}


