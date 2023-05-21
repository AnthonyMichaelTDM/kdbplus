//! This module is provided as examples of "api" feature of `kdbplus` crate. The functions defined here will be
//!  used for simple tests.
//!
//!  # Note:
//!  need to have q installed to run the tests

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                            Load Libraries                            //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

use std::borrow::Cow;

use kdbplus::rusty_api::native;
use kdbplus::rusty_api::types::*;
use kdbplus::rusty_api::*;
use kdbplus::*;
// use libc::{pipe, send};

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          Global Variables                            //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

/// Example of `KUNLL`.
#[no_mangle]
pub extern "C" fn vanity(_: *const K) -> *const K {
    println!("Initialized something, probably it is your mindset.");
    KNULL
}

/// Example of `qnull::U`.
#[no_mangle]
pub extern "C" fn guid_border(_: *const K) -> *const K {
    KVal::Guid(KData::Atom(&qnull_base::U)).to_k()
}

/// Example of `qnull::H`, `qinf::H` and `qninf::H`.
#[no_mangle]
pub extern "C" fn short_borders(_: *const K) -> *const K {
    KVal::Short(KData::List(Cow::from(vec![
        qnull_base::H,
        qinf_base::H,
        qninf_base::H,
    ])))
    .to_k()
}

/// Example of `qnull::I`, `qinf::I` and `qninf::I`.
#[no_mangle]
pub extern "C" fn int_borders(_: *const K) -> *const K {
    KVal::Int(KData::List(Cow::from(vec![
        qnull_base::I,
        qinf_base::I,
        qninf_base::I,
    ])))
    .to_k()
}

/// Example of `qnull::J`, `qinf::J` and `qninf::J`.
#[no_mangle]
pub extern "C" fn long_borders(_: *const K) -> *const K {
    KVal::Long(KData::List(Cow::from(vec![
        qnull_base::J,
        qinf_base::J,
        qninf_base::J,
    ])))
    .to_k()
}

/// Example of `qnull::E`, `qinf::E` and `qninf::E`.
#[no_mangle]
pub extern "C" fn real_borders(_: *const K) -> *const K {
    KVal::Real(KData::List(Cow::from(vec![
        qnull_base::E,
        qinf_base::E,
        qninf_base::E,
    ])))
    .to_k()
}

/// Example of `qnull::F`, `qinf::F` and `qninf::F`.
#[no_mangle]
pub extern "C" fn float_borders(_: *const K) -> *const K {
    KVal::Datetime(KData::List(Cow::from(vec![
        qnull_base::F,
        qinf_base::F,
        qninf_base::F,
    ])))
    .to_k()
}

/// Example of `qnull::C`.
#[no_mangle]
pub extern "C" fn char_border(_: *const K) -> *const K {
    KVal::Char(qnull_base::C).to_k()
}

/// Example of `qnull::S`.
#[no_mangle]
pub extern "C" fn string_borders(_: *const K) -> *const K {
    KVal::CompoundList(Cow::Borrowed(&[
        KVal::Symbol(KData::Atom(&str_to_S!(qnull_base::S)))
            .to_k()
            .cast_mut(),
        KVal::String(Cow::from(qnull_base::S)).to_k().cast_mut(),
    ]))
    .to_k()
}

//%% Utlity %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Example of `str_to_S!`.
#[no_mangle]
pub extern "C" fn pingpong(_: *const K) -> *const K {
    // evaluating q queries in rust still requires direct calls to native functions
    unsafe { native::k(0, str_to_S!("ping"), new_int(77), KNULL) }
}

/// Example of `null_terminated_str_to_const_S`.
///
/// # Safety
/// dereferences a raw pointer
#[no_mangle]
pub extern "C" fn must_be_int(obj: *const K) -> *const K {
    match KVal::from_raw(obj) {
        KVal::Int(KData::Atom(_)) => KNULL,
        _ => unsafe { native::krr(null_terminated_str_to_const_S("not an int\0")) },
    }
}

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                       KVal as Utilities                              //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

/// # Safety
/// input must be a valid pointer
///
// used to be the example for as_mut_slice
#[no_mangle]
pub extern "C" fn modify_long_list_a_bit(long_list: *const K) -> *const K {
    let val = KVal::from_raw(long_list);
    match val {
        KVal::Long(KData::List(mut list)) => {
            if list.len() < 2 {
                return new_error("this list is not long enough. how ironic...\0");
            }
            list.to_mut()[1] = 30000_i64;
            KVal::Long(KData::List(list)).to_k()
        }
        _ => new_error("invalid type\0"),
    }
}

#[no_mangle]
pub extern "C" fn print_bool(atom: *const K) -> *const K {
    match KVal::from_raw(atom) {
        KVal::Bool(KData::Atom(b)) => {
            println!("bool: {}", b);
            KNULL
        }
        _ => new_error("not a bool\0"),
    }
}

