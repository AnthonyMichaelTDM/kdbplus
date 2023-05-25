use rayon::prelude::*;
use std::borrow::Cow;

use crate::rusty_api::K;

use super::{KData, KDict, KVal};

/// representation of a K table, which is itself a wrapper for a K dictionary where the keys are symbols and the values are lists
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct KTable<'a> {
    pub dict: KDict<'a>,
}

impl<'a> KTable<'a> {
    /// constructor for KTable from a K object
    ///
    /// given K object must be a valid table (wrapper around a dictionary with symbols as keys and lists as values)
    ///
    /// when not in release mode, panics if the given K object is not of type
    /// [`TABLE`](crate::qtype::TABLE)
    ///
    /// # Example
    ///
    /// TODO: add example
    #[inline]
    pub(super) fn new_from_k(k: &'a K) -> Self {
        debug_assert!(k.qtype == crate::qtype::TABLE, "invalid qtype for KTable");

        let mut table = Self {
            dict: KDict::new_from_k(unsafe { &**k.cast::<*mut K>() }),
        };

        // because single column tables in q won't be wrapped in a compound list, we explicity check
        // that case and wrap it in a compound list if needed
        // we don't have to check that values is a list because in order to be a valid q TABLE, k
        // must wrap a dictionary whose values is a list.
        if let KVal::CompoundList(_) = table.dict.get_values() {
        } else {
            table.dict = KDict::new(
                table.dict.get_keys().to_owned(),
                KVal::CompoundList(vec![table.dict.get_values().to_owned()]),
            )
            .unwrap();
        }

        table
    }

    /// constructor for KTable
    ///
    /// * kdict.keys: the names of each column
    /// * kdict.values: the columns of the table
    ///
    /// # Note
    /// * kdict's keys must be a symbol list and kdict's values must be a compound list.
    /// * the elements of kdict.values (columns of the table) must themselves be lists, all of equal length
    ///
    /// will error if these conditions are not met,
    /// but the second condition is not checked in release mode for performance reasons, if it is
    /// not met then other methods will error
    /// this constructor does not fully check these conditions for performance reasons, but other methods will panic or error if they aren't met
    ///
    /// the only condition this is not checked is that all the values are the same length, this is for performance reasons because the other checks are O(1) and this would be O(columns)
    #[inline]
    pub fn new(kdict: KDict<'a>) -> Result<KTable<'a>, &'static str> {
        if let KVal::Symbol(KData::List(_)) = kdict.get_keys() {
        } else {
            return Err("keys must be a symbol list\0");
        }

        if let KVal::CompoundList(_) = kdict.get_values() {
        } else {
            return Err("columns must be in a compound list\0");
        }

        // check that all elements of values are the same length, ommitted for performance reasons
        // in optomized builds (debug_assertions is false)
        #[cfg(debug_assertions)]
        match kdict.get_values() {
            KVal::CompoundList(columns) => {
                let len = columns[0].len();
                if !columns.par_iter().all(|x| x.len() == len && x.is_list()) {
                    return Err("invalid table, all columns must be lists with the same length\0");
                }
            }
            _ => unreachable!(), // we previously check if values is a compound list, so this is
                                 // okay
        }

        Ok(KTable { dict: kdict })
    }

    /// get a column of the table by index
    ///
    /// # Note
    /// * given index must be in range of the table
    /// * for enumerated columns, the given enum_sources will take precidence over any existing enum_sources,
    ///   * will return an error if, for a enumerated column, neither enum_sources nor existing sources are provided
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
    ///                 Ok(column) => {
    ///                     let null = unsafe{native::k(0, str_to_S!("{-1 \"column: \", .Q.s1 x}"), column.to_k(), KNULL)};
    ///                     unsafe{decrement_reference_count(null)};
    ///                     KNULL
    ///                 }
    ///                 Err(err) => new_error(err)
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
    #[inline]
    pub fn get_column(
        &'a self,
        index: i64,
        enum_source: Option<&'a str>,
    ) -> Result<KVal<'a>, &'static str> {
        let column = match self.dict.values.as_ref() {
            KVal::CompoundList(columns) => columns
                .get(index as usize)
                .ok_or("invalid column index\0")?,
            _ => return Err("values must be a compound list\0"), // TODO: this may be unreachable
                                                                 // because of the check in the constructor
        };

        // ensure that the column is a list
        // TODO: this may be unreachable because of the check in the constructor
        if !column.is_list() {
            return Err("columns must be lists\0");
        }

        match column {
            KVal::Enum(KData::List(enums), src) => {
                let source = enum_source
                    .or(*src)
                    .ok_or("enum_source must be provided for enumerated columns\0")?;
                Ok(KVal::Enum(
                    KData::List(Cow::Owned(enums.clone().into_owned())),
                    Some(source),
                ))
            }
            col => Ok(col.to_owned()),
        }
    }

