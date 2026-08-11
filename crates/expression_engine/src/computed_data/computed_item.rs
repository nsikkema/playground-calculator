use shareable_string::ShareableString;
use std::fmt;
use units::UnitId;

/// Represents a computed table, consisting of column keys and rows of numeric values.
#[derive(Debug, Clone, PartialEq)]
pub struct ComputedTable {
    /// The column names of the table, in order.
    keys: Vec<ShareableString>,
    /// The evaluated numeric rows; each inner `Vec<f64>` corresponds to one row.
    rows: Vec<Vec<f64>>,
}

impl ComputedTable {
    /// Creates a new unitless `ComputedTable` with the given column `keys` and numeric `rows`.
    pub(crate) const fn new(keys: Vec<ShareableString>, rows: Vec<Vec<f64>>) -> Self {
        Self { keys, rows }
    }

    /// Returns a reference to the keys of the computed table.
    #[must_use]
    pub fn keys(&self) -> &[ShareableString] {
        &self.keys
    }

    /// Returns a reference to the rows of the computed table.
    #[must_use]
    pub fn rows(&self) -> &[Vec<f64>] {
        &self.rows
    }

    /// Returns the value of a cell by row and column index.
    #[must_use]
    pub fn get_cell(&self, row_index: usize, column_index: usize) -> Option<f64> {
        if let Some(row) = self.rows.get(row_index) {
            if let Some(&value) = row.get(column_index) {
                return Some(value);
            }
        }

        None
    }

    /// Returns the value of a cell by row index and column name.
    pub fn get_cell_by_name<S: Into<ShareableString>>(
        &self,
        row_index: usize,
        column_name: S,
    ) -> Option<f64> {
        let column_name = column_name.into();
        if let Some(column_index) = self.keys.iter().position(|key| key == &column_name) {
            self.get_cell(row_index, column_index)
        } else {
            None
        }
    }

    /// Returns the number of rows in the computed table.
    #[must_use]
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Returns the number of columns in the computed table.
    #[must_use]
    pub fn column_count(&self) -> usize {
        self.keys.len()
    }
}

/// Represents a computed table with canonical units for one or more columns.
#[derive(Debug, Clone, PartialEq)]
pub struct ComputedTableWithUnits {
    /// The table's column keys and numeric rows.
    table: ComputedTable,
    /// The canonical unit for each column, in the same order as [`Self::keys`].
    units: Vec<UnitId>,
}

impl ComputedTableWithUnits {
    /// Creates a computed table with units for at least one column.
    pub(crate) fn new(keys: Vec<ShareableString>, units: Vec<UnitId>, rows: Vec<Vec<f64>>) -> Self {
        debug_assert_eq!(keys.len(), units.len());
        debug_assert!(units.iter().any(|unit| *unit != UnitId::None));

        Self {
            table: ComputedTable::new(keys, rows),
            units,
        }
    }

    /// Returns the unitless table data shared by this unit-bearing table.
    #[must_use]
    pub(crate) const fn as_table(&self) -> &ComputedTable {
        &self.table
    }

    /// Splits this table into its numeric data and column units.
    #[must_use]
    pub(crate) fn into_table_and_units(self) -> (ComputedTable, Vec<UnitId>) {
        (self.table, self.units)
    }

    /// Discards column units and returns this table's numeric data.
    #[must_use]
    pub(crate) fn into_table(self) -> ComputedTable {
        self.table
    }

    /// Returns a reference to the keys of the computed table.
    #[must_use]
    pub fn keys(&self) -> &[ShareableString] {
        self.table.keys()
    }

    /// Returns the canonical units of the table columns, in key order.
    #[must_use]
    pub fn units(&self) -> &[UnitId] {
        &self.units
    }

    /// Returns the unit associated with a column index.
    #[must_use]
    pub fn get_unit(&self, column_index: usize) -> Option<UnitId> {
        self.units.get(column_index).copied()
    }

    /// Returns the unit associated with a column name.
    #[must_use]
    pub fn get_unit_by_name<S: Into<ShareableString>>(&self, column_name: S) -> Option<UnitId> {
        let column_name = column_name.into();
        self.table
            .keys()
            .iter()
            .position(|key| key == &column_name)
            .and_then(|column_index| self.get_unit(column_index))
    }

    /// Returns a reference to the rows of the computed table.
    #[must_use]
    pub fn rows(&self) -> &[Vec<f64>] {
        self.table.rows()
    }

    /// Returns the value of a cell by row and column index.
    #[must_use]
    pub fn get_cell(&self, row_index: usize, column_index: usize) -> Option<f64> {
        self.table.get_cell(row_index, column_index)
    }

    /// Returns the value of a cell by row index and column name.
    pub fn get_cell_by_name<S: Into<ShareableString>>(
        &self,
        row_index: usize,
        column_name: S,
    ) -> Option<f64> {
        self.table.get_cell_by_name(row_index, column_name)
    }

    /// Returns the number of rows in the computed table.
    #[must_use]
    pub fn row_count(&self) -> usize {
        self.table.row_count()
    }

    /// Returns the number of columns in the computed table.
    #[must_use]
    pub fn column_count(&self) -> usize {
        self.table.column_count()
    }
}

/// Represents a computed item that can be a float, string, or table.
#[derive(Debug, Clone, PartialEq)]
pub enum ComputedItem {
    /// A boolean value.
    Boolean(bool),
    /// An integer value.
    Integer(i64),
    /// A floating-point number.
    Float(f64),
    /// A floating-point number with a concrete unit.
    FloatWithUnit {
        /// The numeric value, expressed in `unit`.
        value: f64,
        /// The concrete unit associated with `value`.
        unit: UnitId,
    },
    /// A String value.
    String(ShareableString),
    /// An Identifier value.
    Identifier(ShareableString),
    /// Path to a file.
    File(ShareableString),
    /// A table represented as a `ComputedTable`.
    Table(ComputedTable),
    /// A table represented as a `ComputedTableWithUnits`.
    TableWithUnits(ComputedTableWithUnits),
    /// A unit identifier.
    Unit(UnitId),
}

impl fmt::Display for ComputedItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComputedItem::Boolean(value) => write!(f, "{value}"),
            ComputedItem::Integer(value) => write!(f, "{value}"),
            ComputedItem::Float(value) => write!(f, "{value}"),
            ComputedItem::FloatWithUnit { value, unit } => write!(f, "{value} {unit:?}"),
            ComputedItem::String(value)
            | ComputedItem::File(value)
            | ComputedItem::Identifier(value) => write!(f, "{value}"),
            ComputedItem::Table(_) | ComputedItem::TableWithUnits(_) => write!(f, "{self:?}"),
            ComputedItem::Unit(value) => write!(f, "{value:?}"),
        }
    }
}
