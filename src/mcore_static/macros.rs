

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
          $crate::mcore_static::as_mult_thread()
	);
}

#[macro_export]
macro_rules! set_mult {
	($fmt:expr) => (
          $crate::mcore_static::set_boxed_mult_thread($fmt)
	);
}
