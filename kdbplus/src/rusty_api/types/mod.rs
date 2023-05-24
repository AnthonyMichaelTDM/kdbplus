use super::{re_exports, K, S};
use crate::qtype;
use std::{borrow::Cow, ffi::CString};

mod kdata;
pub use kdata::*;
mod ktable;
pub use ktable::*;
mod kdict;
pub use kdict::*;

//++++++++++++++++++++++++++++++++++++++++++++++++++//
// >> Structs
//++++++++++++++++++++++++++++++++++++++++++++++++++//

/// intuitive rust wrappers for q types, allowing for idiomatic rust code
/// that can take full advantage of rust's powerful pattern matching and type system
/// when interacting with q.
///
/// TODO: better document the parameters for each type, what they represent, and why they are the type they are.
#[derive(Debug, Clone)]
pub enum KVal<'a> {
    // by doing it this way, we can use the same enum for both atoms and lists
    /// Slice of pointers to other K objects
    CompoundList(Vec<KVal<'a>>),
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
    Char(char),
    /// Note: the C api uses [`S`] (*mut c_char) for symbols. we use String in Rust to guarentee memory safety.
    Symbol(KData<'a, String>),
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
    /// * We also allow for an optional string to be passed in, which is the source of the enum, this must be set before converting to a [`K`] object.
    /// * Enum elements from different enum sources must be contained in a compound list. Therefore
    ///   this variant intentionally restricts the number of enum sources to one so that user switches
    ///   a simple list to a compound list when more enum sources are needed.
    ///   * if your enum atom/list already has a set source, it will be used, otherwise the given enum_source will be used if it is Some
    ///   * will return an error if the source of your enum, and given enum_source, are None
    ///
    Enum(KData<'a, i64>, Option<&'a str>),
    /// Note: the C api uses [`S`] (*mut c_char) for strings. we use a Cow smart pointer so it's a zero-copy &str wrapper for read-only operations, that is converted to an owned string when needed in Rust.
    String(Cow<'a, str>),
    // TODO: Foreign
    /// a dictionary is a KList with 2 elements, the first being the keys, the second being the values
    Dictionary(KDict<'a>),
    // TODO: Sorted Dictionary
    /// behind the scenes, a table is just a specialized dictionary where keys are symbols and values are lists
    Table(KTable<'a>),
    /// q Error, created by krr or orr. we use Cow<str> in Rust to avoid reading invalid pointers if/when the data is dropped
    /// # Note
    /// * the inner string must be null terminated
    Error(Cow<'a, str>),
    /// the q-equivalent value of null depends on a great many factors.
    Null,
}

impl<'a> KVal<'a> {
    /// Create a new KVal from a reference to a [`K`](type.K.html) value.
    ///
    /// # Examples
    /// ```no_run
    /// use kdbplus::rusty_api::K;
    /// use kdbplus::rusty_api::types::{KVal, KData};
    /// use kdbplus::rusty_api::*;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn modify_long_list_a_bit(long_list: *const K) -> *const K {
    ///     match KVal::from(unsafe { &*long_list }, None) {
    ///         KVal::Long(KData::List(mut list)) => {
    ///             if list.len() < 2 {
    ///                 return new_error("this list is not long enough. how ironic...\0");
    ///             }
    ///             list.to_mut()[1] = 30000_i64;
    ///             KVal::Long(KData::List(list)).to_k()
    ///         }
    ///         _ => new_error("invalid type\0"),
    ///     }
    /// }
    /// ```
    /// ```q
    /// q)modify_long_list_a_bit: LIBPATH_ (`modify_long_list_a_bit; 1)
    /// q)modify_long_list_a_bit 1 2 3
    /// 1 30000 3
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
    /// * This function tries to use as few allocations as possible, but that isn't always possible (e.g. strings and symbols (really anything backed by pointers)).
    #[inline] // because there are large pattern matches, this is a good candidate for inlining to enable more robust compiler optimizations
    pub fn from(k: &'a K, enum_source: Option<&'a str>) -> KVal<'a> {
        match k.qtype {
            /* -128 */
            qtype::ERROR => KVal::Error(Cow::Borrowed(
                std::str::from_utf8(k.as_slice().unwrap()).unwrap(),
            )),
            /* -20  */ qtype::ENUM_ATOM => KVal::Enum(KData::atom(k), enum_source),
            /* -19  */ qtype::TIME_ATOM => KVal::Time(KData::atom(k)),
            /* -18  */ qtype::SECOND_ATOM => KVal::Second(KData::atom(k)),
            /* -17  */ qtype::MINUTE_ATOM => KVal::Minute(KData::atom(k)),
            /* -16  */ qtype::TIMESPAN_ATOM => KVal::Timespan(KData::atom(k)),
            /* -15  */ qtype::DATETIME_ATOM => KVal::Datetime(KData::atom(k)),
            /* -14  */ qtype::DATE_ATOM => KVal::Date(KData::atom(k)),
            /* -13  */ qtype::MONTH_ATOM => KVal::Month(KData::atom(k)),
            /* -12  */ qtype::TIMESTAMP_ATOM => KVal::Timestamp(KData::atom(k)),
            /* -11  */ qtype::SYMBOL_ATOM => KVal::Symbol(KData::symbol(k)),
            /* -10  */ qtype::CHAR => KVal::Char(unsafe { k.value.byte } as char),
            /* -9   */ qtype::FLOAT_ATOM => KVal::Float(KData::atom(k)),
            /* -8   */ qtype::REAL_ATOM => KVal::Real(KData::atom(k)),
            /* -7   */ qtype::LONG_ATOM => KVal::Long(KData::atom(k)),
            /* -6   */ qtype::INT_ATOM => KVal::Int(KData::atom(k)),
            /* -5   */ qtype::SHORT_ATOM => KVal::Short(KData::atom(k)),
            /* -4   */ qtype::BYTE_ATOM => KVal::Byte(KData::atom(k)),
            /* -2   */ qtype::GUID_ATOM => KVal::Guid(KData::guid_atom(k)),
            /* -1   */ qtype::BOOL_ATOM => KVal::Bool(KData::atom(k)),
            /* 0    */
            qtype::COMPOUND_LIST => KVal::CompoundList(
                k.as_slice::<*mut K>()
                    .unwrap()
                    .iter()
                    .map(|k| KVal::from_raw(*k, enum_source))
                    .collect(),
            ),
            /* 1    */ qtype::BOOL_LIST => KVal::Bool(KData::list(k)),
            /* 2    */ qtype::GUID_LIST => KVal::Guid(KData::list(k)),
            /* 4    */ qtype::BYTE_LIST => KVal::Byte(KData::list(k)),
            /* 5    */ qtype::SHORT_LIST => KVal::Short(KData::list(k)),
            /* 6    */ qtype::INT_LIST => KVal::Int(KData::list(k)),
            /* 7    */ qtype::LONG_LIST => KVal::Long(KData::list(k)),
            /* 8    */ qtype::REAL_LIST => KVal::Real(KData::list(k)),
            /* 9    */ qtype::FLOAT_LIST => KVal::Float(KData::list(k)),
            /* 10   */
            qtype::STRING => KVal::String(Cow::Borrowed(
                std::str::from_utf8(k.as_slice().unwrap()).unwrap(),
            )),
            /* 11   */ qtype::SYMBOL_LIST => KVal::Symbol(KData::symbol_list(k)),
            /* 12   */ qtype::TIMESTAMP_LIST => KVal::Timestamp(KData::list(k)),
            /* 13   */ qtype::MONTH_LIST => KVal::Month(KData::list(k)),
            /* 14   */ qtype::DATE_LIST => KVal::Date(KData::list(k)),
            /* 15   */ qtype::DATETIME_LIST => KVal::Datetime(KData::list(k)),
            /* 16   */ qtype::TIMESPAN_LIST => KVal::Timespan(KData::list(k)),
            /* 17   */ qtype::MINUTE_LIST => KVal::Minute(KData::list(k)),
            /* 18   */ qtype::SECOND_LIST => KVal::Second(KData::list(k)),
            /* 19   */ qtype::TIME_LIST => KVal::Time(KData::list(k)),
            /* 20   */ qtype::ENUM_LIST => KVal::Enum(KData::list(k), enum_source),
            /* 98   */ qtype::TABLE => KVal::Table(KTable::new_from_k(k)),
            /* 99   */ qtype::DICTIONARY => KVal::Dictionary(KDict::new_from_k(k)),
            /* 112  */ qtype::FOREIGN => todo!("Foreign objects not yet implemented"),
            /* 127  */ qtype::SORTED_DICTIONARY => KVal::Dictionary(KDict::new_from_k(k)),
            _ => KVal::Null,
        }
    }
}

impl<'a> KVal<'a> {
    /// Create a new KVal from a K object
    ///
    /// dereferences a raw pointer.
    ///
    /// see [`from`](`KVal`) for more information, as this is just a wrapper around that function.
    ///
    /// # Example
    /// ```no_run
    /// use kdbplus::rusty_api::K;
    /// use kdbplus::rusty_api::types::{KVal, KData};
    /// use kdbplus::rusty_api::*;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn modify_long_list_a_bit(long_list: *const K) -> *const K {
    ///     match KVal::from_raw(long_list, None) {
    ///         KVal::Long(KData::List(mut list)) => {
    ///             if list.len() < 2 {
    ///                 return new_error("this list is not long enough. how ironic...\0");
    ///             }
    ///             list.to_mut()[1] = 30000_i64;
    ///             KVal::Long(KData::List(list)).to_k()
    ///         }
    ///         _ => new_error("invalid type\0"),
    ///     }
    /// }
    /// ```
    /// ```q
    /// q)modify_long_list_a_bit: LIBPATH_ (`modify_long_list_a_bit; 1)
    /// q)modify_long_list_a_bit 1 2 3
    /// 1 30000 3
    /// ```
    ///
    /// # Note
    /// * given enum source will be applied to all enum variants, recursively (e.g. enums in a compound list)
    ///
    /// # Safety
    /// * passed K object must be a valid pointer
    ///   * the only way to guaruntee this is to only pass K objects that are given from the q instance calling your function,
    ///     or those created by native api functions.
    #[inline]
    #[allow(clippy::not_unsafe_ptr_arg_deref)] // we can say that this is safe because K objects can only come from the q instance or native api functions, which both guaruntee that the pointer is valid
    pub fn from_raw(k: *const K, enum_source: Option<&'a str>) -> Self {
        match unsafe { k.as_ref() } {
            Some(k) => Self::from(k, enum_source),
            None => Self::Null,
        }
    }

    /// Create a CompoundList Variant from this KVal
    ///
    /// takes self, and a new KVal that is a CompoundList with the contents of self
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
    ///
    /// # Safety
    /// enum_source must be able to be converted to a C string (i.e. no null bytes)
    #[inline] // because there are large pattern matches, this is a good candidate for inlining to enable more robust compiler optimizations
    pub fn to_compound_list(self) -> Result<Self, &'static str> {
        // private macro to reduce code repition in the as_compound_list method when creating a compound list from a KData list
        macro_rules! to_compound {
            // this variant is for when no type conversion is needed, and value is borrowed
            ($simple_list:ident,$constructor:expr) => {{
                Ok(KVal::CompoundList(
                    $simple_list
                        .iter()
                        .map(|a| $constructor(Atom(Cow::Owned(a.to_owned()))))
                        .collect::<Vec<_>>(),
                ))
            }};
            // this variant is for enums
            ($simple_list:ident,$constructor:expr,$enum_source:expr) => {{
                Ok(KVal::CompoundList(
                    $simple_list
                        .iter()
                        .map(|a| $constructor(Atom(Cow::Owned(a.to_owned())), $enum_source))
                        .collect::<Vec<_>>(),
                ))
            }};
        }

        use KData::*; // for brevity
        use KVal::*; // for brevity // for brevity
        match self {
            CompoundList(list) => Ok(CompoundList(list.to_owned())),
            Bool(List(l)) => to_compound!(l, Bool),
            Guid(KData::List(l)) => to_compound!(l, Guid),
            Byte(KData::List(l)) => to_compound!(l, Byte),
            Short(KData::List(l)) => to_compound!(l, Short),
            Int(KData::List(l)) => to_compound!(l, Int),
            Long(KData::List(l)) => to_compound!(l, Long),
            Real(KData::List(l)) => to_compound!(l, Real),
            Float(KData::List(l)) => to_compound!(l, Float),
            Symbol(KData::List(l)) => to_compound!(l, Symbol),
            Timestamp(KData::List(l)) => to_compound!(l, Timestamp),
            Month(KData::List(l)) => to_compound!(l, Month),
            Date(KData::List(l)) => to_compound!(l, Date),
            Datetime(KData::List(l)) => to_compound!(l, Datetime),
            Timespan(KData::List(l)) => to_compound!(l, Timespan),
            Minute(KData::List(l)) => to_compound!(l, Minute),
            Second(KData::List(l)) => to_compound!(l, Second),
            Time(KData::List(l)) => to_compound!(l, Time),
            Enum(KData::List(l), source) => {
                if source.is_none() {
                    return Result::Err("Enum list must have exactly one source per atom\0");
                }
                to_compound!(l, Enum, source)
            }
            _ => Result::Err("self is not a simple list\0"),
        }
    }

    /// Join two KVals together
    ///
    /// acheives same functionality as [`push`](crate::api::KUtility) and [`append`](crate::api::KUtility)
    ///
    /// causes allocations (see [`append`](std::vec::Vec)), clones list wrapped by base and other.
    /// TODO: if possible, avoid these uncessary allocations by consuming other and modifying self.
    ///
    /// * in cases of errror, the error string will be null-terminated
    /// * order will always be [base[..], other[..]]
    ///
    /// # Errors
    /// * if base and other are not the same type (ie Int or Long)
    /// * if base is a simple list and other is a compound list
    /// * if base or other are: Err, Null, Char, String, Table, Dictionary, Foreign, or SortedDictionary variant
    ///
    /// # Note
    /// behavior depends on variant of base and other
    /// * if base is a simple list, other must be a simple list of the same type
    /// * if base is a compound list, other must be a compound list (to combine a compound list with a simple list, use as_compound_list first)
    /// * if base and other are enum lists, the source of base takes priority if set.
    ///
    /// # Examples
    /// TODO: add some
    #[inline] // because there are large pattern matches, this is a good candidate for inlining to enable more robust compiler optimizations
    pub fn join(base: Self, other: Self) -> Result<Self, &'static str> {
        use KData::*; // for brevity
        use KVal::*; // for brevity

        // private macro to reduce code repition when joining 2 simple KVals (that may be atoms or lists)
        macro_rules! join {
            // for typical lists
            ($variant:path, $base:ident, $other:ident) => {{
                let mut base = $base.into_owned();
                base.append(&mut $other.into_owned());
                Ok($variant(List(Cow::Owned(base))))
            }};
            // for enum lists
            ($variant:path, $base:ident, $other:ident, $enum_source:expr) => {{
                let mut base = $base.into_owned();
                base.append(&mut $other.into_owned());
                Ok($variant(List(Cow::Owned(base)), $enum_source))
            }};
        }
        // append other to base, and return it or error
        match (base, other) {
            (CompoundList(base_list), CompoundList(other_list)) => {
                let mut base = base_list.to_owned();
                base.append(&mut other_list.to_owned());
                Ok(CompoundList(base))
            }
            (Bool(List(bl)), Bool(List(ol))) => join!(Bool, bl, ol),
            (Guid(List(bl)), Guid(List(ol))) => join!(Guid, bl, ol),
            (Byte(List(bl)), Byte(List(ol))) => join!(Byte, bl, ol),
            (Short(List(bl)), Short(List(ol))) => join!(Short, bl, ol),
            (Int(List(bl)), Int(List(ol))) => join!(Int, bl, ol),
            (Long(List(bl)), Long(List(ol))) => join!(Long, bl, ol),
            (Real(List(bl)), Real(List(ol))) => join!(Real, bl, ol),
            (Float(List(bl)), Float(List(ol))) => join!(Float, bl, ol),
            (Symbol(List(bl)), Symbol(List(ol))) => join!(Symbol, bl, ol),
            (Timestamp(List(bl)), Timestamp(List(ol))) => {
                join!(Timestamp, bl, ol)
            }
            (Month(List(bl)), Month(List(ol))) => join!(Month, bl, ol),
            (Date(List(bl)), Date(List(ol))) => join!(Date, bl, ol),
            (Datetime(List(bl)), Datetime(List(ol))) => {
                join!(Datetime, bl, ol)
            }
            (Timespan(List(bl)), Timespan(List(ol))) => {
                join!(Timespan, bl, ol)
            }
            (Minute(List(bl)), Minute(List(ol))) => join!(Minute, bl, ol),
            (Second(List(bl)), Second(List(ol))) => join!(Second, bl, ol),
            (Time(List(bl)), Time(List(ol))) => join!(Time, bl, ol),
            (Enum(List(bl), bs), Enum(List(ol), os)) => {
                join!(Enum, bl, ol, bs.or(os))
            }
            _ => Result::Err("not a list or types do not match\0"),
        }
    }

    /// Create a list variant from an atom
    ///
    /// causes allocations
    ///
    /// takes ownwership of self, and returns either a new list variant, or self unchanged
    ///
    /// # Note
    /// * if the object is already a list, it will be returned unchanged
    /// * if the object is an atom, it will be converted to a list (this involved cloning the atom)
    /// * if the object is any other variant, an error will be returned
    /// * if the object is a symbol atom, it will be cloned
    #[inline] // because there are large pattern matches, this is a good candidate for inlining to enable more robust compiler optimizations
    pub fn to_list(self) -> Result<Self, &'static str> {
        use KData::*; // for brevity
        use KVal::*; // for brevity

        macro_rules! to_list {
            // for normal types
            ($kdata:ident, $ktype:path) => {
                match $kdata {
                    Atom(atom) => Ok($ktype(List(Cow::Owned(vec![atom.into_owned()])))),
                    List(list) => Ok($ktype(List(list.to_owned()))),
                }
            };
            // for enums
            ($kdata:ident, $ktype_unused:path, $enum_source:expr) => {
                match $kdata {
                    Atom(atom) => Ok(Enum(
                        List(Cow::Owned(vec![atom.into_owned()])),
                        $enum_source,
                    )),
                    List(list) => Ok(Enum(
                        List(list.to_owned()),
                        $enum_source.or_else(|| unimplemented!("an enum list must have a source")),
                    )),
                }
            };
        }

        match self {
            CompoundList(list) => Ok(CompoundList(list.to_owned())),
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
            Enum(data, src) => to_list!(data, Enum, src),
            _ => Result::Err("invalid type\0"),
        }
    }

    /// Convert this value back into a K value,
    ///
    /// # Note
    /// * uses methods from the native q api to create NEW K objects from the data in this value.
    /// * consumes self, this is deliberate as we don't want multiple references to the same data
    /// * this function should be used to create NEW K objects from manually initialized KVals
    /// * symbols need to be converted from strings to symbols before they can be made into K objects, this is done automatically but means symbol lists are slightly slower to convert than other types of lists
    /// * stored enum source takes precedence over the enum_source parameter
    ///
    /// # Safety
    /// * this function assumes that it will only be called to return a K object to q, and that q will be responsible for freeing the memory (especially true for Symbols), if this is not the case, memory leaks may occur
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
                    .copy_from_slice($from_list.into_owned().as_slice());
                k.cast_const()
            }};
        }

        match self {
            KVal::CompoundList(list) => {
                let k = re_exports::new_list(qtype::COMPOUND_LIST, list.len().try_into().unwrap())
                    .cast_mut();
                unsafe { &mut *k }
                    .as_mut_slice::<*mut K>()
                    .unwrap()
                    .copy_from_slice(
                        list.into_iter()
                            .map(|kv| kv.to_k().cast_mut())
                            .collect::<Vec<_>>()
                            .as_slice(),
                    );
                k.cast_const()
            }
            KVal::Bool(KData::Atom(atom)) => re_exports::new_bool(atom.into_owned()),
            KVal::Bool(KData::List(list)) => list_to_k!(bool, qtype::BOOL_LIST, list),
            KVal::Guid(KData::Atom(atom)) => re_exports::new_guid(atom.into_owned()),
            KVal::Guid(KData::List(list)) => list_to_k!([u8; 16], qtype::GUID_LIST, list),
            KVal::Byte(KData::Atom(atom)) => re_exports::new_byte((atom.into_owned()).into()),
            KVal::Byte(KData::List(list)) => list_to_k!(u8, qtype::BYTE_LIST, list),
            KVal::Short(KData::Atom(atom)) => re_exports::new_short((atom.into_owned()).into()),
            KVal::Short(KData::List(list)) => list_to_k!(i16, qtype::SHORT_LIST, list),
            KVal::Int(KData::Atom(atom)) => re_exports::new_int(atom.into_owned()),
            KVal::Int(KData::List(list)) => list_to_k!(i32, qtype::INT_LIST, list),
            KVal::Long(KData::Atom(atom)) => re_exports::new_long(atom.into_owned()),
            KVal::Long(KData::List(list)) => list_to_k!(i64, qtype::LONG_LIST, list),
            KVal::Real(KData::Atom(atom)) => re_exports::new_real((atom.into_owned()).into()),
            KVal::Real(KData::List(list)) => list_to_k!(f32, qtype::REAL_LIST, list),
            KVal::Float(KData::Atom(atom)) => re_exports::new_float(atom.into_owned()),
            KVal::Float(KData::List(list)) => list_to_k!(f64, qtype::FLOAT_LIST, list),
            KVal::Symbol(KData::Atom(atom)) => re_exports::new_symbol(atom.as_str()),
            KVal::Symbol(KData::List(list)) => {
                let k = re_exports::new_list(qtype::SYMBOL_LIST, list.len().try_into().unwrap())
                    .cast_mut();

                unsafe { &mut *k }
                    .as_mut_slice::<S>()
                    .unwrap()
                    .copy_from_slice(
                        list.iter()
                            .map(|s| unsafe {
                                re_exports::enumerate(
                                    CString::new(s.as_str())
                                        .expect("CString::new failed")
                                        .into_raw(),
                                )
                            })
                            .collect::<Vec<_>>()
                            .as_slice(),
                    );
                k.cast_const()
            }
            KVal::Timestamp(KData::Atom(atom)) => re_exports::new_timestamp(atom.into_owned()),
            KVal::Timestamp(KData::List(list)) => list_to_k!(i64, qtype::TIMESTAMP_LIST, list),
            KVal::Month(KData::Atom(atom)) => re_exports::new_month(atom.into_owned()),
            KVal::Month(KData::List(list)) => list_to_k!(i32, qtype::MONTH_LIST, list),
            KVal::Date(KData::Atom(atom)) => re_exports::new_date(atom.into_owned()),
            KVal::Date(KData::List(list)) => list_to_k!(i32, qtype::DATE_LIST, list),
            KVal::Datetime(KData::Atom(atom)) => re_exports::new_datetime(atom.into_owned()),
            KVal::Datetime(KData::List(list)) => list_to_k!(f64, qtype::DATETIME_LIST, list),
            KVal::Timespan(KData::Atom(atom)) => re_exports::new_timespan(atom.into_owned()),
            KVal::Timespan(KData::List(list)) => list_to_k!(i64, qtype::TIMESPAN_LIST, list),
            KVal::Minute(KData::Atom(atom)) => re_exports::new_minute(atom.into_owned()),
            KVal::Minute(KData::List(list)) => list_to_k!(i32, qtype::MINUTE_LIST, list),
            KVal::Second(KData::Atom(atom)) => re_exports::new_second(atom.into_owned()),
            KVal::Second(KData::List(list)) => list_to_k!(i32, qtype::SECOND_LIST, list),
            KVal::Time(KData::Atom(atom)) => re_exports::new_time(atom.into_owned()),
            KVal::Time(KData::List(list)) => list_to_k!(i32, qtype::TIME_LIST, list),
            KVal::Enum(KData::Atom(atom), src) => re_exports::new_enum(
                src.unwrap_or_else(|| {
                    unimplemented!("you need to pass/set an enum source to create an enum atom")
                }),
                atom.into_owned(),
            ),
            KVal::Enum(KData::List(list), _) => list_to_k!(i64, qtype::ENUM_LIST, list),
            KVal::Char(atom) => re_exports::new_char(atom),
            KVal::String(list) => re_exports::new_string(&list),
            KVal::Error(err) => re_exports::new_error(&err),
            KVal::Null => re_exports::new_null(),
            KVal::Dictionary(dict) => unsafe {
                re_exports::new_dictionary(dict.keys.to_k(), dict.values.to_k())
            },
            KVal::Table(table) => unsafe {
                re_exports::flip(re_exports::new_dictionary(
                    table.dict.keys.to_k(),
                    table.dict.values.to_k(),
                ))
            },
        }
    }

    /// Get the length of q object. The meaning of the returned value varies according to the type:
    /// - atom: 1
    /// - list: The number of elements in the list.
    /// - string: The number of characters in the string.
    /// - table: The number of rows.
    /// - dictionary: The number of keys.
    /// - general null: 1
    /// # Example
    /// ```no_run
    /// use kdbplus::rusty_api::*;
    /// use kdbplus::rusty_api::types::*;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn numbers(obj: *const K) -> *const K{
    ///   let count=format!("{} people are in numbers", KVal::from_raw(obj,None).len());
    ///   new_string(&count)
    /// }
    /// ```
    /// ```q
    /// q)census: `libapi_examples 2: (`numbers; 1);
    /// q)census[(::)]
    /// "1 people are in numbers"
    /// q)census[til 4]
    /// "4 people are in numbers"
    /// q)census[`a`b!("many"; `split`asunder)]
    /// "2 people are in numbers"
    /// q)census[([] id: til 1000)]
    /// "1000 people are in numbers"
    /// ```
    pub fn len(&self) -> i64 {
        use KVal::*; // for brevity

        match self {
            CompoundList(list) => list.len().try_into().unwrap(),
            Bool(data) => data.len(),
            Guid(data) => data.len(),
            Byte(data) => data.len(),
            Short(data) => data.len(),
            Int(data) => data.len(),
            Long(data) => data.len(),
            Real(data) => data.len(),
            Float(data) => data.len(),
            Char(_) => 1,
            Symbol(data) => data.len(),
            Timestamp(data) => data.len(),
            Month(data) => data.len(),
            Date(data) => data.len(),
            Datetime(data) => data.len(),
            Timespan(data) => data.len(),
            Minute(data) => data.len(),
            Second(data) => data.len(),
            Time(data) => data.len(),
            Enum(data, _) => data.len(),
            String(string) => string.len().try_into().unwrap(),
            Error(_) => 1,
            Table(table) => table.len(),
            Dictionary(dict) => dict.len(),
            Null => 1,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// is the kval a list?
    ///
    /// TODO: example
    ///
    pub fn is_list(&self) -> bool {
        use KData::*;
        use KVal::*; // for brevity // for brevity
        matches!(
            self,
            CompoundList(_)
                | Bool(List(_))
                | Guid(List(_))
                | Byte(List(_))
                | Short(List(_))
                | Int(List(_))
                | Long(List(_))
                | Real(List(_))
                | Float(List(_))
                | Symbol(List(_))
                | Timestamp(List(_))
                | Month(List(_))
                | Date(List(_))
                | Datetime(List(_))
                | Timespan(List(_))
                | Minute(List(_))
                | Second(List(_))
                | Time(List(_))
                | Enum(List(_), _)
                | String(_)
        )
    }

    /// is the object an atom?
    ///
    /// TODO: example
    ///
    pub fn is_atom(&self) -> bool {
        use KData::*;
        use KVal::*; // for brevity // for brevity
        matches!(
            self,
            Char(_)
                | Bool(Atom(_))
                | Guid(Atom(_))
                | Byte(Atom(_))
                | Short(Atom(_))
                | Int(Atom(_))
                | Long(Atom(_))
                | Real(Atom(_))
                | Float(Atom(_))
                | Symbol(Atom(_))
                | Timestamp(Atom(_))
                | Month(Atom(_))
                | Date(Atom(_))
                | Datetime(Atom(_))
                | Timespan(Atom(_))
                | Minute(Atom(_))
                | Second(Atom(_))
                | Time(Atom(_))
                | Enum(Atom(_), _)
        )
    }
}
