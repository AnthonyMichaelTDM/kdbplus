//! This module is provided as examples of "api" feature of `kdbplus` crate. The functions defined here will be
//!  used for simple tests.
//!
//!  # Note:
//!  need to have q installed to run the tests

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                            Load Libraries                            //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

// use kdbplus::qtype;
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
/// # Safety
/// assumes k is a valid pointer to a K value
#[no_mangle]
pub unsafe extern "C" fn plus_one_int(k: *const K) -> *const K {
    // assuming k is a non-null, and valid, pointer to a K value
    std::panic::catch_unwind(move || {
        let KVal::Int(KData::Atom(value)) = KVal::from(unsafe{&*k}) else {
             return new_error("type error\0");
        };
        new_int(value + 1)
    })
    .or_else::<u8, _>(|_| Ok(new_error("rust panic\0")))
    .unwrap()
}

#[cfg(test)]
#[test]
fn test_plus_one_int() {
    // declaring this separately to avoid lifetime issues
    let k_base = unsafe { native::ki(1) };
    let k = plus_one_int(k_base);
    // assert operation was successful
    assert_eq!(k, unsafe { native::ki(2) });
    // assert original value was not modified
    assert_eq!(k_base, unsafe { native::ki(1) });
}

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                              Re Exports                              //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

// example for register_callback //
static mut PIPE: [I; 2] = [-1, -1];
// Callback for some message queue.
extern "C" fn callback(socket: I) -> *const K {
    let mut buffer: [*mut K; 1] = [KNULL_MUT];
    unsafe { libc::read(socket, buffer.as_mut_ptr() as *mut V, 8) };
    // Call `shout` function on q side with the received data.
    let result = unsafe { error_to_string(native::k(0, str_to_S("shout"), buffer[0], KNULL)) };
    if let KVal::Err(&err_str) = KVal::from(unsafe { &*result }) {
        eprintln!("Execution error: {}", unsafe { S_to_str(err_str) });
        unsafe { decrement_reference_count(result) };
    };
    KNULL
}
///
#[no_mangle]
pub extern "C" fn plumber(_: *const K) -> *const K {
    if 0 != unsafe { libc::pipe(PIPE.as_mut_ptr()) } {
        return new_error("Failed to create pipe\0");
    }
    if KNULL == register_callback(unsafe { PIPE[0] }, callback) {
        return new_error("Failed to register callback\0");
    }
    // Lock symbol in a worker thread.
    pin_symbol();
    let handle = std::thread::spawn(move || {
        let precious = KVal::Symbol(KData::List(&[
            null_terminated_str_to_S("belief\0"),
            null_terminated_str_to_S("love\0"),
            null_terminated_str_to_S("hope\0"),
        ]))
        .to_k()
        .cast_mut();
        unsafe { libc::write(PIPE[1], std::mem::transmute::<*mut K, *mut V>(precious), 8) };
    });
    handle.join().unwrap();
    unpin_symbol();
    KNULL
}
