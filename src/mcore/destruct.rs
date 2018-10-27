

use std::fmt::Debug;

pub trait MultDestruct: Debug {
	fn destruct(&self);
}

impl<'a, A: MultDestruct> MultDestruct for &'a A {
	#[inline(always)]
	fn destruct(&self) {
		(**self).destruct()
	}
}

impl<'a, A: MultDestruct> MultDestruct for &'a mut A {
	#[inline(always)]
	fn destruct(&self) {
		(**self).destruct()
	}
}