#[no_mangle]
pub extern "C" fn print_guid(atom: *const K) -> *const K {
    match KVal::from_raw(atom) {
        KVal::Guid(KData::Atom(guid)) => {
            let strguid = guid
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>();
            println!(
                "GUID: {}-{}-{}-{}-{}",
                &strguid[0..4],
                &strguid[4..6],
                &strguid[6..8],
                &strguid[8..10],
                &strguid[10..16]
            );
            KNULL
        }
        _ => new_error("not a GUID\0"),
    }
}

#[no_mangle]
pub extern "C" fn print_byte(atom: *const K) -> *const K {
    match KVal::from_raw(atom) {
        KVal::Byte(KData::Atom(byte)) => {
            println!("byte: {:#4x}", byte);
            KNULL
        }
        _ => new_error("not a byte\0"),
    }
}

#[no_mangle]
pub extern "C" fn print_short(atom: *const K) -> *const K {
    match KVal::from_raw(atom) {
        KVal::Short(KData::Atom(short)) => {
            println!("short: {}", short);
            KNULL
        }
        _ => new_error("not a short\0"),
    }
}

#[no_mangle]
pub extern "C" fn print_int(atom: *const K) -> *const K {
    // you may notice that this example is significantly longer than the example for the `api` module,
    // that's because to take advantage of rust's pattern matching and type system
    // we have to handle every valid case explicitly here
    use KData::*;
    use KVal::*; // for brevity // for brevity
                 // private macro to reduce code duplication
    macro_rules! print_int {
        ($int:ident) => {{
            println!("int: {}", $int);
            KNULL
        }};
    }

    match KVal::from_raw(atom) {
        KVal::Int(KData::Atom(int)) => print_int!(int),
        Month(Atom(month)) => print_int!(month),
        Date(Atom(date)) => print_int!(date),
        Minute(Atom(minute)) => print_int!(minute),
        Second(Atom(second)) => print_int!(second),
        Time(Atom(time)) => print_int!(time),
        _ => new_error("not an int\0"),
    }
}

#[no_mangle]
pub extern "C" fn print_long(atom: *const K) -> *const K {
    // you may notice that this example is significantly longer than the example for the `api` module,
    // that's because to take advantage of rust's pattern matching and type system
    // we have to handle every valid case explicitly here
    use KData::*;
    use KVal::*; // for brevity // for brevity
                 // private macro to reduce code duplication
    macro_rules! print_long {
        ($long:ident) => {{
            println!("long: {}", $long);
            KNULL
        }};
    }
    match KVal::from_raw(atom) {
        KVal::Long(KData::Atom(long)) => print_long!(long),
        Timestamp(Atom(timestamp)) => print_long!(timestamp),
        Timespan(Atom(timespan)) => print_long!(timespan),
        Enum(Atom(en)) => print_long!(en),
        _ => new_error("not a long\0"),
    }
}

#[no_mangle]
pub extern "C" fn print_real(atom: *const K) -> *const K {
    match KVal::from_raw(atom) {
        KVal::Real(KData::Atom(real)) => {
            println!("real: {}", real);
            KNULL
        }
        _ => new_error("not a real\0"),
    }
}

#[no_mangle]
pub extern "C" fn print_float(atom: *const K) -> *const K {
    // we have to explicitly handle every valid case
    match KVal::from_raw(atom) {
        KVal::Float(KData::Atom(float)) => {
            println!("float: {:.8}", float);
            KNULL
        }
        KVal::Datetime(KData::Atom(float)) => {
            println!("float: {:.8}", float);
            KNULL
        }
        _ => new_error("not a float\0"),
    }
}

#[no_mangle]
pub extern "C" fn print_char(atom: *const K) -> *const K {
    match KVal::from_raw(atom) {
        KVal::Char(char) => {
            println!("char: \"{}\"", char);
            KNULL
        }
        _ => new_error("not a char\0"),
    }
}

#[no_mangle]
pub extern "C" fn print_symbol2(atom: *const K) -> *const K {
    match KVal::from_raw(atom) {
        KVal::Symbol(KData::Atom(&symbol)) => {
            println!("symbol: '{}", unsafe { S_to_str(symbol) });
            KNULL
        }
        _ => new_error("not a symbol\0"),
    }
}

#[no_mangle]
pub extern "C" fn print_string(list: *const K) -> *const K {
    match KVal::from_raw(list) {
        KVal::String(string) => {
            println!("string: \"{}\"", &string.into_owned());
            KNULL
        }
        _ => new_error("not a string\0"),
    }
}

#[no_mangle]
pub extern "C" fn print_string2(list: *const K) -> *const K {
    match KVal::from_raw(list) {
        KVal::String(string) => {
            println!("string: \"{}\"", &string.into_owned());
            KNULL
        }
        _ => new_error("not a string\0"),
    }
}

// TODO: fn hidden_key(), pre-requisites: support dictionaries
#[no_mangle]
pub extern "C" fn hidden_key(table: *const K) -> *const K {
    todo!()
}

// TODO: fn pick_row(), pre-requisites: support tables
#[no_mangle]
pub extern "C" fn pick_row(obj: *const K, index: *const K) -> *const K {
    todo!();
}

