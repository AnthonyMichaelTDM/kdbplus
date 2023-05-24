use std::borrow::Cow;

use crate::rusty_api::K;

use super::{KData, KDict, KVal};

/// representation of a K table, which is itself a wrapper for a K dictionary where the keys are symbols and the values are lists
#[derive(Debug, Clone)]
pub struct KTable<'a> {
    pub dict: KDict<'a>,
}

impl<'a> KTable<'a> {
    /// constructor for KTable from a K object
    ///
    /// # Example
    ///
    /// TODO: add example
    pub(super) fn new_from_k(k: &'a K) -> Self {
        Self {
            dict: KDict::new_from_k(unsafe { &**k.cast::<*mut K>() }),
        }
    }

    /// constructor for KTable
    /// kval must be a dictionary with symbols as keys and lists as values, all the values must be the same length
    ///
    /// this constructor does not fully check these conditions for performance reasons, but other methods will panic or error if they aren't met
    ///
    /// the only condition this is not checked is that all the values are the same length, this is for performance reasons because the other checks are O(1) and this would be O(columns)
    ///
    pub fn new(kdict: KDict<'a>) -> KTable<'a> {
        // check that keys is a symbol list
        assert!(
            matches!(*kdict.keys, KVal::Symbol(KData::List(_))),
            "invalid keys, must be a symbol list"
        );
        // check that values is a compound list
        assert!(
            matches!(*kdict.values, KVal::CompoundList(_)),
            "invalid values, must be a compound list"
        );
        // check that all elements of values are the same length, ommitted for performance reasons
        // assert!( {
        //     let KVal::CompoundList(columns) = *kdict.values else {unimplemented!()};
        //     let len = columns[0].len();
        //     columns.iter().all(|x| x.len() == len)
        // }, "invalid values, all columns must be the same length" );

        KTable {
            dict: kdict,
        }
    }

    /// get a column of the table by index
    ///
    /// # Note
    /// * given index must be in range of the table
    /// * for enumerated columns, the given enum_sources will take precidence over any existing enum_sources,
    ///   * will panic if, for a enumerated column, neither enum_sources nor existing sources are provided
    ///
    /// # Example
    /// TODO: validate / add example
    /// ```no_run
    /// use kdbplus::rusty_api::*;
    /// use kdbplus::rusty_api::types::*;
    /// use kdbplus::str_to_S;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn print_column(object: *const K, index: *const K) -> *const K{
    ///     match (KVal::from_raw(object, None), KVal::from_raw(index, None)) {
    ///         (KVal::Table(table), KVal::Long(KData::Atom(i))) => {
    ///             match table.get_column(*i, Some("sym")) {
    ///                 Some(column) => {
    ///                     let null = unsafe{native::k(0, str_to_S!("{-1 \"column: \", .Q.s1 x}"), column.to_k(), KNULL)};
    ///                     unsafe{decrement_reference_count(null)};
    ///                     KNULL
    ///                 }
    ///                 None => new_error("invalid column index\0")
    ///             }
    ///        }
    ///       _ => new_error("type error\0")
    ///    }
    /// }
    /// ```
    /// ```q
    /// q)col: `librusty_api_examples 2: (`print_column; 2)
    /// q)table: ([] time: asc `timestamp$.z.p + 3?1000000000; sym: -3?`Green`Yellow`Red; go: "oxx"; miscellaneous: ("cow"; `lion; "eagle"))
    /// q)col[table;2]
    /// column: `Yellow`Green`Red
    /// q)col[table;3]
    /// column: "oxx"
    /// ```
    pub fn get_column(&'a self, index: i64, enum_source: Option<&'a str>) -> Option<KVal<'a>> {
        let KVal::CompoundList(columns) = self.dict.values.as_ref() else {unimplemented!()};

        let column = columns.get(index as usize)?;

        assert!(column.is_list(), "columns must be lists, if you get this error from a K object from q, open an issue on github");

        match column {
            KVal::Enum(KData::List(enums), src) => Some(KVal::Enum(
                KData::List(enums.to_owned()),
                enum_source.or_else(|| {
                    src.or_else(|| {
                        unimplemented!("enum_source must be provided for enumerated columns")
                    })
                }),
            )),
            col => Some(col.to_owned()),
        }
    }

