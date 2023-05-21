//! Re-export of native functions.
//!
//! generally, anything here is going to be unsafe, it's generally safer to use the
//! [`KVal`](super::types::KVal) wrapper. However, that doesn't always fit every use case, so
//! these are here to provide that extra flexibility.

use crate::{qtype, str_to_S};

use super::{native, utils::*, E, F, G, H, I, J, K, KNULL, KNULL_MUT, S, U, V};

//++++++++++++++++++++++++++++++++++++++++++++++++++//
// >> Re-export
//++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% Constructor %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Constructor of q bool object. Relabeling of `kb`.
/// # Example
/// ```no_run
/// use kdbplus::rusty_api::*;
///
/// #[no_mangle]
/// pub extern "C" fn create_bool(_: *const K) -> *const K{
///   new_bool(false)
/// }
/// ```
/// ```q
/// q)no: `libapi_examples 2: (`create_bool; 1);
/// q)no[]
/// 0b
/// ```
#[inline]
pub fn new_bool(boolean: bool) -> *const K {
    unsafe { native::kb(boolean as I) }
}

/// Constructor of q GUID object. Relabeling of `ku`.
/// # Example
/// ```no_run
/// use kdbplus::rusty_api::*;
///
/// #[no_mangle]
/// pub extern "C" fn create_guid(_: *const K) -> *const K{
///   new_guid([0x1e_u8, 0x11, 0x17, 0x0c, 0x42, 0x24, 0x25, 0x2c, 0x1c, 0x14, 0x1e, 0x22, 0x4d, 0x3d, 0x46, 0x24])
/// }
/// ```
/// ```q
/// q)create_guid: `libapi_examples 2: (`create_guid; 1);
/// q)create_guid[]
/// 1e11170c-4224-252c-1c14-1e224d3d4624
/// ```
#[inline]
pub fn new_guid(guid: [G; 16]) -> *const K {
    unsafe { native::ku(U::new(guid)) }
}

/// Constructor of q byte object. Relabeling of `kg`.
/// # Example
/// ```no_run
/// use kdbplus::rusty_api::*;
///
/// #[no_mangle]
/// pub extern "C" fn create_byte(_: *const K) -> *const K{
///   new_byte(0x3c)
/// }
/// ```
/// ```q
/// q)create_byte: `libapi_examples 2: (`create_byte; 1);
/// q)create_byte[]
/// 0x3c
/// ```
#[inline]
pub fn new_byte(byte: I) -> *const K {
    unsafe { native::kg(byte) }
}

/// Constructor of q short object. Relabeling of `kh`.
/// # Example
/// ```no_run
/// use kdbplus::rusty_api::*;
///
/// #[no_mangle]
/// pub extern "C" fn create_short(_: *const K) -> *const K{
///   new_short(-144)
/// }
/// ```
/// ```q
/// q)shortage: `libapi_examples 2: (`create_short; 1);
/// q)shortage[]
/// -144h
/// ```
#[inline]
pub fn new_short(short: I) -> *const K {
    unsafe { native::kh(short) }
}

/// Constructor of q int object. Relabeling of `ki`.
/// # Example
/// ```no_run
/// use kdbplus::rusty_api::*;
///
/// #[no_mangle]
/// pub extern "C" fn create_int(_: *const K) -> *const K{
///   new_int(86400000)
/// }
/// ```
/// ```q
/// q)trvial: `libapi_examples 2: (`create_int; 1);
/// q)trivial[]
/// 86400000i
/// ```
#[inline]
pub fn new_int(int: I) -> *const K {
    unsafe { native::ki(int) }
}

/// Constructor of q long object. Relabeling of `kj`.
/// # Example
/// ```no_run
/// use kdbplus::rusty_api::*;
///
/// #[no_mangle]
/// pub extern "C" fn create_long(_: *const K) -> *const K{
///   new_long(-668541276001729000)
/// }
/// ```
/// ```q
/// q)lengthy: `libapi_examples 2: (`create_long; 1);
/// q)lengthy[]
/// -668541276001729000
/// ```
#[inline]
pub fn new_long(long: J) -> *const K {
    unsafe { native::kj(long) }
}

/// Constructor of q real object. Relabeling of `ke`.
/// # Example
/// ```no_run
/// use kdbplus::rusty_api::*;
///
/// #[no_mangle]
/// pub extern "C" fn create_real(_: *const K) -> *const K{
///   new_real(0.00324)
/// }
/// ```
/// ```q
/// q)reality: `libapi_examples 2: (`create_real; 1);
/// q)reality[]
/// 0.00324e
/// ```
#[inline]
pub fn new_real(real: F) -> *const K {
    unsafe { native::ke(real) }
}

/// Constructor of q float object. Relabeling of `kf`.
/// # Example
/// ```
/// use kdbplus::rusty_api::*;
///
/// #[no_mangle]
/// pub extern "C" fn create_float(_: *const K) -> *const K{
///   new_float(-6302.620)
/// }
/// ```
/// ```q
/// q)coffee_float: `libapi_examples 2: (`create_float; 1);
/// q)coffee_float[]
/// -6302.62
/// ```
#[inline]
pub fn new_float(float: F) -> *const K {
    unsafe { native::kf(float) }
}

///  Constructor of q char object. Relabeling of `kc`.
/// # Example
/// ```no_run
/// use kdbplus::rusty_api::*;
///
/// #[no_mangle]
/// pub extern "C" fn create_char2(_: *const K) -> *const K{
///   new_char('t')
/// }
/// ```
/// ```q
/// q)heavy: `libapi_examples 2: (`create_char2; 1);
/// q)heavy[]
/// "t"
/// ```
#[inline]
pub fn new_char(character: char) -> *const K {
    unsafe { native::kc(character as I) }
}

/// Constructor of q symbol object. Relabeling of `ks`.
/// # Example
/// ```no_run
/// use kdbplus::rusty_api::*;
///
/// #[no_mangle]
/// pub extern "C" fn create_symbol2(_: *const K) -> *const K{
///   new_symbol("symbolic")
/// }
/// ```
/// ```q
/// q)hard: `libapi_examples 2: (`create_symbol2; 1);
/// q)hard[]
/// `symbolic
/// q)`symbolic ~ hard[]
/// 1b
/// ```
#[inline]
pub fn new_symbol(symbol: &str) -> *const K {
    unsafe { native::ks(str_to_S!(symbol)) }
}

/// Constructor of q symbol object from `S`. Relabeling of `ks`.
///
/// same as [`new_symbol`](crate::rusty_api::re_exports::new_symbol) but accepts `S` instead of `&str`
///
/// # Safety
/// passed `cstring` must be valid, meaning it meets the conditions of [`core::ffi:c_str::CStr::from_ptr`]
#[inline]
#[allow(non_snake_case)]
pub unsafe fn new_symbol_from_S(symbol: S) -> *const K {
    unsafe { native::ks(symbol) }
}

