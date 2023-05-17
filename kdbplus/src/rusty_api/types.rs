//++++++++++++++++++++++++++++++++++++++++++++++++++//
// >> Structs
//++++++++++++++++++++++++++++++++++++++++++++++++++//

use super::{re_exports, SafeToCastFromKInner, K, KNULL, S};
use crate::qtype;

/// Rust friendly wrapper for q Atoms and Lists.
/// references are mutable to indicate that changes should propagate back to q.
#[derive(Debug)]
pub enum KData<'a, T> {
    Atom(&'a T),   // TODO: Should this be mut, const, or neither?
    List(&'a [T]), // TODO: Should this be mut, const, or neither?
}

impl<'a, T: 'a + std::fmt::Debug + SafeToCastFromKInner> KData<'a, T> {
    #[inline]
    /// # Safety
    /// k must be a valid pointer to a valid K object
    fn atom(k: &'a K) -> KData<'a, T> {
        KData::Atom(k.cast())
    }

    #[inline]
    /// # Safety
    /// k must be a valid pointer to a valid K object
    fn guid_atom(k: &'a K) -> KData<'a, T> {
        KData::Atom(k.cast_with_ptr_offset()) // while this is an atom, it is packed into a list of 1
    }
    #[inline]
    /// # Safety
    /// same as [`K::as_slice`](type.K.html#method.as_slice)
    /// but, additionally k must be a list of type T
    fn list(k: &'a K) -> KData<'a, T> {
        KData::List(k.as_slice().unwrap())
    }
}

/// intuitive rust wrappers for q types, allowing for idiomatic rust code
/// that can take full advantage of rust's powerful pattern matching and type system
/// when interacting with q.
pub enum KVal<'a> {
    // by doing it this way, we can use the same enum for both atoms and lists
    /// Slice of pointers to other K objects
    CompoundList(Vec<*mut K>),
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
    Char(&'a u8),
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
    /// # Note
    /// * the C api uses [`J`] (i64) for enumeration. we use i64 in Rust.
    /// * this implementation assumes that an Enum is just an index, as that's how it's used in the
    ///   [`api`] module.
    /// * if your KVal is Enum Atom, you'll need to use [`to_k!(kval, enum_src)`](to_k) to convert
    ///   it to a K object. [`to_k`](to_k) will panic if you try to convert a enum atom.
    Enum(KData<'a, i64>),
    /// Note: the C api uses [`S`] (*mut c_char) for strings. we will too for added flexibility in Rust.
    String(&'a S),
    // TODO: Foreign
    // TODO: Dictionary
    // TODO: Sorted Dictionary
    // TODO: Table
    /// q Error, created by krr or orr
    Err(&'a S),
    /// the q-equivalent value of null depends on a great many factors.
    Null,
}

impl<'a> From<&'a K> for KVal<'a> {
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
    /// pub unsafe extern "C" fn plus_one_int(k: *const K) -> *const K {
    ///     // assuming k is a non-null, and valid, pointer to a K value
    ///     std::panic::catch_unwind(move || {
    ///         let KVal::Int(KData::Atom(value)) = KVal::from(unsafe{&*k}) else {
    ///             return new_error("type error\0");
    ///         };
    ///         new_int(value + 1)
    ///     })
    ///     .or_else::<u8, _>(|_| Ok(new_error("rust panic\0")))
    ///     .unwrap()
    /// }
    /// ```
    ///
    /// # Note
    /// the passed reference is mutable to both indicate that changes to the resulting KVal will
    /// propagate (if they don't, please opend an issue), and to tell the borrow checker not to let
    /// people use the k value after it's been passed to this function, changes propagate so it's
    /// unsafe to use the value after it's been passed to this function.
    ///
    /// # Safety
    /// * The value of `k` must be correct for it's qtype.
    /// * This function takes a reference so it's up to implementors to correctly dereference the
    ///   pointer passed by q and returned by the C api's functions.
    /// * don't use `k` after it's been passed to this function unless you know exactly
    ///   what you're doing,
    ///   * treat k as if it's been moved into this function, because it effectively has been.
    ///   * you should not try to mutate the value of the returned KVal as a way to change the
    ///     underlying k object, q is a functional language so functions should not have side effects.
    fn from(k: &'a K) -> KVal<'a> {
        match k.qtype {
            /* -128 */ qtype::ERROR => KVal::Err(k.cast()),
            /* -20  */ qtype::ENUM_ATOM => KVal::Enum(KData::atom(k)),
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
            qtype::COMPOUND_LIST => KVal::CompoundList(k.as_slice::<*mut K>().unwrap().to_owned()),
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
            /* 20   */ qtype::ENUM_LIST => KVal::Enum(KData::list(k)),
            /* 99   */ qtype::TABLE => todo!(),
            /* 101  */ qtype::DICTIONARY => todo!(),
            /* 112  */ qtype::FOREIGN => todo!(),
            /* 127  */
            qtype::SORTED_DICTIONARY => todo!(), // probably reuse the dictionary type
            _ => KVal::Null,
        }
    }
}

impl<'a> KVal<'a> {
    /// Convert this KVal into a CompoundList variant
    ///
    /// port of [`simple_to_compound`](crate::api::simple_to_compound)
    ///
    /// # Examples
    ///
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
    ///
    /// # Note
    /// * creates new k objects when necessary
    /// * if the KVal is an Enum List, enum_source must be provided
    /// * if the KVal is already a CompoundList variant, it will be returned as is
    /// * if the KVal is a simple list, it's contants will be converted to k objects and placed in a new CompoundList
    /// * if KVal is not a list, an error will be returned
    /// * Enum elements from different enum sources must be contained in a compound list. Therefore
    ///   this function intentionally restricts the number of enum sources to one so that user switches
    ///   a simple list to a compound list when the second enum sources are provided.
    ///
    /// # Safety
    /// enum_source must be able to be converted to a C string (i.e. no null bytes)
    #[inline]
    pub fn as_compound_list(&'a self, enum_source: Option<&'a str>) -> KVal<'a> {
        use KData::*;
        use KVal::*;
        match self {
            Bool(List(list)) => CompoundList(
                list.iter()
                    .map(|&atom| re_exports::new_bool(atom).cast_mut())
                    .collect::<Vec<_>>(),
            ),
            Guid(KData::List(list)) => CompoundList(
                list.iter()
                    .map(|&atom| re_exports::new_guid(atom).cast_mut())
                    .collect::<Vec<_>>(),
            ),
            Byte(KData::List(list)) => CompoundList(
                list.iter()
                    .map(|&atom| re_exports::new_byte(atom.into()).cast_mut())
                    .collect::<Vec<_>>(),
            ),
            Short(KData::List(list)) => CompoundList(
                list.iter()
                    .map(|&atom| re_exports::new_short(atom.into()).cast_mut())
                    .collect::<Vec<_>>(),
            ),
            Int(KData::List(list)) => CompoundList(
                list.iter()
                    .map(|&atom| re_exports::new_int(atom).cast_mut())
                    .collect::<Vec<_>>(),
            ),
            Long(KData::List(list)) => CompoundList(
                list.iter()
                    .map(|&atom| re_exports::new_long(atom).cast_mut())
                    .collect::<Vec<_>>(),
            ),
            Real(KData::List(list)) => CompoundList(
                list.iter()
                    .map(|&atom| re_exports::new_real(atom.into()).cast_mut())
                    .collect::<Vec<_>>(),
            ),
            Float(KData::List(list)) => CompoundList(
                list.iter()
                    .map(|&atom| re_exports::new_float(atom).cast_mut())
                    .collect::<Vec<_>>(),
            ),
            Symbol(KData::List(list)) => CompoundList(
                list.iter()
                    .map(|&atom| unsafe { re_exports::new_symbol_from_S(atom) }.cast_mut())
                    .collect::<Vec<_>>(),
            ),
            Timestamp(KData::List(list)) => CompoundList(
                list.iter()
                    .map(|&atom| re_exports::new_timestamp(atom).cast_mut())
                    .collect::<Vec<_>>(),
            ),
            Month(KData::List(list)) => CompoundList(
                list.iter()
                    .map(|&atom| re_exports::new_month(atom).cast_mut())
                    .collect::<Vec<_>>(),
            ),
            Date(KData::List(list)) => CompoundList(
                list.iter()
                    .map(|&atom| re_exports::new_date(atom).cast_mut())
                    .collect::<Vec<_>>(),
            ),
            Datetime(KData::List(list)) => CompoundList(
                list.iter()
                    .map(|&atom| re_exports::new_datetime(atom).cast_mut())
                    .collect::<Vec<_>>(),
            ),
            Timespan(KData::List(list)) => CompoundList(
                list.iter()
                    .map(|&atom| re_exports::new_timespan(atom).cast_mut())
                    .collect::<Vec<_>>(),
            ),
            Minute(KData::List(list)) => CompoundList(
                list.iter()
                    .map(|&atom| re_exports::new_minute(atom).cast_mut())
                    .collect::<Vec<_>>(),
            ),
            Second(KData::List(list)) => CompoundList(
                list.iter()
                    .map(|&atom| re_exports::new_second(atom).cast_mut())
                    .collect::<Vec<_>>(),
            ),
            Time(KData::List(list)) => CompoundList(
                list.iter()
                    .map(|&atom| re_exports::new_time(atom).cast_mut())
                    .collect::<Vec<_>>(),
            ),
            Enum(KData::List(list)) => {
                let Some(source) = enum_source else {
                    return KVal::from(unsafe {&*re_exports::new_error("Enum list must have exactly one source per atom\0")});
                };
                CompoundList(
                    list.iter()
                        .map(|&atom| re_exports::new_enum(source, atom).cast_mut())
                        .collect::<Vec<_>>(),
                )
            }
            _ => KVal::from(unsafe { &*re_exports::new_error("not a simple list\0") }),
        }
    }

    /// Convert this value back into a K value,
    ///
    /// # Note
    /// * uses methods from the native q api to create NEW K objects from the data in this value.
    /// * consumes self, this is deliberate as we don't want multiple references to the same data
    /// * this function should be used to create NEW K objects from manually initialized KVals
    pub fn to_k(self) -> *const K {
        // TODO: once this wrapper has more control over the memory, decrease the reference count
        // of the original k object if there is one

        match self {
            KVal::CompoundList(list) => {
                let k = re_exports::new_list(qtype::COMPOUND_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                // copy the elements over
                unsafe { &mut *k }
                    .as_mut_slice::<*mut K>()
                    .unwrap()
                    .copy_from_slice(&list);
                k.cast_const()
            }
            KVal::Bool(KData::Atom(&atom)) => re_exports::new_bool(atom),
            KVal::Bool(KData::List(list)) => {
                let k = re_exports::new_list(qtype::BOOL_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<i32>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Guid(KData::Atom(&atom)) => re_exports::new_guid(atom),
            KVal::Guid(KData::List(list)) => {
                let k = re_exports::new_list(qtype::GUID_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<[u8; 16]>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Byte(KData::Atom(&atom)) => re_exports::new_byte(atom.into()),
            KVal::Byte(KData::List(list)) => {
                let k = re_exports::new_list(qtype::BYTE_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<u8>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Short(KData::Atom(&atom)) => re_exports::new_short(atom.into()),
            KVal::Short(KData::List(list)) => {
                let k = re_exports::new_list(qtype::SHORT_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<i16>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Int(KData::Atom(&atom)) => re_exports::new_int(atom),
            KVal::Int(KData::List(list)) => {
                let k = re_exports::new_list(qtype::INT_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<i32>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Long(KData::Atom(&atom)) => re_exports::new_long(atom),
            KVal::Long(KData::List(list)) => {
                let k = re_exports::new_list(qtype::LONG_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<i64>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Real(KData::Atom(&atom)) => re_exports::new_real(atom.into()),
            KVal::Real(KData::List(list)) => {
                let k = re_exports::new_list(qtype::REAL_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<f32>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Float(KData::Atom(&atom)) => re_exports::new_float(atom),
            KVal::Float(KData::List(list)) => {
                let k = re_exports::new_list(qtype::FLOAT_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<f64>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Symbol(KData::Atom(&atom)) => unsafe { re_exports::new_symbol_from_S(atom) },
            KVal::Symbol(KData::List(list)) => {
                let k = re_exports::new_list(qtype::SYMBOL_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<S>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Timestamp(KData::Atom(&atom)) => re_exports::new_timestamp(atom),
            KVal::Timestamp(KData::List(list)) => {
                let k = re_exports::new_list(qtype::TIMESTAMP_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<i64>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Month(KData::Atom(&atom)) => re_exports::new_month(atom),
            KVal::Month(KData::List(list)) => {
                let k = re_exports::new_list(qtype::MONTH_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<i32>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Date(KData::Atom(&atom)) => re_exports::new_date(atom),
            KVal::Date(KData::List(list)) => {
                let k = re_exports::new_list(qtype::DATE_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<i32>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Datetime(KData::Atom(&atom)) => re_exports::new_datetime(atom),
            KVal::Datetime(KData::List(list)) => {
                let k = re_exports::new_list(qtype::DATETIME_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<f64>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Timespan(KData::Atom(&atom)) => re_exports::new_timespan(atom),
            KVal::Timespan(KData::List(list)) => {
                let k = re_exports::new_list(qtype::TIMESPAN_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<i64>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Minute(KData::Atom(&atom)) => re_exports::new_minute(atom),
            KVal::Minute(KData::List(list)) => {
                let k = re_exports::new_list(qtype::MINUTE_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<i32>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Second(KData::Atom(&atom)) => re_exports::new_second(atom),
            KVal::Second(KData::List(list)) => {
                let k = re_exports::new_list(qtype::SECOND_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<i32>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Time(KData::Atom(&atom)) => re_exports::new_time(atom),
            KVal::Time(KData::List(list)) => {
                let k = re_exports::new_list(qtype::TIME_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<i32>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Enum(KData::Atom(_)) => unimplemented!("pass an enum source to the macro"),
            KVal::Enum(KData::List(list)) => {
                let k = re_exports::new_list(qtype::ENUM_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<i64>()
                    .unwrap()
                    .copy_from_slice(list);
                k.cast_const()
            }
            KVal::Char(&atom) => re_exports::new_char(atom as char),
            KVal::String(&list) => unsafe { re_exports::new_string_from_S(list) },
            KVal::Err(&err) => unsafe { re_exports::new_error_from_S(err) },
            KVal::Null => KNULL,
        }
    }
}

/// Macro to convert a KVal to a K object.
///
/// uses [`to_k`](KVal::to_k) under the hood.
///
/// # Examples
/// ```no_run
/// use kdbplus::rusty_api::types::*;
/// use kdbplus::rusty_api::*;
/// use kdbplus::to_k;
///
/// let kval = KVal::Int(KData::Atom(&42));
/// let k = to_k!(kval);
/// assert_eq!(k, unsafe { new_int(42) });
/// ```
/// ```no_run
/// # use kdbplus::rusty_api::types::*;
/// # use kdbplus::rusty_api::*;
/// # use kdbplus::to_k;
///
/// let kval = KVal::Enum(KData::Atom(&1_i64));
/// let k = to_k!(kval, "enum_src");
/// assert_eq!(k, unsafe { new_enum("enum_src", 1_i64) });
/// ```
#[macro_export]
macro_rules! to_k {
    ($kval:expr) => {
        $kval.to_k()
    };
    ($kval:expr, $enum_src:expr ) => {
        match $kval {
            kdbplus::rusty_api::types::KVal::Enum(KData::Atom(&atom)) => {
                kdbplus::rusty_api::new_enum($enum_src, atom)
            }
            _ => $kval.to_k(),
        }
    };
}
