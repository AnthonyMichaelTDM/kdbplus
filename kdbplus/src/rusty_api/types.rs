//++++++++++++++++++++++++++++++++++++++++++++++++++//
// >> Structs
//++++++++++++++++++++++++++++++++++++++++++++++++++//

use super::{re_exports, SafeToCastFromKInner, K, KNULL, S};
use crate::qtype;

/// Rust friendly wrapper for q Atoms and Lists.
/// references are mutable to indicate that changes should propagate back to q.
#[derive(Debug)]
pub enum KData<'a, T> {
    Atom(&'a mut T),   // TODO: Should this be mut, const, or neither?
    List(&'a mut [T]), // TODO: Should this be mut, const, or neither?
}

impl<'a, T: 'a + std::fmt::Debug + SafeToCastFromKInner> KData<'a, T> {
    #[inline]
    /// # Safety
    /// k must be a valid pointer to a valid K object
    fn atom(k: &'a mut K) -> KData<'a, T> {
        KData::Atom(k.cast())
    }

    #[inline]
    /// # Safety
    /// k must be a valid pointer to a valid K object
    fn guid_atom(k: &'a mut K) -> KData<'a, T> {
        KData::Atom(k.cast_with_ptr_offset()) // while this is an atom, it is packed into a list of 1
    }
    #[inline]
    /// # Safety
    /// same as [`K::as_slice`](type.K.html#method.as_slice)
    /// but, additionally k must be a list of type T
    fn list(k: &'a mut K) -> KData<'a, T> {
        KData::List(k.as_mut_slice().unwrap())
    }
}

/// intuitive rust wrappers for q types, allowing for idiomatic rust code
/// that can take full advantage of rust's powerful pattern matching and type system
/// when interacting with q.
pub enum KVal<'a> {
    // by doing it this way, we can use the same enum for both atoms and lists
    /// Slice of pointers to other K objects
    CompoundList(&'a mut [*mut K]),
    /// Note: the C api uses [`I`] (i32) for booleans. we do too to leave as much control in the
    /// implementors hands in Rust.
    Bool(KData<'a, i32>),
    /// Note: the C api uses \[[`G`]; 16\] (c_uchar) for guids. we use [u8; 16] in Rust.
    Guid(KData<'a, [u8; 16]>),
    /// Note: the C api uses [`I`] (i32) for bytes. we use u8 in Rust.
    Byte(KData<'a, u8>),
    /// Note: the C api uses [`I`] (i32) for shorts. we use i16 in Rust.
    Short(KData<'a, i16>),
    /// Note: the C api uses [`I`] (i32) for ints. we use i32 in Rust.
    Int(KData<'a, i32>),
    /// Note: the C api uses [`J`] (i64) for longs. we use i64 in Rust.
    Long(KData<'a, i64>),
    /// Note: the C api uses [`F`] (f64) for reals. we use f32 in Rust.
    Real(KData<'a, f32>),
    /// Note: the C api uses [`F`] (f64) for floats. we use f64 in Rust.
    Float(KData<'a, f64>),
    /// Note: the C api uses [`I`] (i32) for chars. we use u8 (c_uchar) in Rust https://github.com/KxSystems/kdb/blob/bbc40b8cb870948122a36cb80a486bc5f7e470d7/c/c/k.h#L29.
    Char(&'a mut u8),
    /// Note: the C api uses [`S`] (*mut c_char) for symbols. we use the same in Rust to avoid unecessary unsafety.
    Symbol(KData<'a, S>),
    /// Note: the C api uses [`J`] (i64) for timestamps. we use i64 in Rust.
    Timestamp(KData<'a, i64>),
    /// Note: the C api uses [`I`] (i32) for months. we use i32 in Rust.
    Month(KData<'a, i32>),
    /// Note: the C api uses [`I`] (i32) for dates. we use i32 in Rust.
    Date(KData<'a, i32>),
    /// Note: the C api uses [`F`] (f64) for datetimes. we use f64 in Rust.
    Datetime(KData<'a, f64>),
    /// Note: the C api uses [`J`] (i64) for timespans. we use i64 in Rust.
    Timespan(KData<'a, i64>),
    /// Note: the C api uses [`I`] (i32) for minute. we use i32 in Rust.
    Minute(KData<'a, i32>),
    /// Note: the C api uses [`I`] (i32) for3 second. we use i32 in Rust.
    Second(KData<'a, i32>),
    /// Note: the C api uses [`I`] (i32) for time. we use i32 in Rust.
    Time(KData<'a, i32>),
    // TODO: Enum
    /// Note: the C api uses [`S`] (*mut c_char) for strings. we will too for added flexibility in Rust.
    String(&'a mut S),
    // TODO: Foreign
    // TODO: Dictionary
    // TODO: Sorted Dictionary
    // TODO: Table
    /// q Error, created by krr or orr
    Err(&'a mut S),
    /// the q-equivalent value of null depends on a great many factors.
    Null,
}

impl<'a> KVal<'a> {
    /// Create a new KVal from a const pointer to a [`K`](type.K.html) value.
    ///
    /// # Examples
    /// ```
    /// use kdbplus::rusty_api::K;
    /// use kdbplus::rusty_api::types::KVal;
    /// use kdbplus::rusty_api::native;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn plus_one_int(k: *const K) -> *const K {
    ///     let addr = catch_unwind(|| match KVal::from_raw(k).unwrap()) {
    ///        KVal::Int(KData::Atom(i)) => KVal::Int(KData::Atom(i + 1)),
    ///        _ => unsafe {native::new_error("type error\0")},
    ///     }).or_else::<u8, _>(|_| Ok(native::new_error("rust panic\0")))
    ///     .unwrap()
    /// }
    ///
    ///# #[test]
    ///# fn test_plus_one_int() {
    ///     let k = native::ki(1);
    ///     let k = unsafe { plus_one_int(&k) };
    ///     assert_eq!(k, native::ki(2));
    ///# }
    /// ```
    ///
    /// # Safety
    /// parameter 'k' must be a valid pointer to a [`K`](type.K.html) value.
    /// if 'k' is null, the returned value will be [`KVal::Null`](enum.KVal.html#variant.Null).
    pub unsafe fn from_raw(k: *const K) -> KVal<'a> {
        if k.is_null() {
            return KVal::Null;
        }
        Self::new(unsafe { &mut *(k.cast_mut()) })
    }

    /// Create a new KVal from a reference to a [`K`](type.K.html) value.
    ///
    /// # Examples
    /// TODO: test this example, make sure rust doesn't take ownership of the value
    /// ```no_run
    /// use kdbplus::rusty_api::K;
    /// use kdbplus::rusty_api::types::{KVal, KData};
    /// use kdbplus::rusty_api::*;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn plus_one_int(k: *const K) -> *const K {
    ///     // assuming k is a non-null, and valid, pointer to a K value
    ///     std::panic::catch_unwind(move || {
    ///         let KVal::Int(KData::Atom(mut value)) = KVal::new(unsafe{&*k}) else {
    ///             return new_error("type error\0");
    ///         };
    ///
    ///         *value += 1;
    ///
    ///         k
    ///     }).or_else::<u8, _>(|_| Ok(new_error("rust panic\0")))
    ///     .unwrap()
    /// }
    /// ```
    ///
    /// # Note
    /// while calling it `new` does imply that there is an allocation involved, this should be a
    /// zero-copy wrapper that directly references the internals of the given `k`,
    /// TODO: rename this to avoid confusion (maybe just imple the Into trait for these?)
    ///
    /// # Safety
    /// The value of `k` must be correct for it's qtype.
    /// As long as `k` comes from q or the C q api, this will be safe.
    ///
    pub fn new(k: &'a mut K) -> KVal<'a> {
        match k.qtype {
            /* -128 */ qtype::ERROR => KVal::Err(k.cast()),
            /* -20  */ qtype::ENUM_ATOM => todo!(),
            /* -19  */ qtype::TIME_ATOM => KVal::Time(KData::atom(k)),
            /* -18  */ qtype::SECOND_ATOM => KVal::Second(KData::atom(k)),
            /* -17  */ qtype::MINUTE_ATOM => KVal::Minute(KData::atom(k)),
            /* -16  */ qtype::TIMESPAN_ATOM => KVal::Timespan(KData::atom(k)),
            /* -15  */ qtype::DATETIME_ATOM => KVal::Datetime(KData::atom(k)),
            /* -14  */ qtype::DATE_ATOM => KVal::Date(KData::atom(k)),
            /* -13  */ qtype::MONTH_ATOM => KVal::Month(KData::atom(k)),
            /* -12  */ qtype::TIMESTAMP_ATOM => KVal::Timestamp(KData::atom(k)),
            /* -11  */ qtype::SYMBOL_ATOM => KVal::Symbol(KData::atom(k)),
            /* -10  */ qtype::CHAR => KVal::Char(k.cast()),
            /* -9   */ qtype::FLOAT_ATOM => KVal::Float(KData::atom(k)),
            /* -8   */ qtype::REAL_ATOM => KVal::Real(KData::atom(k)),
            /* -7   */ qtype::LONG_ATOM => KVal::Long(KData::atom(k)),
            /* -6   */ qtype::INT_ATOM => KVal::Int(KData::atom(k)),
            /* -5   */ qtype::SHORT_ATOM => KVal::Short(KData::atom(k)),
            /* -4   */ qtype::BYTE_ATOM => KVal::Byte(KData::atom(k)),
            /* -2   */ qtype::GUID_ATOM => KVal::Guid(KData::guid_atom(k)),
            /* -1   */ qtype::BOOL_ATOM => KVal::Bool(KData::atom(k)),
            /* 0    */
            qtype::COMPOUND_LIST => KVal::CompoundList(k.as_mut_slice::<*mut K>().unwrap()),
            /* 1    */ qtype::BOOL_LIST => KVal::Bool(KData::list(k)),
            /* 2    */ qtype::GUID_LIST => KVal::Guid(KData::list(k)),
            /* 4    */ qtype::BYTE_LIST => KVal::Byte(KData::list(k)),
            /* 5    */ qtype::SHORT_LIST => KVal::Short(KData::list(k)),
            /* 6    */ qtype::INT_LIST => KVal::Int(KData::list(k)),
            /* 7    */ qtype::LONG_LIST => KVal::Long(KData::list(k)),
            /* 8    */ qtype::REAL_LIST => KVal::Real(KData::list(k)),
            /* 9    */ qtype::FLOAT_LIST => KVal::Float(KData::list(k)),
            /* 10   */ qtype::STRING => KVal::String(k.cast()),
            /* 11   */ qtype::SYMBOL_LIST => KVal::Symbol(KData::list(k)),
            /* 12   */ qtype::TIMESTAMP_LIST => KVal::Timestamp(KData::list(k)),
            /* 13   */ qtype::MONTH_LIST => KVal::Month(KData::list(k)),
            /* 14   */ qtype::DATE_LIST => KVal::Date(KData::list(k)),
            /* 15   */ qtype::DATETIME_LIST => KVal::Datetime(KData::list(k)),
            /* 16   */ qtype::TIMESPAN_LIST => KVal::Timespan(KData::list(k)),
            /* 17   */ qtype::MINUTE_LIST => KVal::Minute(KData::list(k)),
            /* 18   */ qtype::SECOND_LIST => KVal::Second(KData::list(k)),
            /* 19   */ qtype::TIME_LIST => KVal::Time(KData::list(k)),
            /* 20   */ qtype::ENUM_LIST => todo!(),
            /* 99   */ qtype::TABLE => todo!(),
            /* 101  */ qtype::DICTIONARY => todo!(),
            /* 112  */ qtype::FOREIGN => todo!(),
            /* 127  */
            qtype::SORTED_DICTIONARY => todo!(), // probably reuse the dictionary type

            _ => KVal::Null,
        }
    }

    // TODO: add a method to convert back to a K value
    /// Convert this value back into a K value,
    /// though technically, because KVal operates on references, changes should propagate TODO: Unsure of this
    ///
    /// # Note
    /// * uses methods from the native q api to create NEW K objects from the data in this value.
    /// * consumes self, this is deliberate as we don't want multiple references to the same data
    ///
    /// TODO: support zero-copy conversion (idea: store the original K object and modify that to create a new one?)
    pub fn to_k(self) -> *const K {
        match self {
            KVal::CompoundList(list) => {
                let k = re_exports::new_list(qtype::COMPOUND_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                // copy the elements over
                unsafe { &mut *k }
                    .as_mut_slice::<*mut K>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Bool(KData::Atom(&mut atom)) => re_exports::new_bool(atom),
            KVal::Bool(KData::List(list)) => {
                let k = re_exports::new_list(qtype::BOOL_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<i32>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Guid(KData::Atom(&mut atom)) => re_exports::new_guid(atom),
            KVal::Guid(KData::List(list)) => {
                let k = re_exports::new_list(qtype::GUID_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<[u8; 16]>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Byte(KData::Atom(&mut atom)) => re_exports::new_byte(atom.into()),
            KVal::Byte(KData::List(list)) => {
                let k = re_exports::new_list(qtype::BYTE_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<u8>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Short(KData::Atom(&mut atom)) => re_exports::new_short(atom.into()),
            KVal::Short(KData::List(list)) => {
                let k = re_exports::new_list(qtype::SHORT_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<i16>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Int(KData::Atom(&mut atom)) => re_exports::new_int(atom),
            KVal::Int(KData::List(list)) => {
                let k = re_exports::new_list(qtype::INT_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<i32>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Long(KData::Atom(&mut atom)) => re_exports::new_long(atom),
            KVal::Long(KData::List(list)) => {
                let k = re_exports::new_list(qtype::LONG_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<i64>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Real(KData::Atom(&mut atom)) => re_exports::new_real(atom.into()),
            KVal::Real(KData::List(list)) => {
                let k = re_exports::new_list(qtype::REAL_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<f32>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Float(KData::Atom(&mut atom)) => re_exports::new_float(atom),
            KVal::Float(KData::List(list)) => {
                let k = re_exports::new_list(qtype::FLOAT_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<f64>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Symbol(KData::Atom(&mut atom)) => re_exports::new_symbol_from_S(atom),
            KVal::Symbol(KData::List(list)) => {
                let k = re_exports::new_list(qtype::SYMBOL_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<S>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Timestamp(KData::Atom(&mut atom)) => re_exports::new_timestamp(atom),
            KVal::Timestamp(KData::List(list)) => {
                let k = re_exports::new_list(qtype::TIMESTAMP_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<i64>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Month(KData::Atom(&mut atom)) => re_exports::new_month(atom),
            KVal::Month(KData::List(list)) => {
                let k = re_exports::new_list(qtype::MONTH_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<i32>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Date(KData::Atom(&mut atom)) => re_exports::new_date(atom),
            KVal::Date(KData::List(list)) => {
                let k = re_exports::new_list(qtype::DATE_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<i32>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Datetime(KData::Atom(&mut atom)) => re_exports::new_datetime(atom),
            KVal::Datetime(KData::List(list)) => {
                let k = re_exports::new_list(qtype::DATETIME_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<f64>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Timespan(KData::Atom(&mut atom)) => re_exports::new_timespan(atom),
            KVal::Timespan(KData::List(list)) => {
                let k = re_exports::new_list(qtype::TIMESPAN_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<i64>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Minute(KData::Atom(&mut atom)) => re_exports::new_minute(atom),
            KVal::Minute(KData::List(list)) => {
                let k = re_exports::new_list(qtype::MINUTE_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<i32>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Second(KData::Atom(&mut atom)) => re_exports::new_second(atom),
            KVal::Second(KData::List(list)) => {
                let k = re_exports::new_list(qtype::SECOND_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<i32>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Time(KData::Atom(&mut atom)) => re_exports::new_time(atom),
            KVal::Time(KData::List(list)) => {
                let k = re_exports::new_list(qtype::TIME_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<i32>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Char(&mut atom) => re_exports::new_char(atom as char),
            KVal::String(&mut list) => re_exports::new_string_from_S(list),
            KVal::Err(&mut err) => re_exports::new_error_from_S(err),
            KVal::Null => KNULL,
        }
    }
}