/// Constructor of q timestamp from elapsed time in nanoseconds since kdb+ epoch (`2000.01.01`). Relabeling of `ktj`.
/// ```no_run
/// use kdbplus::rusty_api::*;
///
/// #[no_mangle]
/// pub extern "C" fn create_timestamp2(_: *const K) -> *const K{
///   // 2015.03.16D00:00:00:00.000000000
///   new_timestamp(479779200000000000)
/// }
/// ```
/// ```q
/// q)stamp: `libapi_examples 2: (`create_timestamp2; 1);
/// q)stamp[]
/// 2015.03.16D00:00:00.000000000
/// ```
#[inline]
pub fn new_timestamp(nanoseconds: J) -> *const K {
    unsafe { native::ktj(qtype::TIMESTAMP_ATOM as I, nanoseconds) }
}

/// Create a month object from the number of months since kdb+ epoch (`2000.01.01`).
///  This is a complememtal constructor of missing month type.
/// # Example
/// ```no_run
/// use kdbplus::rusty_api::*;
///
/// #[no_mangle]
/// pub extern "C" fn create_month(_: *const K) -> *const K{
///   // 2010.07m
///   new_month(126)
/// }
/// ```
/// ```q
/// q)create_month: `libapi_examples 2: (`create_month; 1);
/// q)create_month[]
/// 2010.07m
/// ```
#[inline]
pub fn new_month(months: I) -> *const K {
    unsafe {
        let month = native::ka(qtype::MONTH_ATOM as I) as *mut K;
        (*month).value.int = months;
        month
    }
}

/// Constructor of q date object. Relabeling of `kd`.
/// # Example
/// ```no_run
/// use kdbplus::rusty_api::*;
///
/// #[no_mangle]
/// pub extern "C" fn create_date(_: *const K) -> *const K{
///   // 1999.12.25
///   new_date(-7)
/// }
/// ```
/// ```q
/// q)nostradamus: `libapi_examples 2: (`create_date; 1);
/// q)nostradamus[]
/// 1999.12.25
/// ```
#[inline]
pub fn new_date(days: I) -> *const K {
    unsafe { native::kd(days) }
}

/// Constructor of q datetime object from the number of days since kdb+ epoch (`2000.01.01`). Relabeling of `kz`.
/// ```no_run
/// use kdbplus::rusty_api::*;
///
/// #[no_mangle]
/// pub extern "C" fn create_datetime(_: *const K) -> *const K{
///   // 2015.03.16T12:00:00:00.000
///   new_datetime(5553.5)
/// }
/// ```
/// ```q
/// q)omega_date: libc_api_examples 2: (`create_datetime; 1);
/// q)omega_date[]
/// 2015.03.16T12:00:00.000
/// ```
#[inline]
pub fn new_datetime(days: F) -> *const K {
    unsafe { native::kz(days) }
}

/// Constructor of q timespan object from nanoseconds. Relabeling of `ktj`.
/// ```no_run
/// use kdbplus::rusty_api::*;
///
/// #[no_mangle]
/// pub extern "C" fn create_timespan2(_: *const K) -> *const K{
///   // -1D01:30:00.001234567
///   new_timespan(-91800001234567)
/// }
/// ```
/// ```q
/// q)duration: libc_api_examples 2: (`create_timespan2; 1);
/// q)duration[]
/// -1D01:30:00.001234567
/// ```
#[inline]
pub fn new_timespan(nanoseconds: J) -> *const K {
    unsafe { native::ktj(qtype::TIMESPAN_ATOM as I, nanoseconds) }
}

/// Create a month object. This is a complememtal constructor of
///  missing minute type.
/// # Example
/// ```no_run
/// use kdbplus::rusty_api::*;
///
/// #[no_mangle]
/// pub extern "C" fn create_minute(_: *const K) -> *const K{
///   // 10:40
///   new_minute(640)
/// }
/// ```
/// ```q
/// q)minty: `libapi_examples 2: (`create_minute; 1);
/// q)minty[]
/// 10:40
/// ```
#[inline]
pub fn new_minute(minutes: I) -> *const K {
    unsafe {
        let minute = native::ka(qtype::MINUTE_ATOM as I) as *mut K;
        (*minute).value.int = minutes;
        minute
    }
}

/// Create a month object. This is a complememtal constructor of
///  missing second type.
/// # Example
/// ```no_run
/// use kdbplus::rusty_api::*;
///
/// #[no_mangle]
/// pub extern "C" fn create_second(_: *const K) -> *const K{
///   // -02:00:00
///   new_second(-7200)
/// }
/// ```
/// ```q
/// q)third: `libapi_examples 2: (`create_second; 1);
/// q)third[]
/// -02:00:00
/// ```
#[inline]
pub fn new_second(seconds: I) -> *const K {
    unsafe {
        let second = native::ka(qtype::SECOND_ATOM as I) as *mut K;
        (*second).value.int = seconds;
        second
    }
}

/// Constructor of q time object. Relabeling of `kt`.
/// # Example
/// ```no_run
/// use kdbplus::rusty_api::*;
///
/// #[no_mangle]
/// pub extern "C" fn create_time(_: *const K) -> *const K{
///   // -01:30:00.123
///   new_time(-5400123)
/// }
/// ```
/// ```q
/// q)ancient: libc_api_examples 2: (`create_time; 1);
/// q)ancient[]
/// -01:30:00.123
/// ```
#[inline]
pub fn new_time(milliseconds: I) -> *const K {
    unsafe { native::kt(milliseconds) }
}

