//! This module is provided as examples of "api" feature of `kdbplus` crate. The functions defined here will be
//!  used for simple tests.
//!
//!  # Note:
//!  need to have q installed to run the tests

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                            Load Libraries                            //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

use kdbplus::rusty_api::native;
use kdbplus::rusty_api::types::*;
use kdbplus::rusty_api::*;
// use libc::{pipe, send};

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                           KVal as a constructor                      //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                  KVal as wrapper for operations                      //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

// example for KVal::from( &mut K )
#[no_mangle]
pub extern "C" fn plus_one_int(k: *mut K) -> *const K {
    // assuming k is a non-null, and valid, pointer to a K value
    std::panic::catch_unwind(move || {
        let KVal::Int(KData::Atom(value)) = KVal::from(unsafe{&mut *k}) else {
             return new_error("type error\0");
         };
        *value += 1;
        k.cast_const()
    })
    .or_else::<u8, _>(|_| Ok(new_error("rust panic\0")))
    .unwrap()
}

#[cfg(test)]
#[test]
fn test_plus_one_int() {
    // declaring this separately to avoid lifetime issues
    let k_base = unsafe { native::ki(1) }.cast_mut();
    let k = plus_one_int(k_base);
    assert_eq!(k, unsafe { native::ki(2) });
}
