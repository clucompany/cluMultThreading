


use std::fmt::Debug;
use mult_core::stat::MultStat;

pub trait MultDestruct: MultStat + Debug {
	fn destruct(&self);
}

