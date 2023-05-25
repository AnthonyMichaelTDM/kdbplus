use crate::rusty_api::K;

use super::KVal;

/// representation of a K dictionary, which is itself a slice of 2 K lists of equal length
/// where the first list contains the keys and the second list contains the values
#[derive(Debug, Clone)]
#[non_exhaustive] // prevent construction outside of this module
pub struct KDict<'a> {
    pub keys: Box<KVal<'a>>,
    pub values: Box<KVal<'a>>,
}

impl<'a> KDict<'a> {
    /// constructor for a KDict from a K object,
    ///
    /// K object must be a valid dictionary,
    /// if it isn't it could cause undefined behavior in release mode or panic otherwise
    ///
    /// only called by the KVal::from method,
    /// which should guarentee that the passed K object is a valid dictionary,
    /// therefore checks are only performed when debug_assertions
    ///
    ///
    /// Errors if the K object is not a valid dictionary
    #[inline]
    pub(super) fn new_from_k(k: &'a K) -> KDict<'a> {
        debug_assert!(
            // okay to panic because this isn't public api and it's conditions should be met
            // by the constructor
            k.qtype == crate::qtype::DICTIONARY || k.qtype == crate::qtype::SORTED_DICTIONARY,
            "invalid k object, must be a dictionary"
        );
        let slice = k.as_slice::<*mut K>().unwrap();
        debug_assert!(
            // this should never happen
            slice.len() == 2,
            "invalid dictionary, must be a list of two items"
        );
        Self {
            keys: Box::new(KVal::from_raw(slice[0].cast_const(), None)),
            values: Box::new(KVal::from_raw(slice[1].cast_const(), None)),
        }
    }

    /// constructor for a KDict,
    /// keys must be a list,
    /// values must be a list,
    /// keys and values must be of equal length
    ///
    /// errors if these conditions are not met
    ///
    /// # Example
    ///
    /// TODO: add example
    #[inline]
    pub fn new(keys: KVal<'a>, values: KVal<'a>) -> Result<KDict<'a>, &'static str> {
        if !keys.is_list() {
            return Err("invalid keys, must be a list\0");
        }
        if !values.is_list() {
            return Err("invalid values, must be a list\0");
        }
        if keys.len() != values.len() {
            return Err("invalid dictionary, keys and values must be of equal length\0");
        }
        Ok(KDict {
            keys: Box::new(keys),
            values: Box::new(values),
        })
    }

    /// get the Keys list of the dictionary
    ///
    /// # Example
    ///
    /// TODO: add example
    #[inline]
    pub fn get_keys(&self) -> &KVal<'a> {
        &self.keys
    }

    /// get the Values list of the dictionary
    ///
    /// # Example
    ///
    /// TODO: add example
    #[inline]
    pub fn get_values(&self) -> &KVal<'a> {
        &self.values
    }

    /// get the length of the dictionary
    ///
    /// # Example
    ///
    /// TODO: add example
    #[inline]
    pub fn len(&self) -> i64 {
        self.keys.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
