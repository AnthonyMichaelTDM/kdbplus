use crate::rusty_api::K;

use super::KVal;

/// representation of a K dictionary, which is itself a slice of 2 K lists of equal length
/// where the first list contains the keys and the second list contains the values
#[derive(Debug, Clone)]
pub struct KDict<'a> {
    pub keys: Box<KVal<'a>>,
    pub values: Box<KVal<'a>>,
    _private: (), // prevent construction outside of this module
}

impl<'a> KDict<'a> {
    /// constructor for a KDict from a K object
    ///
    /// # Example
    ///
    /// TODO: add example
    pub(super) fn new_from_k(k: &'a K) -> KDict<'a> {
        assert!(
            k.qtype == crate::qtype::DICTIONARY || k.qtype == crate::qtype::SORTED_DICTIONARY,
            "invalid k object, must be a dictionary"
        );
        let slice = k.as_slice::<*mut K>().unwrap();
        assert!(
            slice.len() == 2,
            "invalid dictionary, must be a list of two items"
        );
        Self {
            keys: Box::new(KVal::from_raw(slice[0].cast_const(), None)),
            values: Box::new(KVal::from_raw(slice[1].cast_const(), None)),
            _private: (),
        }
    }

    /// constructor for a KDict,
    /// keys must be a list,
    /// values must be a list of equal length to keys
    /// panics if these conditions are not met
    ///
    /// # Example
    ///
    /// TODO: add example
    pub fn new(keys: KVal<'a>, values: KVal<'a>) -> KDict<'a> {
        assert!(keys.is_list(), "invalid keys, must be a list");
        assert!(values.is_list(), "invalid values, must be a list");
        assert_eq!(
            keys.len(),
            values.len(),
            "invalid dictionary, keys and values must be of equal length"
        );

        KDict {
            keys: Box::new(keys),
            values: Box::new(values),
            _private: (),
        }
    }

    /// get the Keys list of the dictionary
    ///
    /// # Example
    ///
    /// TODO: add example
    pub fn get_keys(&self) -> &KVal<'a> {
        &self.keys
    }

    /// get the Values list of the dictionary
    ///
    /// # Example
    ///
    /// TODO: add example
    pub fn get_values(&self) -> &KVal<'a> {
        &self.values
    }

    /// get the length of the dictionary
    ///
    /// # Example
    ///
    /// TODO: add example
    pub fn len(&self) -> i64 {
        self.keys.len()
    }
}