/// Constructor of q enum object. This is a complememtal constructor of
///  missing second type.
/// # Example
/// ```no_run
/// use kdbplus::rusty_api::*;
/// use kdbplus::rusty_api::types::*;
///
/// #[no_mangle]
/// pub extern "C" fn create_enum(source: *const K, index: *const K) -> *const K{
///     // Error if the specified enum source does not exist or it is not a symbol list or the index is out of enum range
///     match (KVal::from_raw(source), KVal::from_raw(index)) {
///         (KVal::Symbol(KData::Atom(&source)), KVal::Long(KData::Atom(&index))) => {
///             new_enum(unsafe {S_to_str(source.clone())}, index)
///         }
///         _ => new_error("type error, source must be symbol atom and index must be long atom\0")
///     }
/// }
/// ```
/// ```q
/// q)enumerate: libc_api_examples 2: (`create_enum; 2);
/// q)sym: `a`b`c
/// q)enumerate["sym"; 1]
/// `sym$`b
/// q)enumerate["sym"; 3]
/// 'index out of enum range
///   [0]  enumerate["sym"; 3]
///        ^
/// q)enumerate["som"; 0]
/// 'som
/// [1]  som
///      ^
/// q))\
/// q)som:til 3
/// q)enumerate["som"; 0]
/// 'enum must be cast to symbol list
///   [0]  enumerate["som"; 0]
///        ^
/// q)som:`a`b
/// q)enumerate["som"; 0]
/// `som$`a
/// ```
#[inline]
pub fn new_enum(source: &str, index: J) -> *const K {
    let sym = unsafe { native::k(0, str_to_S!(source), KNULL) };
    if unsafe { (*sym).qtype } == qtype::ERROR {
        // Error. Specified sym does not exist
        sym
    } else if unsafe { (*sym).qtype } != qtype::SYMBOL_LIST {
        // sym is not a symbol list
        unsafe {
            native::r0(sym);
            native::krr(null_terminated_str_to_const_S(
                "enum must be cast to symbol list\0",
            ))
        }
    } else if unsafe { (*sym).value.list.n } <= index {
        // Index is out of sym range
        unsafe {
            native::r0(sym);
            native::krr(null_terminated_str_to_const_S("index out of enum range\0"))
        }
    } else {
        let function = format!("{{`{}${} x}}", source, source);
        unsafe {
            native::r0(sym);
            native::k(0, str_to_S!(function.as_str()), native::kj(index), KNULL)
        }
    }
}

/// Constructor of q simple list.
/// # Example
/// See the example of [`new_dictionary`](fn.new_dictionary.html).
#[inline]
pub fn new_list(qtype: i8, length: J) -> *const K {
    unsafe { native::ktn(qtype as I, length) }
}

/// Constructor of q string object.
/// # Example
/// ```no_run
/// use kdbplus::rusty_api::*;
///
/// #[no_mangle]
/// pub extern "C" fn create_string(_: *const K) -> *const K{
///   new_string("this is a text.")
/// }
/// ```
/// ```q
/// q)text: libc_api_examples 2: (`create_string; 1);
/// q)text[]
/// "this is a text."
/// ```
#[inline]
pub fn new_string(string: &str) -> *const K {
    unsafe { native::kp(str_to_S!(string)) }
}

/// same as [`new_string`] but without the conversion from `&str` to `S`,
///
/// # Safety
/// passed `cstring` must be valid, meaning it meets the conditions of [`core::ffi:c_str::CStr::from_ptr`]
#[inline]
#[allow(non_snake_case)]
pub unsafe fn new_string_from_S(cstring: S) -> *const K {
    unsafe { native::kp(cstring) }
}

/// Constructor if q string object with a fixed length.
/// # Example
/// ```no_run
/// use kdbplus::rusty_api::*;
///
/// #[no_mangle]
/// pub extern "C" fn create_string2(_: *const K) -> *const K{
///   new_string_n("The meeting was too long and I felt it s...", 24)
/// }
/// ```
/// ```q
/// q)speak_inwardly: libc_api_examples 2: (`create_string2; 1);
/// q)speak_inwardly[]
/// "The meeting was too long"
/// ```
#[inline]
pub fn new_string_n(string: &str, length: J) -> *const K {
    unsafe { native::kpn(str_to_S!(string), length) }
}

/// Constructor of q dictionary object.
/// # Example
/// ```no_run
/// use kdbplus::rusty_api::*;
/// use kdbplus::qtype;
///
/// #[no_mangle]
/// pub extern "C" fn create_dictionary() -> *const K{
///   let keys=new_list(qtype::INT_LIST, 2).cast_mut();
///   unsafe { (*keys).as_mut_slice_unchecked::<I>()[0..2].copy_from_slice(&[0, 1]) };
///   let values=new_list(qtype::COMPOUND_LIST, 2).cast_mut();
///   let date_list=new_list(qtype::DATE_LIST, 3).cast_mut();
///   // 2000.01.01 2000.01.02 2000.01.03
///   unsafe { (*date_list).as_mut_slice_unchecked::<I>()[0..3].copy_from_slice(&[0, 1, 2])};
///   let string=new_string("I'm afraid I would crash the application...").cast_mut();
///   unsafe { (*values).as_mut_slice_unchecked::<*mut K>()[0..2].copy_from_slice(&[date_list, string])};
///   unsafe {new_dictionary(keys, values)}
/// }
/// ```
/// ```q
/// q)create_dictionary: `libapi_examples 2: (`create_dictionary; 1);
/// q)create_dictionary[]
/// 0| 2000.01.01 2000.01.02 2000.01.03
/// 1| "I'm afraid I would crash the application..."
/// ```
///
/// # Safety
/// inputs must be valid pointers
#[inline]
pub unsafe fn new_dictionary(keys: *const K, values: *const K) -> *const K {
    unsafe { native::xD(keys, values) }
}

/// Constructor of q general null.
/// # Example
/// ```no_run
/// use kdbplus::qtype;
/// use kdbplus::rusty_api::*;
/// use kdbplus::rusty_api::types::*;
/// use std::borrow::Cow;
///
///
/// #[no_mangle]
/// pub extern "C" fn nullify(_: *const K) -> *const K {
///     KVal::CompoundList(Cow::Borrowed(&[
///         new_null().cast_mut(),
///         new_string("null is not a general null").cast_mut(),
///         new_null().cast_mut(),
///     ])).to_k()
/// }
/// ```
/// ```q
/// q)void: `libapi_examples 2: (`nullify; 1);
/// q)void[]
/// ::
/// "null is not a general null"
/// ::
/// ```
#[inline]
pub fn new_null() -> *const K {
    unsafe {
        let null = native::ka(qtype::NULL as I) as *mut K;
        (*null).value.byte = 0;
        null
    }
}

/// Constructor of q error. The input must be null-terminated.
/// # Example
/// ```no_run
/// use kdbplus::rusty_api::*;
///
/// pub extern "C" fn thai_kick(_: *const K) -> *const K{
///   new_error("Thai kick unconditionally!!\0")
/// }
/// ```
/// ```q
/// q)monstrous: `libapi_examples 2: (`thai_kick; 1);
/// q)monstrous[]
/// 'Thai kick unconditionally!!
/// [0]  monstrous[]
///      ^
/// ```
#[inline]
pub fn new_error(message: &str) -> *const K {
    unsafe { native::krr(null_terminated_str_to_const_S(message)) }
}