    /// get the number of rows in the table (length of the table)
    ///
    /// panics if columns are not stored in a compound list, which should never happen because the
    /// constructor enforces that columns are stored in a compound list
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
    #[inline]
    pub fn len(&self) -> i64 {
        match self.dict.values.as_ref() {
            KVal::CompoundList(columns) if !columns.is_empty() => columns[0].len(),
            KVal::CompoundList(_) => 0_i64,
            _ => unreachable!("values must be a compound list\0"),
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get a table row of the given index, as a Dictionary. For enumerated column, a names of a target `sym` list
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
    ///                 Ok(row) => {
    ///                     let null = unsafe{native::k(0, str_to_S!("{-1 \"row: \", .Q.s1 x}"), row.to_k(), KNULL)};
    ///                     unsafe{decrement_reference_count(null)};
    ///                     KNULL
    ///                 },
    ///                 Err(err) => new_error(err)
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
    #[inline]
    pub fn get_row(
        &'a self,
        index: i64,
        enum_sources: &[Option<&'a str>],
    ) -> Result<KVal<'a>, &'static str> {
        if !self.dict.is_empty()
            && !enum_sources.is_empty()
            && index >= self.get_column(0, enum_sources[0])?.len()
        {
            return Err("index out of bounds\0");
        }

        // macro to get the value of a column in row `index`
        macro_rules! atom_from_column_at_row {
            ($list:ident, $variant:path) => {
                $variant(KData::Atom(Cow::Owned(
                    $list
                        .get(index as usize)
                        .ok_or("index out of bounds, columns were not the same length\0")?
                        .to_owned(),
                )))
            };
            ($list:ident, $variant:path, $enum_src:expr) => {
                $variant(
                    KData::Atom(Cow::Owned(
                        $list
                            .get(index as usize)
                            .ok_or("index out of bounds, columns were not the same length\0")?
                            .to_owned(),
                    )),
                    $enum_src,
                )
            };
        }

        let KDict { keys, values } = self.dict.to_owned();

        match *values {
            KVal::CompoundList(columns) => {
                let mut row: Vec<KVal> =
                    Vec::with_capacity(self.dict.keys.len().try_into().unwrap());
                let mut enum_index = 0;

                // could probably try to parallelize this, but it's not a bottleneck, and dealing
                // with atomics might cause more overhead than it's worth
                for column in columns.iter() {
                    let value_of_column_at_row: KVal = match column {
                        KVal::Enum(KData::List(enumerated_column), src) => {
                            let enum_source = enum_sources
                                .get(enum_index)
                                .unwrap_or(src)
                                .ok_or("enum source must be provided for enumerated columns\0")?;
                            enum_index += 1;
                            atom_from_column_at_row!(
                                enumerated_column,
                                KVal::Enum,
                                Some(enum_source)
                            )
                        }
                        KVal::CompoundList(column) => column
                            .get(index as usize)
                            .ok_or("index out of bounds, columns were not the same length\0")?
                            .to_owned(),
                        KVal::Bool(KData::List(column)) => {
                            atom_from_column_at_row!(column, KVal::Bool)
                        }
                        KVal::Guid(KData::List(column)) => {
                            atom_from_column_at_row!(column, KVal::Guid)
                        }
                        KVal::Byte(KData::List(column)) => {
                            atom_from_column_at_row!(column, KVal::Byte)
                        }
                        KVal::Short(KData::List(column)) => {
                            atom_from_column_at_row!(column, KVal::Short)
                        }
                        KVal::Int(KData::List(column)) => {
                            atom_from_column_at_row!(column, KVal::Int)
                        }
                        KVal::Long(KData::List(column)) => {
                            atom_from_column_at_row!(column, KVal::Long)
                        }
                        KVal::Real(KData::List(column)) => {
                            atom_from_column_at_row!(column, KVal::Real)
                        }
                        KVal::Float(KData::List(column)) => {
                            atom_from_column_at_row!(column, KVal::Float)
                        }
                        KVal::String(str) => KVal::Char(
                            str.chars()
                                .nth(index as usize)
                                .ok_or("index out of bounds, columns were not the same length\0")?,
                        ),
                        KVal::Symbol(KData::List(column)) => {
                            atom_from_column_at_row!(column, KVal::Symbol)
                        }
                        KVal::Timestamp(KData::List(column)) => {
                            atom_from_column_at_row!(column, KVal::Timestamp)
                        }
                        KVal::Month(KData::List(column)) => {
                            atom_from_column_at_row!(column, KVal::Month)
                        }
                        KVal::Date(KData::List(column)) => {
                            atom_from_column_at_row!(column, KVal::Date)
                        }
                        KVal::Datetime(KData::List(column)) => {
                            atom_from_column_at_row!(column, KVal::Datetime)
                        }
                        KVal::Timespan(KData::List(column)) => {
                            atom_from_column_at_row!(column, KVal::Timespan)
                        }
                        KVal::Minute(KData::List(column)) => {
                            atom_from_column_at_row!(column, KVal::Minute)
                        }
                        KVal::Second(KData::List(column)) => {
                            atom_from_column_at_row!(column, KVal::Second)
                        }
                        KVal::Time(KData::List(column)) => {
                            atom_from_column_at_row!(column, KVal::Time)
                        }
                        _ => Err("columns must each be lists, within a compound list\0")?,
                    };
                    row.push(value_of_column_at_row)
                }
                Ok(KVal::Dictionary(KDict::new(
                    *keys,
                    KVal::CompoundList(row.to_owned()),
                )?))
            }
            _ => Err("values must be a compound list\0"),
        }
    }
}