    /// get the number of rows in the table (length of the table)
    ///
    /// # Example
    /// ```no_run
    /// use kdbplus::rusty_api::*;
    /// use kdbplus::rusty_api::types::*;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn print_length(object: *const K) -> *const K{
    ///     match KVal::from_raw(object, None) {
    ///         KVal::Table(table) => {
    ///             println!("length: {}", table.len());
    ///             KNULL
    ///         }
    ///         _ => new_error("not a table\0")
    ///     }
    /// }
    /// ```
    /// ```q
    /// q)col: `librusty_api_examples 2: (`print_length; 1)
    /// q)table: ([] time: asc `timestamp$.z.p + 3?1000000000; sym: -3?`Green`Yellow`Red; go: "oxx"; miscellaneous: ("cow"; `lion; "eagle"))
    /// q)col[table]
    /// length: 3
    /// ```
    pub fn len(&self) -> i64 {
        if let KVal::CompoundList(columns) = self.dict.values.as_ref() {
            columns[0].len()
        } else {
            unimplemented!()
        }
    }

    /// Get a table row of the given index, as a CompoundList. For enumerated column, a names of a target `sym` list
    ///  to which symbol values are cast must be passed. In the example below, it is assumed that
    ///  there is a single enum column in a table and the column values are cast to a symbol list whose name is `sym`.
    ///    
    /// # Note
    /// * given index must be in range of the table
    /// * given enum_sources will take precidence over any existing enum_sources,
    ///   * will panic if, for a enumerated column, neither enum_sources nor existing sources are provided
    ///
    /// # Example
    ///
    /// ```no_run
    /// use kdbplus::rusty_api::*;
    /// use kdbplus::rusty_api::types::*;
    /// use kdbplus::str_to_S;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn print_row(object: *const K, index: *const K) -> *const K{
    ///     match (KVal::from_raw(object, None), KVal::from_raw(index, None)) {
    ///         (KVal::Table(table), KVal::Long(KData::Atom(i))) => {
    ///             match table.get_row(*i,&[Some("sym")]) {
    ///                 Some(row) => {
    ///                     let null = unsafe{native::k(0, str_to_S!("{-1 \"row: \", .Q.s1 x}"), row.to_k(), KNULL)};
    ///                     unsafe{decrement_reference_count(null)};
    ///                     KNULL
    ///                 },
    ///                 None => new_error("invalid row index\0")
    ///             }
    ///         },
    ///         _ => new_error("type error\0")
    ///     }
    /// }
    /// ```
    /// ```q
    /// q)row: `librusty_api_examples 2: (`print_row; 2)
    /// q)table: ([] time: asc `timestamp$.z.p + 3?1000000000; sym: -3?`Green`Yellow`Red; go: "oxx"; miscellaneous: ("cow"; `lion; "eagle"))
    /// q)row[table;2]
    /// row: `time`sym`go`miscellaneous!(2022.01.30D07:55:48.404520689;`Yellow;"x";"eagle")
    /// q)row[table;1]
    /// row: `time`sym`go`miscellaneous!(2022.01.30D07:55:47.987133353;`Green;"x";`lion)
    /// ```
    pub fn get_row(&'a self, index: i64, enum_sources: &[Option<&'a str>]) -> Option<KVal<'a>> {
        // check that index is in bounds
        if self.dict.len() > 0 && index >= self.get_column(0, enum_sources[0])?.len() {
            return None;
        }

        let mut row = Vec::with_capacity(self.dict.keys.len().try_into().unwrap());

        let values = self.dict.values.to_owned();

        match *values {
            KVal::CompoundList(cols) => {
                for (i, col) in cols.iter().enumerate() {
                    let enum_source = enum_sources[i];
                    row.push(match col {
                        KVal::Enum(KData::List(enums), src) => KVal::Enum(
                            KData::Atom(Cow::Owned(enums[i])),
                            enum_source.or_else(|| {
                                src.or_else(|| {
                                    unimplemented!(
                                        "enum_source must be provided for enumerated columns"
                                    )
                                })
                            }),
                        ),
                        colum => colum.to_owned(),
                    })
                }
                Some(KVal::CompoundList(row.to_owned()))
            }
            _ => None,
        }
    }
}