/// same as [`new_error`] but without the conversion from `&str` to `S`,
///
/// # Safety
/// passed `cstring` must be valid, meaning it meets the conditions of [`core::ffi:c_str::CStr::from_ptr`]
#[inline]
#[allow(non_snake_case)]
pub unsafe fn new_error_from_S(cstring: S) -> *const K {
    unsafe { native::krr(cstring) }
}

/// Similar to `new_error` but this function appends a system-error message to string `S` before passing it to internal `krr`.
///  The input must be null-terminated.
#[inline]
pub fn new_error_os(message: &str) -> *const K {
    unsafe { native::orr(null_terminated_str_to_const_S(message)) }
}

/// Convert an error object into usual `K` object which has the error string in the field `symbol`.
/// # Example
/// ```no_run
/// use kdbplus::*;
/// use kdbplus::rusty_api::*;
/// use kdbplus::rusty_api::types::*;
///
/// #[no_mangle]
/// extern "C" fn no_panick(func: *const K, args: *const K) -> *const K{
///     let result=unsafe{error_to_string(apply(func, args))};
///     match KVal::from_raw(result) {
///         KVal::Error(&error) => {
///             println!("FYI: {}", unsafe { S_to_str(error) } );
///             // Decrement reference count of the error object which is no longer used.
///             unsafe{decrement_reference_count(result)};
///             KNULL
///         },
///         _ => result
///     }
/// }
/// ```
/// ```q
/// q)chill: `libapi_examples 2: (`no_panick; 2);
/// q)chill[$; ("J"; "42")]
/// success!
/// 42
/// q)chill[+; (1; `a)]
/// FYI: type
/// ```
/// # Note
/// If you intend to use the error string only in Rust side and not to return the value, you need
///  to decrement the reference count of the error object created by `error_to_string` as shown above.
///  If you want to propagate the error to q side after some operation, you can just return it (See the
///  example of [`is_error`](fn.is_error.html)).
///
/// # Safety
/// In q, an error is a 0 pointer. This causes a problem of false positive by `error_to_string`, i.e.,
///  `KNULL` is also catched as an error object and its type is set `qtype::ERROR`. In such a case you must NOT
///  return the catched object because it causes segmentation fault. If you want to check if the catched object
///  is an error and then return if it is, you should use [`is_error`](fn.is_error.html). If you want to use the
///  underlying error string of the catched object, you should use [`get_error_string`](trait.KUtility.html#tymethod.get_error_string).
#[inline]
pub unsafe fn error_to_string(error: *const K) -> *const K {
    native::ee(error)
}

/// Judge if a catched object by [`error_to_string`](fn.error_to_string.html) is a genuine error object of type
///  `qtype::ERROR` (This means false positive of the `KNULL` case can be eliminated).
/// # Examples
/// ```no_run
/// use kdbplus::*;
/// use kdbplus::rusty_api::*;
/// use kdbplus::rusty_api::types::*;
///
/// fn love_even(arg: *const K) -> *const K{
///     match KVal::from_raw(arg) {
///         KVal::Int(KData::Atom(&int)) => {
///             if int % 2 == 0{
///                 // Silent for even value
///                 KNULL
///             }
///             else{
///                 // Shout against odd value
///                 new_error("great is the even value!!\0")
///             }
///         }
///         _ => {
///             // Pass through
///             unsafe { increment_reference_count(arg) }
///         }
///     }
/// }
///
/// #[no_mangle]
/// pub extern "C" fn propagate(arg: *const K) -> *const K{
///     let result=unsafe{error_to_string(love_even(arg))};
///     if unsafe{is_error(result)}{
///         // Propagate the error
///         return result;
///     }
///     match KVal::from_raw(result) {
///         KVal::Error(_) => {
///             // KNULL
///             println!("this is KNULL");
///             unsafe{decrement_reference_count(result)};
///             KNULL
///         },
///         _ => new_symbol("sonomama")
///     }
/// }
/// ```
/// ```q
/// q)convey: `libapi_examples 2: (`propagate; 1);
/// q)convey[7i]
/// 'great is the even value!!
/// q)convey[12i]
/// this is KNULL
/// q)convey[5.5]
/// `sonomama
/// ```
/// # Note
/// In this example `KNULL` is used as a returned value of the function called by another function to demonstrate
///  how `is_error` works. However, `KNULL` should not be used in such a way in order to avoid this kind of complexity.
///  To return a general null for inner functions, use [`new_null`](fn.new_null.html) instead.
///
///  # Safety
///  The input must be a valid pointer.
#[inline]
pub unsafe fn is_error(catched: *const K) -> bool {
    !catched.is_null() && (*catched).qtype == qtype::ERROR && !((*catched).value.symbol).is_null()
}

//%% Symbol %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Extract the first `n` chars from a character array and enumerate it internally.
///  This function must be used to add a character array as a symbol value to a symbol list.
///  The returned value is the same character array as the input.
/// # Example
/// See the example of [`flip`](fn.flip.html).
/// # Note
/// The reason why this function must be used is to enumerate the character array before handling
///  it as a q symbol type value. q/kdb+ is enumerating all symbol values to optimize comparison
///  or memory usage. On the other hand [`new_symbol`] does the enumeration internally and
///  therefore it does not need this function.
/// # Safety
/// The input must be a valid pointer.
#[inline]
pub unsafe fn enumerate_n(string: S, n: I) -> S {
    unsafe { native::sn(string, n) }
}

/// Enumerate a null-terminated character array internally. This function must be used
///  to add a character array as a symbol value to a symbol list. The returned value is
///  the same character array as the input.
/// # Example
/// See the example of [`flip`](fn.flip.html).
/// # Note
/// The reason why this function must be used is to enumerate the character array before handling
///  it as a q symbol type value. q/kdb+ is enumerating all symbol values to optimize comparison
///  or memory usage. On the other hand [`new_symbol`] does the enumeration internally and
///  therefore it does not need this function.
/// # Safety
/// The input must be a valid pointer.
#[inline]
pub unsafe fn enumerate(string: S) -> S {
    unsafe { native::ss(string) }
}

