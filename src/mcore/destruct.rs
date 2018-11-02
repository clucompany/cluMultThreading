

use std::fmt::Debug;

///Trait describing actions when destroying a scheduler.
pub trait MultDestruct: Debug {
	fn destruct(&self);
}

///Trait describing actions when destroying a scheduler.
impl<'a, A: MultDestruct> MultDestruct for &'a A {
	#[inline(always)]
	fn destruct(&self) {
		(**self).destruct()
	}
}

///Trait describing actions when destroying a scheduler.
impl<'a, A: MultDestruct> MultDestruct for &'a mut A {
	#[inline(always)]
	fn destruct(&self) {
		(**self).destruct()
	}
}