/// example of KVal::join()
#[no_mangle]
pub extern "C" fn concat_list2(list1: *const K, list2: *const K) -> *const K {
    let list1 = KVal::from_raw(list1);
    let list2 = KVal::from_raw(list2);

    match list1.join(list2) {
        Ok(list3) => list3.to_k(),
        Err(e) => new_error(e),
    }
}

#[no_mangle]
pub extern "C" fn create_compound_list2(int: *const K) -> *const K {
    // we don't actually need to check if int is an int, because
    // compound lists can contain any type of K object
    let simp_list: KVal = KVal::Long(KData::List(Cow::from((0..5).collect::<Vec<i64>>())));
    let comp_list: KVal = simp_list.to_compound_list(None).unwrap();
    comp_list
        .join(KVal::CompoundList(Cow::Borrowed(&[int.cast_mut()])))
        .unwrap()
        .to_k()
}

#[no_mangle]
pub extern "C" fn create_symbol_list2(_: *const K) -> *const K {
    KVal::Symbol(KData::List(Cow::from(vec![
        str_to_S!("Abraham"),
        str_to_S!("Isaac"),
        str_to_S!("Jacob"),
        str_to_S!("Josephine"),
    ])))
    .to_k()
}

/// Example of `get_attribute`.
#[no_mangle]
pub extern "C" fn murmur(list: *const K) -> *const K {
    // TODO: add this functionality to KVal
    todo!();
}

/// Example of `set_attribute`.
#[no_mangle]
pub extern "C" fn labeling(mut list: *const K) -> *const K {
    // TODO: add this functionality to KVal
    todo!();
}

/// Example of `len`.
#[no_mangle]
pub extern "C" fn numbers(obj: *const K) -> *const K {
    //TODO: add len functionality to KVal
    todo!()
}

/// Example of `q_ipc_encode`.
#[no_mangle]
pub extern "C" fn encrypt(list: *const K) -> *const K {
    //TODO: add q_ipc_encode functionality to KVal
    todo!()
}

/// Example of `q_ipc_decode`.
#[no_mangle]
pub extern "C" fn decrypt(list: *const K) -> *const K {
    //TODO: add q_ipc_decode functionality to KVal
    todo!()
}

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                            Constructors                              //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                           KVal as a constructor                      //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

///////////////////////////////////////////
// examples for KVal::as_compound_list() //
///////////////////////////////////////////

// make a compound list from scratch
#[no_mangle]
pub extern "C" fn drift(_: *const K) -> *const K {
    KVal::CompoundList(Cow::Borrowed(&[
        KVal::Int(KData::Atom(&12)).to_k().cast_mut(),
        KVal::Int(KData::Atom(&34)).to_k().cast_mut(),
        KVal::Symbol(KData::Atom(&str_to_S!("vague")))
            .to_k()
            .cast_mut(),
        KVal::Int(KData::Atom(&-3000)).to_k().cast_mut(),
    ]))
    .to_k()
}
// make a compound list from an existing simple list
#[no_mangle]
pub extern "C" fn drift2(_: *const K) -> *const K {
    let existing_list = KVal::Enum(KData::List(Cow::from(vec![0_i64, 1]))); // error messages returned by 'as_compound_list' are null terminated

    // Convert a list of enum indices into a compound list while creating enum values from the indices which are tied with
    //  an existing enum variable named "enum", i.e., Enum indices [0, 1] in the code are cast into `(enum[0]; enum[1])`.
    let existing_list = match existing_list.to_compound_list(Some("enum")) {
        Ok(compound) => compound,
        Err(e_str) => return new_error(e_str),
    };

    // another compound list we want to add to the existing list
    let binding = [
        to_k!(KVal::Enum(KData::Atom(&2)), "enum2").cast_mut(), // `enum2[2]`.
        KVal::Month(KData::Atom(&3)).to_k().cast_mut(),
    ];
    let other_list = KVal::CompoundList(Cow::Borrowed(&binding));

    // return the joined list
    match existing_list.join(other_list) {
        Ok(joined) => joined.to_k(),
        Err(e_str) => new_error(e_str),
    }
}

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                              Re Exports                              //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

///////////////////////////////////
// example for register_callback //
///////////////////////////////////

static mut PIPE: [I; 2] = [-1, -1];
// Callback for some message queue.
extern "C" fn callback(socket: I) -> *const K {
    let mut buffer: [*mut K; 1] = [KNULL_MUT];
    unsafe { libc::read(socket, buffer.as_mut_ptr() as *mut V, 8) };
    // Call `shout` function on q side with the received data.
    let result = unsafe { error_to_string(native::k(0, str_to_S!("shout"), buffer[0], KNULL)) };
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
        let precious = KVal::Symbol(KData::List(Cow::from(vec![
            str_to_S!("belief"),
            str_to_S!("love"),
            str_to_S!("hope"),
        ])))
        .to_k()
        .cast_mut();
        unsafe { libc::write(PIPE[1], std::mem::transmute::<*mut K, *mut V>(precious), 8) };
    });
    handle.join().unwrap();
    unpin_symbol();
    KNULL
}