//%% Table %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Constructor of q table object from a q dictionary object.
/// # Note
/// Basically this is a `flip` command of q. Hence the value of the dictionary must have
///  lists as its elements.
/// ```no_run
/// use kdbplus::rusty_api::*;
/// use kdbplus::*;
///
/// #[no_mangle]
/// pub extern "C" fn create_table2(_: *const K) -> *const K{
///   // Build keys
///   let keys=new_list(qtype::SYMBOL_LIST, 2).cast_mut();
///   let keys_slice=unsafe{(*keys).as_mut_slice_unchecked::<S>()};
///   keys_slice[0]=unsafe{enumerate(str_to_S!("time"))};
///   keys_slice[1]=unsafe{enumerate_n(str_to_S!("temperature_and_humidity"), 11)};
///   
///   // Build values
///   let values=new_list(qtype::COMPOUND_LIST, 2).cast_mut();
///   let time=new_list(qtype::TIMESTAMP_LIST, 3).cast_mut();
///   // 2003.10.10D02:24:19.167018272 2006.05.24D06:16:49.419710368 2008.08.12D23:12:24.018691392
///   unsafe{(*time).as_mut_slice_unchecked::<J>()}.copy_from_slice(&[119067859167018272_i64, 201766609419710368, 271897944018691392]);
///   let temperature=new_list(qtype::FLOAT_LIST, 3).cast_mut();
///   unsafe{(*temperature).as_mut_slice_unchecked::<F>()}.copy_from_slice(&[22.1_f64, 24.7, 30.5]);
///   unsafe{(*values).as_mut_slice_unchecked::<*mut K>()}.copy_from_slice(&[time, temperature]);
///   
///   unsafe{flip(new_dictionary(keys, values))}
/// }
/// ```
/// ```q
/// q)climate_change: libc_api_examples 2: (`create_table2; 1);
/// q)climate_change[]
/// time                          temperature
/// -----------------------------------------
/// 2003.10.10D02:24:19.167018272 22.1       
/// 2006.05.24D06:16:49.419710368 24.7       
/// 2008.08.12D23:12:24.018691392 30.5    
/// ```
/// # Safety
/// The input must be a valid pointer.
#[inline]
pub unsafe fn flip(dictionary: *const K) -> *const K {
    match unsafe { (*dictionary).qtype } {
        qtype::DICTIONARY => unsafe { native::xT(dictionary) },
        _ => unsafe { native::krr(null_terminated_str_to_const_S("not a dictionary\0")) },
    }
}

/// Constructor of simple q table object from a q keyed table object.
/// # Example
/// ```no_run
/// use kdbplus::rusty_api::*;
/// use kdbplus::rusty_api::types::{KVal, KData};
/// use kdbplus::*;
///
/// #[no_mangle]
/// pub extern "C" fn create_table2(_: *const K) -> *const K{
///   // Build keys
///   let keys = KVal::Symbol(KData::List(std::borrow::Cow::Borrowed(&[
///     unsafe { enumerate(str_to_S!("time")) },
///     unsafe { enumerate_n(str_to_S!("temperature_and_humidity"),11) },
///   ]))).to_k();
///   
///   // Build values
///   let time = KVal::Timestamp(KData::List(
///     std::borrow::Cow::Borrowed(&[119067859167018272_i64, 201766609419710368, 271897944018691392])
///   )).to_k().cast_mut();
///   // 2003.10.10D02:24:19.167018272 2006.05.24D06:16:49.419710368 2008.08.12D23:12:24.018691392
///   let temperature = KVal::Float(KData::List(
///     std::borrow::Cow::Borrowed(&[22.1_f64, 24.7, 30.5])
///   )).to_k().cast_mut();
///
///   let values = KVal::CompoundList(
///     std::borrow::Cow::Borrowed(&[time, temperature])
///   ).to_k();
///   
///   unsafe{flip(new_dictionary(keys, values))}
/// }
///
/// #[no_mangle]
/// pub extern "C" fn create_keyed_table(dummy: *const K) -> *const K{
///   unsafe{enkey(create_table2(dummy), 1)}
/// }
///
/// #[no_mangle]
/// pub extern "C" fn keyed_to_simple_table(dummy: *const K) -> *const K{
///   unsafe{unkey(create_keyed_table(dummy))}
/// }
/// ```
/// ```q
/// q)unkey: libc_api_examples 2: (`keyed_to_simple_table; 1);
/// q)unkey[]
/// time                          temperature
/// -----------------------------------------
/// 2003.10.10D02:24:19.167018272 22.1       
/// 2006.05.24D06:16:49.419710368 24.7       
/// 2008.08.12D23:12:24.018691392 30.5    
/// ```
///
/// # Safety
/// input must be a valid pointer
#[inline]
pub unsafe fn unkey(keyed_table: *const K) -> *const K {
    match unsafe { (*keyed_table).qtype } {
        qtype::DICTIONARY => unsafe { native::ktd(keyed_table) },
        _ => unsafe { native::krr(null_terminated_str_to_const_S("not a keyed table\0")) },
    }
}

/// Constructor of q keyed table object.
/// # Parameters
/// - `table`: q table object to be enkeyed.
/// - `n`: The number of key columns from the left.
/// # Example
/// ```no_run
/// use kdbplus::rusty_api::*;
/// use kdbplus::*;
///
/// #[no_mangle]
/// pub extern "C" fn create_table2(_: *const K) -> *const K{
///   // Build keys
///   let keys=new_list(qtype::SYMBOL_LIST, 2).cast_mut();
///   let keys_slice=unsafe{(*keys).as_mut_slice_unchecked::<S>()};
///   keys_slice[0]=unsafe{enumerate(str_to_S!("time"))};
///   keys_slice[1]=unsafe{enumerate_n(str_to_S!("temperature_and_humidity"), 11)};
///   
///   // Build values
///   let values=new_list(qtype::COMPOUND_LIST, 2).cast_mut();
///   let time=new_list(qtype::TIMESTAMP_LIST, 3).cast_mut();
///   // 2003.10.10D02:24:19.167018272 2006.05.24D06:16:49.419710368 2008.08.12D23:12:24.018691392
///   unsafe{(*time).as_mut_slice_unchecked::<J>()}.copy_from_slice(&[119067859167018272_i64, 201766609419710368, 271897944018691392]);
///   let temperature=new_list(qtype::FLOAT_LIST, 3).cast_mut();
///   unsafe{(*temperature).as_mut_slice_unchecked::<F>()}.copy_from_slice(&[22.1_f64, 24.7, 30.5]);
///   unsafe{(*values).as_mut_slice_unchecked::<*mut K>()}.copy_from_slice(&[time, temperature]);
///   
///   unsafe{flip(new_dictionary(keys, values))}
/// }
///
/// #[no_mangle]
/// pub extern "C" fn create_keyed_table(dummy: *const K) -> *const K{
///   unsafe{enkey(create_table2(dummy), 1)}
/// }
/// ```
/// ```q
/// q)locker: libc_api_examples 2: (`create_keyed_table; 1);
/// q)locker[]
/// time                         | temperature
/// -----------------------------| -----------
/// 2003.10.10D02:24:19.167018272| 22.1       
/// 2006.05.24D06:16:49.419710368| 24.7       
/// 2008.08.12D23:12:24.018691392| 30.5  
/// ```
///
/// # Safety
/// input must be a valid pointer to a q table object
#[inline]
pub unsafe fn enkey(table: *const K, n: J) -> *const K {
    if table.is_null() {
        return native::krr(null_terminated_str_to_const_S("null table\0"));
    }

    match unsafe { (*table).qtype } {
        qtype::TABLE => unsafe { native::knt(n, table) },
        _ => unsafe { native::krr(null_terminated_str_to_const_S("not a table\0")) },
    }
}

