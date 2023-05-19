//++++++++++++++++++++++++++++++++++++++++++++++++++//
// >> Structs
//++++++++++++++++++++++++++++++++++++++++++++++++++//

use super::{re_exports, SafeToCastFromKInner, K, KNULL, S};
use crate::qtype;
use std::borrow::Cow;

/// Rust friendly wrapper for q Atoms and Lists.
/// references are mutable to indicate that changes should propagate back to q.
#[derive(Debug)]
pub enum KData<'a, T>
where
    T: 'a + std::fmt::Debug + SafeToCastFromKInner,
    [T]: 'a + ToOwned<Owned = Vec<T>>,
{
    Atom(&'a T), // TODO: Should this be mut, const, or neither?
    //List(&'a [T]), // TODO: Should this be mut, const, or neither?
    List(Cow<'a, [T]>),
}

impl<'a, T> KData<'a, T>
where
    T: 'a + std::fmt::Debug + SafeToCastFromKInner + std::clone::Clone,
    [T]: 'a + ToOwned<Owned = Vec<T>>,
{
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
        KData::List(Cow::from(k.as_slice().unwrap()))
    }
}

/// intuitive rust wrappers for q types, allowing for idiomatic rust code
/// that can take full advantage of rust's powerful pattern matching and type system
/// when interacting with q.
pub enum KVal<'a> {
    // by doing it this way, we can use the same enum for both atoms and lists
    /// Slice of pointers to other K objects
    CompoundList(Vec<*mut K>),
    /// Note: the C api uses [`I`] (i32) for booleans. we use bool in Rust.
    Bool(KData<'a, bool>),
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
    /// Note: the C api uses [`I`] (i32) for chars. we use char in Rust https://github.com/KxSystems/kdb/blob/bbc40b8cb870948122a36cb80a486bc5f7e470d7/c/c/k.h#L29.
    Char(&'a char),
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
    /// Note: the C api uses [`S`] (*mut c_char) for strings. we use a Cow smart pointer so it's a zero-copy &str wrapper for read-only operations, that is converted to an owned string when needed in Rust.
    String(Cow<'a, str>),
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
    #[inline] // because there are large pattern matches, this is a good candidate for inlining to enable more robust compiler optimizations
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
            /* 10   */
            qtype::STRING => KVal::String(Cow::Borrowed(unsafe {
                std::str::from_utf8_unchecked(k.as_slice_unchecked())
            })),
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
    /// Create a new KVal from a K object
    ///
    /// dereferences a raw pointer.
    ///
    /// # Safety
    /// * passed K object must be a valid pointer
    ///   * the only way to guaruntee this is to only pass K objects that are given from the q instance calling your function,
    ///     or those created by native api functions.
    #[inline]
    #[allow(clippy::not_unsafe_ptr_arg_deref)] // we can say that this is safe because K objects can only come from the q instance or native api functions, which both guaruntee that the pointer is valid
    pub fn from_raw(k: *const K) -> Self {
        match unsafe { k.as_ref() } {
            Some(k) => Self::from(k),
            None => Self::Null,
        }
    }

    /// Create a CompoundList Variant from this KVal
    ///
    /// takes ownership of self, and returns it or a new KVal
    ///
    /// in cases of an error, the error string will be null-terminated
    ///
    /// port of [`simple_to_compound`](crate::api::simple_to_compound)
    ///
    /// # Examples
    /// TODO: add some
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
    /// * if the KVal is a simple list, it's contents will be converted to k objects and placed in a new CompoundList
    /// * if KVal is not a list, an error will be returned
    /// * Enum elements from different enum sources must be contained in a compound list. Therefore
    ///   this function intentionally restricts the number of enum sources to one so that user switches
    ///   a simple list to a compound list when the second enum sources are provided.
    ///
    /// # Safety
    /// enum_source must be able to be converted to a C string (i.e. no null bytes)
    #[inline] // because there are large pattern matches, this is a good candidate for inlining to enable more robust compiler optimizations
    pub fn to_compound_list(self, enum_source: Option<&'a str>) -> Result<KVal<'a>, &'static str> {
        // private macro to reduce code repition in the as_compound_list method when creating a compound list from a KData list
        macro_rules! to_compound {
            ($simple_list:ident,$constructor:expr) => {{
                Ok(KVal::CompoundList(
                    $simple_list
                        .iter()
                        .map(|&a| $constructor(a.into()).cast_mut())
                        .collect::<Vec<_>>(),
                ))
            }};
        }

        use re_exports::*;
        use KData::*; // for brevity
        use KVal::*; // for brevity // for brevity
        match self {
            CompoundList(_) => Ok(self),
            Bool(List(l)) => to_compound!(l, new_bool),
            Guid(KData::List(l)) => to_compound!(l, new_guid),
            Byte(KData::List(l)) => to_compound!(l, new_byte),
            Short(KData::List(l)) => to_compound!(l, new_short),
            Int(KData::List(l)) => to_compound!(l, new_int),
            Long(KData::List(l)) => to_compound!(l, new_long),
            Real(KData::List(l)) => to_compound!(l, new_real),
            Float(KData::List(l)) => to_compound!(l, new_float),
            Symbol(KData::List(l)) => to_compound!(l, |a| unsafe { new_symbol_from_S(a) }),
            Timestamp(KData::List(l)) => to_compound!(l, new_timestamp),
            Month(KData::List(l)) => to_compound!(l, new_month),
            Date(KData::List(l)) => to_compound!(l, new_date),
            Datetime(KData::List(l)) => to_compound!(l, new_datetime),
            Timespan(KData::List(l)) => to_compound!(l, new_timespan),
            Minute(KData::List(l)) => to_compound!(l, new_minute),
            Second(KData::List(l)) => to_compound!(l, new_second),
            Time(KData::List(l)) => to_compound!(l, new_time),
            Enum(KData::List(l)) => {
                let Some(source) = enum_source else {
                    return Result::Err("Enum list must have exactly one source per atom\0");
                };
                to_compound!(l, |a| new_enum(source, a))
            }
            _ => Result::Err("self is not a simple list\0"),
        }
    }

    /// Join two KVals together
    ///
    /// acheives same functionality as [`push`](crate::api::KUtility::push) and [`append`](crate::api::KUtility::append)
    ///
    /// * in cases of errror, the error string will be null-terminated
    /// * will mutate base (except in error cases), causing base's inner Cow to be converted to it's owned variant (usually by cloning)
    /// * consumes other
    /// * order will always be [base[..], other[..]]
    ///
    /// # Side Effects
    /// * takes ownership of base
    /// * other will be consumed
    ///
    /// # Errors
    /// * if base and other are not the same type (ie Int or Long)
    /// * if base is a simple list and other is a compound list
    /// * if base or other are: Err, Null, Char, String, Table, Dictionary, Foreign, or SortedDictionary variant
    /// 
    /// # Note
    /// behavior depends on variant of base and other
    /// * if base is an atom, other must be an atom or simple list of the same type, and base will be converted to a list (inner cow will be Owned)
    /// * if base is a simple list, other must be a simple list or atom of the same type
    /// * if base is a compound list, other must be a compound list (to combine a compound list with a simple list, use as_compound_list first)
    /// 
    /// # Examples
    /// TODO: add some
    #[inline] // because there are large pattern matches, this is a good candidate for inlining to enable more robust compiler optimizations
    pub fn join(mut base: Self, other: Self) -> Result<Self, &'static str> {
        use KData::*; // for brevity
        use KVal::*; // for brevity
        // private macro to reduce code repition when joining 2 simple KVals (that may be atoms or lists)
        macro_rules! join {
            ($base:ident, $other:ident, $type:path) => {
                match ($base,$other) {
                    (Atom(&base_atom), Atom(&other_atom)) => base = $type(List(Cow::from(vec![base_atom, other_atom]))),
                    (Atom(&base_atom), List(other_list)) => {
                        let mut tmp = vec![base_atom];
                        tmp.append(&mut other_list.into_owned());
                        base = $type(List(Cow::from(tmp)))
                    },
                    (List(base_list), Atom(&other_atom)) => base_list.to_owned().into_owned().push(other_atom),
                    (List(base_list), List(other_list)) => base_list.to_owned().into_owned().append(&mut other_list.into_owned()),
                }
            };
        }
        // append other to base, or return error
        match (&mut base, other) {
            (CompoundList(base_data), CompoundList(mut other_data)) => {
                base_data.append(&mut other_data);
            },
            (Bool(base_data), Bool(other_data)) => join!(base_data, other_data, KVal::Bool),
            (Guid(base_data), Guid(other_data)) => join!(base_data, other_data, KVal::Guid),
            (Byte(base_data), Byte(other_data)) => join!(base_data, other_data, KVal::Byte),
            (Short(base_data), Short(other_data)) => join!(base_data, other_data, KVal::Short),
            (Int(base_data), Int(other_data)) => join!(base_data, other_data, KVal::Int),
            (Long(base_data), Long(other_data)) => join!(base_data, other_data, KVal::Long),
            (Real(base_data), Real(other_data)) => join!(base_data, other_data, KVal::Real),
            (Float(base_data), Float(other_data)) => join!(base_data, other_data, KVal::Float),
            (Symbol(base_data), Symbol(other_data)) => join!(base_data, other_data, KVal::Symbol),
            (Timestamp(base_data), Timestamp(other_data)) => join!(base_data, other_data, KVal::Timestamp),
            (Month(base_data), Month(other_data)) => join!(base_data, other_data, KVal::Month),
            (Date(base_data), Date(other_data)) => join!(base_data, other_data, KVal::Date),
            (Datetime(base_data), Datetime(other_data)) => join!(base_data, other_data, KVal::Datetime),
            (Timespan(base_data), Timespan(other_data)) => join!(base_data, other_data, KVal::Timespan),
            (Minute(base_data), Minute(other_data)) => join!(base_data, other_data, KVal::Minute),
            (Second(base_data), Second(other_data)) => join!(base_data, other_data, KVal::Second),
            (Time(base_data), Time(other_data)) => join!(base_data, other_data, KVal::Time),
            (Enum(base_data), Enum(other_data)) => join!(base_data, other_data, KVal::Enum),
            _ => return Result::Err("invalid types\0"),
        }
        // if we get here, there was not an error, return base
        Ok(base)
    }

    /// Create a list variant from an atom
    ///
    /// takes ownwership of self, and returns either a new list variant, or self unchanged
    ///
    /// # Note
    /// * if the object is already a list, it will be returned unchanged
    /// * if the object is an atom, it will be converted to a list
    /// * if the object is any other variant, an error will be returned
    #[inline] // because there are large pattern matches, this is a good candidate for inlining to enable more robust compiler optimizations
    pub fn to_list(self) -> Result<KVal<'a>, &'static str> {
        use KData::*; // for brevity
        use KVal::*; // for brevity

        macro_rules! to_list {
            ($kdata:ident, $ktype:path) => {
                match $kdata {
                    Atom(&atom) => Ok($ktype(List(Cow::from(vec![atom])))),
                    List(list) => Ok($ktype(List(list))),
                }
            };
        }

        match self {
            CompoundList(_) => Ok(self),
            Bool(data) => to_list!(data, Bool),
            Guid(data) => to_list!(data, Guid),
            Byte(data) => to_list!(data, Byte),
            Short(data) => to_list!(data, Short),
            Int(data) => to_list!(data, Int),
            Long(data) => to_list!(data, Long),
            Real(data) => to_list!(data, Real),
            Float(data) => to_list!(data, Float),
            Symbol(data) => to_list!(data, Symbol),
            Timestamp(data) => to_list!(data, Timestamp),
            Month(data) => to_list!(data, Month),
            Date(data) => to_list!(data, Date),
            Datetime(data) => to_list!(data, Datetime),
            Timespan(data) => to_list!(data, Timespan),
            Minute(data) => to_list!(data, Minute),
            Second(data) => to_list!(data, Second),
            Time(data) => to_list!(data, Time),
            Enum(data) => to_list!(data, Enum),
            _ => Result::Err("invalid type\0"),
        }
    }

    /// Convert this value back into a K value,
    ///
    /// # Note
    /// * uses methods from the native q api to create NEW K objects from the data in this value.
    /// * consumes self, this is deliberate as we don't want multiple references to the same data
    /// * this function should be used to create NEW K objects from manually initialized KVals
    ///
    /// # Examples
    /// TODO: add some
    #[inline] // because there are large pattern matches, this is a good candidate for inlining to enable more robust compiler optimizations
    pub fn to_k(self) -> *const K {
        // private macro to reduce repition in the to_k method when initializing a list
        macro_rules! list_to_k {
            ($slice_type:ty, $new_list_type:expr,$from_list:ident) => {{
                // create new k list with the same length as from_list
                let k = re_exports::new_list($new_list_type, $from_list.len().try_into().unwrap())
                    .cast_mut();
                // copy elements over
                unsafe { &mut *k }
                    .as_mut_slice::<$slice_type>()
                    .unwrap()
                    .copy_from_slice($from_list.as_ref());
                k.cast_const()
            }};
        }

        match self {
            KVal::CompoundList(list) => list_to_k!(*mut K, qtype::COMPOUND_LIST, list),
            KVal::Bool(KData::Atom(&atom)) => re_exports::new_bool(atom),
            KVal::Bool(KData::List(list)) => list_to_k!(bool, qtype::BOOL_LIST, list),
            KVal::Guid(KData::Atom(&atom)) => re_exports::new_guid(atom),
            KVal::Guid(KData::List(list)) => list_to_k!([u8; 16], qtype::GUID_LIST, list),
            KVal::Byte(KData::Atom(&atom)) => re_exports::new_byte(atom.into()),
            KVal::Byte(KData::List(list)) => list_to_k!(u8, qtype::BYTE_LIST, list),
            KVal::Short(KData::Atom(&atom)) => re_exports::new_short(atom.into()),
            KVal::Short(KData::List(list)) => list_to_k!(i16, qtype::SHORT_LIST, list),
            KVal::Int(KData::Atom(&atom)) => re_exports::new_int(atom),
            KVal::Int(KData::List(list)) => list_to_k!(i32, qtype::INT_LIST, list),
            KVal::Long(KData::Atom(&atom)) => re_exports::new_long(atom),
            KVal::Long(KData::List(list)) => list_to_k!(i64, qtype::LONG_LIST, list),
            KVal::Real(KData::Atom(&atom)) => re_exports::new_real(atom.into()),
            KVal::Real(KData::List(list)) => list_to_k!(f32, qtype::REAL_LIST, list),
            KVal::Float(KData::Atom(&atom)) => re_exports::new_float(atom),
            KVal::Float(KData::List(list)) => list_to_k!(f64, qtype::FLOAT_LIST, list),
            KVal::Symbol(KData::Atom(&atom)) => unsafe { re_exports::new_symbol_from_S(atom) },
            KVal::Symbol(KData::List(list)) => list_to_k!(S, qtype::SYMBOL_LIST, list),
            KVal::Timestamp(KData::Atom(&atom)) => re_exports::new_timestamp(atom),
            KVal::Timestamp(KData::List(list)) => list_to_k!(i64, qtype::TIMESTAMP_LIST, list),
            KVal::Month(KData::Atom(&atom)) => re_exports::new_month(atom),
            KVal::Month(KData::List(list)) => list_to_k!(i32, qtype::MONTH_LIST, list),
            KVal::Date(KData::Atom(&atom)) => re_exports::new_date(atom),
            KVal::Date(KData::List(list)) => list_to_k!(i32, qtype::DATE_LIST, list),
            KVal::Datetime(KData::Atom(&atom)) => re_exports::new_datetime(atom),
            KVal::Datetime(KData::List(list)) => list_to_k!(f64, qtype::DATETIME_LIST, list),
            KVal::Timespan(KData::Atom(&atom)) => re_exports::new_timespan(atom),
            KVal::Timespan(KData::List(list)) => list_to_k!(i64, qtype::TIMESPAN_LIST, list),
            KVal::Minute(KData::Atom(&atom)) => re_exports::new_minute(atom),
            KVal::Minute(KData::List(list)) => list_to_k!(i32, qtype::MINUTE_LIST, list),
            KVal::Second(KData::Atom(&atom)) => re_exports::new_second(atom),
            KVal::Second(KData::List(list)) => list_to_k!(i32, qtype::SECOND_LIST, list),
            KVal::Time(KData::Atom(&atom)) => re_exports::new_time(atom),
            KVal::Time(KData::List(list)) => list_to_k!(i32, qtype::TIME_LIST, list),
            KVal::Enum(KData::Atom(_)) => unimplemented!("pass an enum source to the to_k! macro"),
            KVal::Enum(KData::List(list)) => list_to_k!(i64, qtype::ENUM_LIST, list),
            KVal::Char(&atom) => re_exports::new_char(atom),
            KVal::String(list) => re_exports::new_string(&list),
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
