
extern crate num_cpus;

use std::sync::Arc;
use std::fmt::Debug;

pub mod core;
pub mod core_empty;
pub mod core_destruct;


//type EmptyCore = core_empty::EmptyCore;
//type EmptyMutCore = core_empty::MutEmptyCore;

type DefCore<'a> = core::UnStruct<'a>;
type DefMutCore<'a> = core::MutStruct<'a>;

/*
pub fn empty() -> impl UnMutMultCore<EmptyMutCore> {
     EmptyCore::new()     
}
*/

/*
pub fn threads<M: MutMultCore>(a: usize) -> Box<UnMutMultCore<M>> {
     if a == 0 {
          return Box::new(empty());
     }

     Box::new(DefCore::threads(a))
}*/

pub fn system_threads<'a>() -> impl UnMutMultCore<'a, DefMutCore<'a>> + 'a {
     DefCore::threads(num_cpus::get())
}


pub trait UnMutMultCore<'a, M: MutMultCore<'a, Self> + 'a>: Debug where Self: 'a + Sized {
     fn def_thread_len(&self) -> usize;
     fn lock_core(&'a self) -> M;

     fn set_count_thread(self: &'a Arc<Self>, count: usize) -> Result<SetCountOk, SetCountErr> {
		let mut lock = self.lock_core();
		/*inf!("Connector threads {}->{}", lock_count, count);

		if count < MinThreads {
			return Some(SetCountResult::MinThreads(MinThreads));
		}*/

		lock.set_count_thread(self, count)
	}
}

pub trait MutMultCore<'a, R: UnMutMultCore<'a, Self> + 'a>: Debug where Self: 'a + Sized {
     fn set_count_thread(&mut self, root: &'a Arc<R>, a: usize) -> Result<SetCountOk, SetCountErr> {
          /*if *lock_count == count {
			warn!("Runnable, unknown set_count_threads count, len == count");
			//ничего не делать, количество оки
			return Ok(SetCountOk::Equally);
		}
		
		if *lock_count > count {
			//уменьшить
			let ncount = (*lock_count)-count;
			return SetCountResult::DelThreads( self._del_thread(ncount, lock_count) );
		}
		//if *lock_count < count {
			//увеличить
			self._add_thread(count, lock_count);
			return SetCountResult::AddThreads( count );
		//}
		//None
		*/

		Err( SetCountErr::ErrMinThreads )
     }
}

#[derive(Debug)]
pub enum SetCountOk {
     Equally,
     AddThreads(usize),
     DelThreads(usize),
}

#[derive(Debug)]
pub enum SetCountErr {
     ErrMinThreads,
}