//%% Reference Count %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Decrement reference count of the q object. The decrement must be done when `k` function gets an error
///  object whose type is `qtype::ERROR` and when you created an object but do not intend to return it to
///  q side. See details on [the reference page](https://code.kx.com/q/interfaces/c-client-for-q/#managing-memory-and-reference-counting).
/// # Example
/// ```no_run
/// use kdbplus::rusty_api::*;
///
/// #[no_mangle]
/// pub extern "C" fn agriculture(_: *const K) -> *const K {
///   // Produce an apple.
///   let fruit=new_symbol("apple");
///   // Sow the apple seed.
///   unsafe { decrement_reference_count(fruit) };
///   // Return null.
///   KNULL
/// }
/// ```
/// ```q
/// q)do_something: `libapi_examples 2: (`agriculture; 1);
/// q)do_something[]
/// q)
/// ```
///
/// # Safety
/// input must be a valid pointer
#[inline]
pub unsafe fn decrement_reference_count(qobject: *const K) -> V {
    unsafe { native::r0(qobject) }
}

/// Increment reference count of the q object. Increment must be done when you passed arguments
///  to Rust function and intends to return it to q side or when you pass some `K` objects to `k`
///  function and intend to use the argument after the call.
///  See details on [the reference page](https://code.kx.com/q/interfaces/c-client-for-q/#managing-memory-and-reference-counting).
/// # Example
/// ```no_run
/// use kdbplus::rusty_api::*;
/// use kdbplus::*;
///
/// fn eat(apple: *const K){
///   println!("おいしい！");
/// }
///
/// #[no_mangle]
/// pub extern "C" fn satisfy_5000_men(apple: *const K) -> *const K{
///   for _ in 0..10{
///     eat(apple);
///   }
///   unsafe{native::k(0, str_to_S!("eat"), increment_reference_count(apple), KNULL);}
///   unsafe { increment_reference_count(apple) }
/// }
/// ```
/// ```q
/// q)eat:{[apple] show "Collect the clutter of apples!";}
/// q)bread_is_a_sermon: libc_api_examples 2: (`satisfy_5000_men; 1);
/// q)bread_is_a_sermon[`green_apple]
/// おいしい！
/// おいしい！
/// おいしい！
/// おいしい！
/// おいしい！
/// おいしい！
/// おいしい！
/// おいしい！
/// おいしい！
/// おいしい！
/// "Collect the clutter of apples!"
/// ```
///
/// # Safety
/// input must be a valid pointer
#[inline]
pub unsafe fn increment_reference_count(qobject: *const K) -> *const K {
    unsafe { native::r1(qobject) }
}

//%% Callback %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Remove callback from the associated kdb+ socket and call `kclose`.
///  Return null if the socket is invalid or not the one which had been registered by `sd1`.
/// # Note
/// A function which calls this function must be executed at the exit of the process.
#[inline]
pub fn destroy_socket(socket: I) {
    unsafe {
        native::sd0(socket);
    }
}

/// Remove callback from the associated kdb+ socket and call `kclose` if the given condition is satisfied.
///  Return null if the socket is invalid or not the one which had been registered by `sd1`.
/// # Note
/// A function which calls this function must be executed at the exit of the process.
#[inline]
pub fn destroy_socket_if(socket: I, condition: bool) {
    unsafe {
        native::sd0x(socket, condition as I);
    }
}

/// Register callback to the associated kdb+ socket.
/// ```no_run
/// use kdbplus::rusty_api::*;
/// use kdbplus::rusty_api::types::{KVal, KData};
/// use kdbplus::*;
/// use std::borrow::Cow;
///
/// static mut PIPE:[I; 2]=[-1, -1];
///
/// // Callback for some message queue.
/// extern "C" fn callback(socket: I) -> *const K {
///   let mut buffer: [*mut K; 1]=[0 as *mut K];
///   unsafe{libc::read(socket, buffer.as_mut_ptr() as *mut V, 8)};
///   // Call `shout` function on q side with the received data.
///   let result=unsafe { error_to_string(unsafe{native::k(0, str_to_S!("shout"), buffer[0], KNULL)}) };
///   if let KVal::Error(& err_str) = KVal::from(unsafe{&*result}) {
///     eprintln!("Execution error: {}", unsafe { S_to_str(err_str) });
///     unsafe { decrement_reference_count(result) };
///   };
///   KNULL
/// }
///
/// #[no_mangle]
/// pub extern "C" fn plumber(_: *const K) -> *const K {
///     if 0 != unsafe { libc::pipe(PIPE.as_mut_ptr()) } {
///         return new_error("Failed to create pipe\0");
///     }
///     if KNULL == register_callback(unsafe { PIPE[0] }, callback) {
///         return new_error("Failed to register callback\0");
///     }
///     // Lock symbol in a worker thread.
///     pin_symbol();
///     let handle = std::thread::spawn(move || {
///         let precious = KVal::Symbol(KData::List(Cow::from(vec![
///             str_to_S!("belief"),
///             str_to_S!("love"),
///             str_to_S!("hope"),
///         ])))
///         .to_k()
///         .cast_mut();
///         unsafe { libc::write(PIPE[1], std::mem::transmute::<*mut K, *mut V>(precious), 8) };
///     });
///     handle.join().unwrap();
///     unpin_symbol();
///     KNULL
/// }
/// ```
/// ```q
/// q)shout:{[precious] -1 "What are the three largest elements?: ", .Q.s1 precious;};
/// q)fall_into_pipe: `libc_api_example 2: (`plumber; 1);
/// q)fall_into_pipe[]
/// What are the three largest elements?: `belief`love`hope
/// ```
#[inline]
pub fn register_callback(socket: I, function: extern "C" fn(I) -> *const K) -> *const K {
    unsafe { native::sd1(socket, function) }
}

//%% Miscellaneous %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Apply a function to q list object `.[func; args]`.
/// # Example
/// See the example of [`error_to_string`](fn.error_to_string.html).
///
/// # Safety
/// inputs must be valid pointers
#[inline]
pub fn apply(func: *const K, args: *const K) -> *const K {
    apply_unsafe(func, args)
}
/// # Safety
/// inputs must be valid pointers
#[inline(always)]
fn apply_unsafe(func: *const K, args: *const K) -> *const K {
    unsafe { native::dot(func, args) }
}

