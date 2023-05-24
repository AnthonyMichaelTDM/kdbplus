use std::{borrow::Cow, ffi::CStr};

use crate::rusty_api::{SafeToCastFromKInner, K, S};

/// Rust friendly wrapper for q Atoms and Lists.
/// references are mutable to indicate that changes should propagate back to q.
#[derive(Debug, Clone)]
pub enum KData<'a, T>
where
    T: std::fmt::Debug + Clone,
    [T]: ToOwned<Owned = Vec<T>>,
{
    /// wrapper for q atoms
    /// Clone On Write (Cow) to allow zero copy when possible without sacrificing safety, and to allow for Atoms that are owned types (i.e. symbols)
    Atom(Cow<'a, T>), // TODO: Should this be mut, const, or neither?

    //List(&'a [T]), // TODO: Should this be mut, const, or neither?
    /// wrapper for q lists
    /// Clone On Write (Cow) to allow zero copy when possible without sacrificing safety, and to allow for ownership when necessary (i.e. merging 2 lists)
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
    pub(super) fn atom(k: &'a K) -> KData<'a, T> {
        KData::Atom(Cow::Borrowed(k.cast()))
    }

    #[inline]
    /// # Safety
    /// k must be a valid pointer to a valid K object
    pub(super) fn guid_atom(k: &'a K) -> KData<'a, T> {
        KData::Atom(Cow::Borrowed(k.cast_with_ptr_offset())) // while this is an atom, it is packed into a list of 1
    }

    #[inline]
    /// # Safety
    /// same requirements as [`K::as_slice`](crate::rusty_api)
    /// but, additionally k must be a list of type T
    pub(super) fn list(k: &'a K) -> KData<'a, T> {
        KData::List(Cow::Borrowed(k.as_slice().unwrap()))
    }
}

/// for Symbols, which are backed by a [`S`] (*mut c_char) (a pointer) and therefore need to be converted to a owned String to guarentee memory safety
impl<'a> KData<'a, String> {
    #[inline]
    /// # Safety
    /// k must be a valid pointer to a valid K object
    pub(super) fn symbol(k: &'a K) -> KData<'a, String> {
        KData::Atom(Cow::Owned(
            String::from_utf8_lossy(
                unsafe { CStr::from_ptr(k.cast::<S>().cast_const()) }.to_bytes(),
            )
            .to_string(),
        ))
    }

    #[inline]
    /// # Safety
    /// k must be a valid pointer to a valid K object
    pub(super) fn symbol_list(k: &'a K) -> KData<'a, String> {
        KData::List(Cow::Owned(
            k.as_slice::<S>()
                .unwrap()
                .iter()
                .map(|s| {
                    String::from_utf8_lossy(unsafe { CStr::from_ptr(*s) }.to_bytes()).to_string()
                })
                .collect::<Vec<String>>(),
        ))
    }
}

/// generic utility functions
impl<'a, T: std::fmt::Debug + Clone> KData<'a, T> {
    pub fn len(&self) -> i64 {
        match self {
            KData::Atom(_) => 1,
            KData::List(l) => l.len().try_into().unwrap(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            KData::Atom(_) => false,
            KData::List(l) => l.is_empty(),
        }
    }
}
