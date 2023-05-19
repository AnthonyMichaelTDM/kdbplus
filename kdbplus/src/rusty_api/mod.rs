pub mod types;

//++++++++++++++++++++++++++++++++++++++++++++++++++//
// >> Load Libraries
//++++++++++++++++++++++++++++++++++++++++++++++++++//

use crate::qtype;
use libc::{c_char, c_double, c_float, c_int, c_longlong, c_schar, c_short, c_uchar, c_void};
use std::str;
pub mod native;
mod utils;
pub use utils::*;
mod re_exports;
pub use re_exports::*;

//++++++++++++++++++++++++++++++++++++++++++++++++++//
// >> Global Variables
//++++++++++++++++++++++++++++++++++++++++++++++++++//

/// `K` nullptr. This value can be used as void value of a function which is called directly by q process
///  and returns `K`. This null pointer is interpreted as a general null value (`::`) whose type is `101h`.
/// # Example
/// ```
/// use kdbplus::rusty_api::*;
///
/// #[no_mangle]
/// pub extern "C" fn vanity(_: *const K) -> *const K{
///   println!("Initialized something, probably it is your mindset.");
///   KNULL
/// }
/// ```
/// # Warning
/// This value must NOT be used as a returned value for functions called by another function
///  because [`error_to_string`](fn.error_to_string.html) misunderstands the value as an error.
///  For detail, see its warning section.
pub const KNULL: *const K = std::ptr::null::<K>();
/// same as `KNULL` but mutable.
pub const KNULL_MUT: *mut K = std::ptr::null_mut::<K>();

//++++++++++++++++++++++++++++++++++++++++++++++++++//
// >> Traits
//++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% SafeToCastFromKInner %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Trait for types that can be safely cast from the `inner` field of a K object.
/// should not be implemented by user code.
pub trait SafeToCastFromKInner {
    // because this function takes ownership of inner, it is safe to cast it to a mutable reference
    fn cast<'a>(inner: k_inner) -> &'a mut Self;
    // because this function takes ownership of inner, it is safe to cast it to a mutable reference
    fn cast_with_ptr_offset<'a>(inner: k_inner) -> &'a mut Self;
}

macro_rules! impl_safe_cast_for {
    ($t:ty) => {
        impl SafeToCastFromKInner for $t {
            #[inline]
            fn cast<'a>(inner: k_inner) -> &'a mut Self {
                // get a pointer to the start of the block of memory used by the union
                let ptr = unsafe { &inner.byte_array as *const u8 };
                if ptr.is_null() {
                    unimplemented!()
                }
                // cast the pointer to a pointer of the correct type, then dereference it
                let ptr = ptr as *mut Self;
                unsafe { &mut *ptr }
            }

            #[inline]
            fn cast_with_ptr_offset<'a>(inner: k_inner) -> &'a mut Self {
                // get a pointer to the start of the block of memory used by the union
                let ptr = unsafe { &inner.byte_array as *const u8 };
                if ptr.is_null() {
                    unimplemented!()
                }
                // cast the pointer to a pointer of the correct type
                let ptr = unsafe { ptr.add(std::mem::size_of::<usize>()) } as *mut Self;
                // then dereference it
                unsafe { &mut *ptr }
                // unsafe { &mut *((ptr as *const usize).offset(1) as *mut Self) }
            }
        }
    };
}

#[cfg(test)]
mod cast_sanity_checks {
    use super::k_inner;

    #[test]
    fn test_k_inner_casting_methods() {
        for i in 0..=10 {
            let inner = k_inner { int: i };

            let cast = {
                let ptr = unsafe { &inner.byte_array as *const u8 };
                let ptr = ptr as *mut i32;
                unsafe { &mut *ptr }
            };

            assert_eq!(i, *cast);
        }
    }
}

//++++++++++++++++++++++++++++++++++++++++++++++++++//
// >> Structs
//++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% Alias %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// `char*` in C. Also used to access symbol of q.
pub type S = *mut c_char;
/// `const char*` in C.
#[allow(non_camel_case_types)]
pub type const_S = *const c_char;
/// `char` in C. Also used to access char of q.
pub type C = c_char;
/// `unsigned char` in C. Also used to access byte of q.
pub type G = c_uchar;
/// `i16` in C. Also used to access short of q.
pub type H = c_short;
/// `i32` in C. Also used to access int and compatible types (month, date, minute, second and time) of q.
pub type I = c_int;
/// `i64` in C. Also used to access long and compatible types (timestamp and timespan) of q.
pub type J = c_longlong;
/// `f32` in C. Also used to access real of q.
pub type E = c_float;
/// `f64` in C. Also used to access float and datetime of q.
pub type F = c_double;
/// `void` in C.
pub type V = c_void;

//%% U %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Struct representing 16-bytes GUID.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct U {
    pub guid: [G; 16],
}

impl U {
    /// Create 16-byte GUID object.
    pub fn new(guid: [u8; 16]) -> Self {
        U { guid }
    }
}

//%% K %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Underlying list value of q object.
/// # Note
/// Usually this struct does not need to be accessed this struct directly unless user wants to
///  access via a raw pointer for non-trivial stuff.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct KList {
    /// Length of the list.
    pub n: J,
    /// Pointer referring to the head of the list. This pointer will be interpreted
    ///  as various types when accessing `K` object to edit the list with
    ///  `as_mut_slice`
    pub g0: [G; 1], // TODO: shouldn't this be a pointer, like *mut u8?
}

