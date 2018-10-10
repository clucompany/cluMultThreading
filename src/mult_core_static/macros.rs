

#[macro_export]
macro_rules! mult {
	($fmt:expr) => (

	);
     ($fmt:expr, $($x:expr,)*) => (
          
     );
     (wait, $fmt:expr) => (

     );
}


#[macro_export]
macro_rules! as_mult {
	() => (
          $crate::mult_core_static::as_mult_thread()
	);
}

#[macro_export]
macro_rules! set_mult {
	($fmt:expr) => (
          $crate::mult_core_static::set_boxed_mult_thread($fmt)
	);
}
