use std::ffi::CStr;

use super::{const_S, S};

//++++++++++++++++++++++++++++++++++++++++++++++++++//
// >> Utility
//++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% Utility %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Convert `&str` to `S` (null-terminated character array).
/// # Example
/// ```no_run
/// use kdbplus::rusty_api::*;
/// use kdbplus::str_to_S;
///
/// #[no_mangle]
/// pub extern "C" fn pingpong(_: K) -> K{
///   unsafe{native::k(0, str_to_S!("ping"), new_int(77), KNULL)}
/// }
/// ```
/// ```q
/// q)ping:{[int] `$string[int], "_pong!!"}
/// q)pingpong: `libapi_examples 2: (`pingpong; 1);
/// q)pingpong[]
/// `77_pong!!
/// ```
/// # Note
/// This macro cannot be created as a function due to freeing resource of Rust (not sure).
#[macro_export]
macro_rules! str_to_S {
    ($string: expr) => {
        [$string.as_bytes(), &[b'\0']].concat().as_ptr() as S
    };
}

/// Convert `S` to `&str`. This function is intended to convert symbol type (null-terminated char-array) to `str`.
/// # Example
/// ```no_run
/// use kdbplus::*;
/// use kdbplus::rusty_api::*;
///
/// #[no_mangle]
/// pub extern "C" fn print_symbol(symbol: *const K) -> *const K{
///   unsafe{
///     if (*symbol).qtype == qtype::SYMBOL_ATOM{
///       println!("symbol: `{}", S_to_str((*symbol).value.symbol));
///     }
///     // return null
///     KNULL
///   }
/// }
/// ```
/// ```q
/// q)print_symbol:`libapi_examples 2: (`print_symbol; 1)
/// q)a:`kx
/// q)print_symbol a
/// symbol: `kx
/// ```
///
/// # Safety
/// * The memory pointed to by `ptr` must contain a valid nul terminator at the
///  end of the string.
///
/// * `ptr` must be [valid](core::ptr#safety) for reads of bytes up to and including the null terminator.
///  This means in particular:
///
///  * The entire memory range of this `CStr` must be contained within a single allocated object!
///  * `ptr` must be non-null even for a zero-length cstr.
/// * The memory referenced by the returned `CStr` must not be mutated for
///   the duration of lifetime `'a`.
#[inline]
#[allow(non_snake_case)]
pub unsafe fn S_to_str<'a>(cstring: S) -> &'a str {
    unsafe { CStr::from_ptr(cstring) }.to_str().unwrap()
}

/// Convert null-terminated `&str` to `S`.
/// # Example
/// ```no_run
/// use kdbplus::rusty_api::*;
///
/// #[no_mangle]
/// pub extern "C" fn pingpong2(_: *const K) -> *const K{
///   unsafe{native::k(0, null_terminated_str_to_S("ping\0"), new_int(77), KNULL)}
/// }
/// ```
/// ```q
/// q)ping:{[int] `$string[int], "_pong!!"};
/// q)pingpong: `libapi_examples 2: (`pingpong2; 1);
/// q)pingpong[]
/// `77_pong!!
/// ```
///
/// # Safety
/// input must be:
/// - null terminated
/// - no null bytes in the middle
///
#[inline]
#[allow(non_snake_case)]
pub fn null_terminated_str_to_S(string: &str) -> S {
    unsafe { CStr::from_bytes_with_nul_unchecked(string.as_bytes()).as_ptr() as S }
}

/// Convert null terminated `&str` into `const_S`. Expected usage is to build
///  a q error object with `krr`.
/// # Example
/// ```no_run
/// use kdbplus::rusty_api::*;
/// use kdbplus::rusty_api::native::*;
/// use kdbplus::qtype;
///
/// pub extern "C" fn must_be_int2(obj: *const K) -> *const K{
///   unsafe{
///     if (*obj).qtype != qtype::INT_ATOM{
///       krr(null_terminated_str_to_const_S("not an int\0"))
///     }
///     else{
///       KNULL
///     }
///   }
/// }
/// ```
/// ```q
/// q)check:`libapi_examples 2: (`must_be_int; 1)
/// q)a:100
/// q)check a
/// 'not an int
///   [0]  check a
///        ^
/// q)a:42i
/// q)check a
/// ```
#[inline]
#[allow(non_snake_case)]
pub fn null_terminated_str_to_const_S(string: &str) -> const_S {
    string.as_bytes().as_ptr() as const_S
}