/// Enable the remote threads to refer to the sym list in the main thread so that enumeration
///  of remotely created symbol values reain valid in the main thread after joining the
///  remote threads. This function must be used before starting any other threads if the
/// threads create symbol values. The previously set value is returned.
/// # Example
/// See the example of [`register_callback`](fn.register_callback.html).
#[inline]
pub fn pin_symbol() -> I {
    unsafe { native::setm(1) }
}

/// Unlock the symbol list in the main thread. This function should be called after joining
///  threads.
/// # Example
/// See the example of [`register_callback`](fn.register_callback.html).
#[inline]
pub fn unpin_symbol() -> I {
    unsafe { native::setm(0) }
}

/// Drop Rust object inside q. Passed as the first element of a foreign object.
/// # Parameters
/// - `obj`: List of (function to free the object; foreign object).
///
/// # Errors
/// - `input is not a list\0`: The input is not a list.
///
/// # Example
/// See the example of [`load_as_q_function`](fn.load_as_q_function.html).
///
/// # Safety
/// inputs must be valid, non-null, pointers to a list of two elements
pub unsafe fn drop_q_object(obj: *const K) -> *const K {
    let Ok(obj_slice) = unsafe { *obj }.as_mut_slice::<*mut K>() else {
        return new_error("input is not a list\0");
    };
    // Take ownership of `K` object from a raw pointer and drop at the end of this scope.
    unsafe { Box::from_raw(obj_slice[1]) };
    // Fill the list with null.
    obj_slice.copy_from_slice(&[KNULL_MUT, KNULL_MUT]);
    obj
}

/// Load C function as a q function (`K` object).
/// # Parameters
/// - `func`: A function takes a C function that would take `n` `K` objects as arguments and returns a `K` object.
/// - `n`: The number of arguments for the function.
/// # Example
/// ```no_run
/// use kdbplus::rusty_api::*;
/// use kdbplus::rusty_api::types::*;
/// use kdbplus::*;
///
/// #[derive(Clone, Debug)]
/// struct Planet{
///     name: String,
///     population: i64,
///     water: bool
/// }
///
/// impl Planet {
///     /// Constructor of `Planet`.
///     fn new(name: &str, population: i64, water: bool) -> Self{
///         Planet{
///             name: name.to_string(),
///             population: population,
///             water: water
///         }
///     }
///
///     /// Description of the planet.
///     fn description(&self)->String{
///         let mut desc=format!("The planet {} is a beautiful planet where {} people reside.", self.name, self.population);
///         if self.water{
///             desc+=" Furthermore water is flowing on the surface of it.";
///         }
///         desc
///     }
/// }
///
/// /// Example of `set_type`.
/// #[no_mangle]
/// pub unsafe extern "C" fn eden(_: *const K) -> *const K{
///     let earth=Planet::new("earth", 7500_000_000, true);
///     let mut foreign=new_list(qtype::COMPOUND_LIST, 2).cast_mut();
///     let foreign_slice=unsafe { (*foreign).as_mut_slice_unchecked::<*mut K>()};
///     foreign_slice[0]=drop_q_object as *mut K;
///     foreign_slice[1]=Box::into_raw(Box::new(earth)) as *mut K;
///     // Set as foreign object.
///     unsafe {(*foreign).qtype = qtype::FOREIGN };
///     foreign
/// }
///
/// extern "C" fn invade(planet: *const K, action: *const K) -> *const K{
///     let obj=unsafe{(*planet.cast_mut()).as_mut_slice_unchecked::<*mut K>()[1] as *const Planet};
///     println!("{:?}", unsafe{obj.as_ref()}.unwrap());
///     let mut desc=unsafe{obj.as_ref()}.unwrap().description();
///     match KVal::from_raw(action) {
///         KVal::Bool(KData::Atom(&b)) => {
///             if b {
///                 desc+=" You shall not curse what God blessed."
///             } else {
///                 desc+=" I perceived I could find favor of God by blessing them.";
///             }
///         }
///         _ => return new_error("input is not a boolean\0"),
///     }
///
///     new_string(&desc)
/// }
///
/// /// Example of `load_as_q_function`.
/// #[no_mangle]
/// pub extern "C" fn probe(planet: *const K) -> *const K {
///     // Return monadic function
///     unsafe{native::k(0, str_to_S!("{[func; planet] func[planet]}"), load_as_q_function(invade as *const V, 2), planet, KNULL)}
/// }
/// ```
/// ```q
/// q)eden: libc_api_example 2: (`eden; 1);
/// q)earth: eden[]
/// q)type earth
/// 112h
/// q)probe: libc_api_example 2: (`probe; 1);
/// q)invade: probe[earth];
/// q)\c 25 200
/// q)invade 1b
/// "The planet earth is a beautiful planet where 7500000000 people reside. Furthermore water is flowing on the surface of it. You shall not curse what God blessed."
/// ```
///
/// # Safety
/// input `func` must be a valid pointer to a C-style function that takes `n` `K` objects as arguments and returns a `K` object.
#[inline]
pub unsafe fn load_as_q_function(func: *const V, n: J) -> *const K {
    unsafe { native::dl(func, n) }
}

/// Convert ymd to the number of days from `2000.01.01`.
/// # Example
/// ```no_run
/// use kdbplus::rusty_api::*;
///
/// let days=ymd_to_days(2020, 4, 1);
/// assert_eq!(days, 7396);
/// ```
#[inline]
pub fn ymd_to_days(year: I, month: I, date: I) -> I {
    unsafe { native::ymd(year, month, date) }
}

/// Convert the number of days from `2000.01.01` to a number expressed as `yyyymmdd`.
/// # Example
/// ```no_run
/// use kdbplus::rusty_api::*;
///
/// let number=days_to_ymd(7396);
/// assert_eq!(number, 20200401);
/// ```
#[inline]
pub fn days_to_ymd(days: I) -> I {
    unsafe { native::dj(days) }
}

