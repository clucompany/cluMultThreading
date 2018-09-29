

/*

pub trait MultCoreConstructor<'a, M: 'a + Sized, A: 'a> {
     type Result;
     type Arguments;

     fn constructor(args: A) -> M;
}

*/

use mult_core::core_destruct::CoreRootDestruct;
use mult_core::MutMultCore;
use mult_core::UnMutMultCore;

pub trait MultCoreConstructor<'a> {
     type Result: UnMutMultCore<'a, Self::MutResult> + 'a;
     type MutResult: MutMultCore<'a, Self::Result> + 'a;
     type Arguments: 'a;

     fn root_destructor(args: Self::Arguments) -> CoreRootDestruct<'a, Self::Result, Self::MutResult>;
     fn constructor(args: Self::Arguments) -> Self::Result;
}


