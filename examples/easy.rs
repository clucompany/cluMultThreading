

extern crate cluMultThreading;

use cluMultThreading::mult_core::MutMultCore;
use cluMultThreading::mult_core::UnMutMultCore;
use cluMultThreading::mult_core_constr::MultCoreConstructor;
use std::sync::Arc;

fn main() {

     //let t = cluMultThreading::mult_core::core::MultCore::
     let t = Arc::new(cluMultThreading::mult_core::core_empty::UnStruct::constructor( () ));

     let mut lock = t.lock_core();
     
     println!("{:?}", lock.set_count_thread(&t, 10));

}