/// Convert a simple list to a compound list. Expected usage is to concatinate a simple list
///  with a different type of list.
///
/// returns KNULL if the input is a null pointer,
/// returns a K error if input is not a simple list
///
/// # Example
/// ```no_run
/// use kdbplus::*;
/// use kdbplus::rusty_api::*;
/// use kdbplus::rusty_api::types::{KVal, KData};
/// use std::borrow::Cow;
///
/// #[no_mangle]
/// pub extern "C" fn drift(_: *const K) -> *const K {
///     KVal::CompoundList(Cow::Borrowed(&[
///         KVal::Int(KData::Atom(&12)).to_k().cast_mut(),
///         KVal::Int(KData::Atom(&34)).to_k().cast_mut(),
///         KVal::Symbol(KData::Atom(&str_to_S!("vague")))
///             .to_k()
///             .cast_mut(),
///         KVal::Int(KData::Atom(&-3000)).to_k().cast_mut(),
///     ]))
///     .to_k()
/// }
///
/// #[no_mangle]
/// pub extern "C" fn drift2(_: *const K) -> *const K {
///   let existing_list = KVal::Enum(KData::List(Cow::from(vec![0_i64, 1]))); // error messages returned by 'as_compound_list' are null terminated
///  
///     // Convert a list of enum indices into a compound list while creating enum values from the indices which are tied with
///     //  an existing enum variable named "enum", i.e., Enum indices [0, 1] in the code are cast into `(enum[0]; enum[1])`.
///     let existing_list = match existing_list.to_compound_list(Some("enum")) {
///         Ok(compound) => compound,
///         Err(e_str) => return new_error(e_str),
///     };
///  
///     // another compound list we want to add to the existing list
///     let binding = [
///         to_k!(KVal::Enum(KData::Atom(&2)), "enum2").cast_mut(), // `enum2[2]`.
///         KVal::Month(KData::Atom(&3)).to_k().cast_mut(),
///     ];
///     let other_list = KVal::CompoundList(Cow::Borrowed(&binding));
///  
///     // return the joined list
///     match existing_list.join(other_list) {
///         Ok(joined) => joined.to_k(),
///         Err(e_str) => new_error(e_str),
///     }
/// }
/// ```
/// ```q
/// q)drift: LIBPATH_ (`drift; 1);
/// q)drift2: LIBPATH_ (`drift2; 1);
/// q)drift[]
/// 12i
/// 34i
/// `vague
/// -3000i
/// q)enum: `mashroom`broccoli`cucumber
/// q)enum2: `mackerel`swordfish`tuna
/// q)drift2[]
/// `enum$`mashroom
/// `enum$`broccoli
/// `enum2$`tuna
/// 2000.04m
/// ```
/// # Note
/// - To convert a list provided externally (i.e., passed from a q process), apply
///  [`increment_reference_count`](fn.increment_reference_count.html) before converting the list.
/// - Enum elements from different enum sources must be contained in a compound list. Therefore
///  this function intentionally restricts the number of enum sources to one so that user switches
///  a simple list to a compound list when the second enum sources are provided.
///
///  # Safety
///  input `simple` must be a valid pointer to a K object, we can check if the pointer is null but
///  not if the pointer itself is valid.
pub unsafe fn simple_to_compound(simple: *const K, enum_source: &str) -> *const K {
    // make sure simple is a valid pointer
    if simple.is_null() {
        return KNULL;
    }
    // safe because we previously check if simple is a null pointer
    let simple = unsafe { simple.cast_mut().as_mut() }.unwrap();
    // let simple = unsafe { &mut *simple };

    // make sure simple points to a list of some sort (excluding compound lists)
    if !simple.is_list() {
        return new_error("not a simple list\0");
    }
    // this is safe because we've already checked that simple is a list, and isn't a null pointer
    let size = unsafe { simple.value.list.n } as usize;
    let compound = new_list(qtype::COMPOUND_LIST, size as J);
    let compound_slice = unsafe { *compound }.as_mut_slice::<*mut K>().unwrap();
    match simple.qtype {
        qtype::BOOL_LIST => {
            let simple_slice = simple.as_slice::<G>().unwrap();
            for i in 0..size {
                compound_slice[i] = new_bool(simple_slice[i] != 0) as *mut K;
            }
        }
        qtype::GUID_LIST => {
            let simple_slice = simple.as_slice::<[u8; 16]>().unwrap();
            for i in 0..size {
                compound_slice[i] = new_guid(simple_slice[i]) as *mut K;
            }
        }
        qtype::BYTE_LIST => {
            let simple_slice = simple.as_slice::<G>().unwrap();
            for i in 0..size {
                compound_slice[i] = new_byte(simple_slice[i] as I) as *mut K;
            }
        }
        qtype::SHORT_LIST => {
            let simple_slice = simple.as_slice::<H>().unwrap();
            for i in 0..size {
                compound_slice[i] = new_short(simple_slice[i] as I) as *mut K;
            }
        }
        qtype::INT_LIST => {
            let simple_slice = simple.as_slice::<I>().unwrap();
            for i in 0..size {
                compound_slice[i] = new_int(simple_slice[i]) as *mut K;
            }
        }
        qtype::LONG_LIST => {
            let simple_slice = simple.as_slice::<J>().unwrap();
            for i in 0..size {
                compound_slice[i] = new_long(simple_slice[i]) as *mut K;
            }
        }
        qtype::REAL_LIST => {
            let simple_slice = simple.as_slice::<E>().unwrap();
            for i in 0..size {
                compound_slice[i] = new_real(simple_slice[i] as F) as *mut K;
            }
        }
        qtype::FLOAT_LIST => {
            let simple_slice = simple.as_slice::<F>().unwrap();
            for i in 0..size {
                compound_slice[i] = new_float(simple_slice[i]) as *mut K;
            }
        }
        qtype::STRING => {
            let simple_slice = simple.as_slice::<G>().unwrap();
            for i in 0..size {
                compound_slice[i] = new_char(simple_slice[i] as char) as *mut K;
            }
        }
        qtype::SYMBOL_LIST => {
            let simple_slice = simple.as_slice::<S>().unwrap();
            for i in 0..size {
                compound_slice[i] = new_symbol(S_to_str(simple_slice[i])) as *mut K;
            }
        }
        qtype::TIMESTAMP_LIST => {
            let simple_slice = simple.as_slice::<J>().unwrap();
            for i in 0..size {
                compound_slice[i] = new_timestamp(simple_slice[i]) as *mut K;
            }
        }
        qtype::DATE_LIST => {
            let simple_slice = simple.as_slice::<I>().unwrap();
            for i in 0..size {
                compound_slice[i] = new_date(simple_slice[i]) as *mut K;
            }
        }
        qtype::TIME_LIST => {
            let simple_slice = simple.as_slice::<I>().unwrap();
            for i in 0..size {
                compound_slice[i] = new_time(simple_slice[i]) as *mut K;
            }
        }
        qtype::ENUM_LIST => {
            let simple_slice = simple.as_slice::<J>().unwrap();
            for i in 0..size {
                compound_slice[i] = new_enum(enum_source, simple_slice[i]) as *mut K;
            }
        }
        _ => unimplemented!(),
    }
    // Free simple list
    decrement_reference_count(simple);
    compound
}