/// Underlying atom value of q object.
/// # Note
/// Usually this struct does not need to be accessed directly unless user wants to
///  access via a raw pointer for non-trivial stuff.
#[derive(Clone, Copy)]
#[repr(C)]
pub union k_inner {
    // a union is a continuous block of memory as big as the biggest member,
    // reading from any member will read from that memory block as if it was that type
    /// Byte type holder.
    pub byte: G,
    /// Short type holder.
    pub short: H,
    /// Int type holder.
    pub int: I,
    /// Long type older.
    pub long: J,
    /// Real type holder.
    pub real: E,
    /// Float type holder.
    pub float: F,
    /// Symbol type holder.
    pub symbol: S,
    /// Table type holder.
    pub table: *mut K,
    /// List type holder.
    pub list: KList,
    /// utility for accessing raw bytes
    pub byte_array: [u8; 16],
}

impl_safe_cast_for!(G);
impl_safe_cast_for!(H);
impl_safe_cast_for!(I);
impl_safe_cast_for!(J);
impl_safe_cast_for!(E);
impl_safe_cast_for!(F);
impl_safe_cast_for!(S);
impl_safe_cast_for!(*mut K);
impl_safe_cast_for!(KList);
impl_safe_cast_for!([u8; 16]);
impl_safe_cast_for!(bool);

/// Underlying struct of raw `K` object.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct K {
    /// For internal usage.
    m: c_schar,
    /// For internal usage.
    a: c_schar,
    /// Type indicator.
    pub qtype: c_schar,
    /// Attribute of list.
    pub attribute: C,
    /// Reference count of the object.
    pub refcount: I,
    /// Underlying value.
    pub value: k_inner,
}
// these are accessors for the (untagged) union represented by value
impl K {
    #[inline]
    pub fn cast<'a, T: SafeToCastFromKInner>(&self) -> &'a mut T {
        T::cast(self.value)
    }

    #[inline]
    pub fn cast_with_ptr_offset<'a, T: SafeToCastFromKInner>(&self) -> &'a mut T {
        T::cast_with_ptr_offset(self.value)
    }

    #[inline]
    /// Derefer `K` as a mutable slice of the specified type. The supported types are:
    /// - `K` (*mut K): Equivalent to C API macro `kK`.
    /// - `G` (c_uchar): Equivalent to C API macro `kG`.
    /// - `H`: Equivalent to C API macro `kH`.
    /// - `I`: Equivalent to C API macro `kI`.
    /// - `J`: Equivalent to C API macro `kJ`.
    /// - `E`: Equivalent to C API macro `kE`.
    /// - `F`: Equivalent to C API macro `kF`.
    /// - `C`: Equivalent to C API macro `kC`.
    /// - `S`: Equivalent to C API macro `kS`.
    ///
    /// # Safety
    /// should be safe as long as the K object is a list coming from a q process or api call
    /// however, as long as self.value is a list of supported types, this should be fine.
    pub unsafe fn as_mut_slice_unchecked<'a, T: 'a>(&mut self) -> &'a mut [T] {
        std::slice::from_raw_parts_mut(
            self.value.list.g0.as_mut_ptr() as *mut T,
            self.value.list.n as usize,
        )
    }

    #[inline]
    /// same as as_mut_slice_unchecked, but returned slice is not mutable
    ///
    /// # Safety
    /// should be safe as long as the K object is a list coming from a q process or api call
    /// however, as long as self.value is a list of supported types, this should be fine.
    pub unsafe fn as_slice_unchecked<'a, T: 'a>(&self) -> &'a [T] {
        std::slice::from_raw_parts(
            self.value.list.g0.as_ptr() as *const T,
            self.value.list.n as usize,
        )
    }

    #[inline]
    /// Derefer `K` as a mutable slice of the specified type. The supported types are:
    /// - `K` (* K): Equivalent to C API macro `kK`.
    /// - `G` (c_uchar): Equivalent to C API macro `kG`.
    /// - `H`: Equivalent to C API macro `kH`.
    /// - `I`: Equivalent to C API macro `kI`.
    /// - `J`: Equivalent to C API macro `kJ`.
    /// - `E`: Equivalent to C API macro `kE`.
    /// - `F`: Equivalent to C API macro `kF`.
    /// - `C`: Equivalent to C API macro `kC`.
    /// - `S`: Equivalent to C API macro `kS`.
    pub fn as_mut_slice<'a, T: 'a + SafeToCastFromKInner>(
        &mut self,
    ) -> Result<&'a mut [T], &'a str> {
        // is this a list?
        match self.qtype {
            qtype::COMPOUND_LIST..=qtype::ENUM_LIST | qtype::DICTIONARY | qtype::TABLE => {
                // yes, slice it up
                Ok(unsafe { self.as_mut_slice_unchecked() })
            }
            _ => Err("not a list"),
        }
    }

    #[inline]
    /// same as as_mut_slice, but returned slice is not mutable
    pub fn as_slice<'a, T: 'a + SafeToCastFromKInner>(&self) -> Result<&'a [T], &'a str> {
        // is this a list?
        match self.qtype {
            qtype::COMPOUND_LIST..=qtype::ENUM_LIST | qtype::DICTIONARY | qtype::TABLE => {
                // yes, slice it up
                Ok(unsafe { self.as_slice_unchecked() })
            }
            _ => Err("not a list"),
        }
    }

    #[inline]
    /// is this K an atom?
    ///
    /// # Note
    /// is_atom != !is_list
    ///
    /// excludes chars
    ///
    pub fn is_atom(&self) -> bool {
        self.qtype < 0 && self.qtype >= -20 && self.qtype != qtype::CHAR
    }

    #[inline]
    /// is this K a simple list?
    ///
    /// # Note
    /// is_atom != !is_list
    /// excludes strings, compound lists, dictionaries, and tables
    pub fn is_list(&self) -> bool {
        self.qtype > 0 && self.qtype <= 20 && self.qtype != qtype::STRING
    }
}
