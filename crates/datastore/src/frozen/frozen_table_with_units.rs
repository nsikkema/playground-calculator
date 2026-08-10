use crate::definition::TableWithUnitsDefinition;
use crate::editable::TableWithUnitsEditable;
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;

/// Represents a table of number-with-units data in the frozen data.
///
/// Cell values are stored as plain strings (`rows`). Units are stored once per
/// column (`units`), in definition column order, rather than per cell.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableWithUnitsFrozen {
    /// Definition metadata for this table value.
    definition: TableWithUnitsDefinition,
    /// One unit string per column, in definition column order.
    units: Vec<ShareableString>,
    /// Row data; each inner `Vec` holds one plain-string value per column.
    rows: Vec<Vec<ShareableString>>,
    /// Parameter key associated with this table instance.
    parameter: ShareableString,
    /// Pre-computed BLAKE3 hash of all rows and column units for fast diffing.
    hash: [u8; 32],
}

impl TableWithUnitsFrozen {
    /// Creates a new `TableWithUnitsFrozen` with a definition.
    ///
    /// Column units are initialised from each column's preferred unit in
    /// definition order; rows start empty.
    #[must_use]
    pub fn new(definition: TableWithUnitsDefinition) -> Self {
        let units = definition
            .iter()
            .map(|(_, col_def)| col_def.preferred_units().string_id().into())
            .collect();
        let mut table = Self {
            definition,
            units,
            rows: Vec::new(),
            parameter: ShareableString::new(""),
            hash: [0u8; 32],
        };
        table.update_hash();
        table
    }

    /// Creates a new `TableWithUnitsFrozen` with a definition, rows, and column units.
    ///
    /// `units` must contain one entry per column in definition column order.
    #[must_use]
    pub fn new_from_rows(
        definition: TableWithUnitsDefinition,
        rows: Vec<Vec<ShareableString>>,
        units: Vec<ShareableString>,
    ) -> Self {
        let mut table = Self {
            definition,
            units,
            rows,
            parameter: ShareableString::new(""),
            hash: [0u8; 32],
        };
        table.update_hash();
        table
    }

    /// Creates a new `TableWithUnitsFrozen` from a `TableWithUnitsEditable`.
    #[must_use]
    pub fn new_from_editable(editable_table: &TableWithUnitsEditable) -> Self {
        let mut table = Self {
            definition: editable_table.definition().clone(),
            rows: editable_table.rows().to_vec(),
            units: editable_table.units().to_vec(),
            parameter: editable_table.parameter().clone(),
            hash: [0u8; 32],
        };
        table.update_hash();
        table
    }

    /// Converts this `TableWithUnitsFrozen` into a `TableWithUnitsEditable`.
    #[must_use]
    pub fn thaw(&self) -> TableWithUnitsEditable {
        TableWithUnitsEditable::new(self)
    }

    /// Recomputes and stores the BLAKE3 hash of all column units and row cells.
    fn update_hash(&mut self) {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&[0x01]);
        hasher.update(b"TableWithUnits");

        // Hash column units so that changing a unit changes the hash.
        hasher.update(
            &u64::try_from(self.units.len())
                .unwrap_or(u64::MAX)
                .to_le_bytes(),
        );
        for unit in &self.units {
            hasher.update(&unit.current_blake3_hash());
        }

        // Hash row cell values.
        hasher.update(
            &u64::try_from(self.rows.len())
                .unwrap_or(u64::MAX)
                .to_le_bytes(),
        );
        for row in &self.rows {
            hasher.update(&u64::try_from(row.len()).unwrap_or(u64::MAX).to_le_bytes());
            for cell in row {
                hasher.update(&cell.current_blake3_hash());
            }
        }

        self.hash = *hasher.finalize().as_bytes();
    }

    /// Returns the value of a cell by row and column index.
    #[must_use]
    pub fn cell_by_index(&self, row: usize, column: usize) -> Option<&ShareableString> {
        self.rows.get(row)?.get(column)
    }

    /// Returns the value of a cell by row index and column name.
    pub fn cell_by_name<S: Into<ShareableString>>(
        &self,
        row: usize,
        column_name: S,
    ) -> Option<&ShareableString> {
        let column_index = self
            .definition
            .get_column_index_by_name(column_name.into())?;
        self.cell_by_index(row, column_index)
    }

    /// Returns the row at the specified index.
    #[must_use]
    pub fn row(&self, row: usize) -> Option<&Vec<ShareableString>> {
        self.rows.get(row)
    }

    /// Returns a reference to all rows in the table.
    #[must_use]
    pub fn rows(&self) -> &[Vec<ShareableString>] {
        &self.rows
    }

    /// Returns a slice of all column units, one per column in definition order.
    #[must_use]
    pub fn units(&self) -> &[ShareableString] {
        &self.units
    }

    /// Returns the unit for the column at the given index.
    #[must_use]
    pub fn unit_by_index(&self, column: usize) -> Option<&ShareableString> {
        self.units.get(column)
    }

    /// Returns the unit for the column with the given name.
    pub fn unit_by_name<S: Into<ShareableString>>(
        &self,
        column_name: S,
    ) -> Option<&ShareableString> {
        let column_index = self
            .definition
            .get_column_index_by_name(column_name.into())?;
        self.unit_by_index(column_index)
    }

    /// Returns a reference to the table definition.
    #[must_use]
    pub const fn definition(&self) -> &TableWithUnitsDefinition {
        &self.definition
    }

    /// Returns the number of rows in the table.
    #[must_use]
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Returns the number of columns in the table.
    #[must_use]
    pub fn column_count(&self) -> usize {
        self.definition.count()
    }

    /// Returns the pre-calculated BLAKE3 hash of the table.
    #[must_use]
    pub const fn hash(&self) -> [u8; 32] {
        self.hash
    }

    /// Returns a reference to the parameter value for the table.
    #[must_use]
    pub const fn parameter(&self) -> &ShareableString {
        &self.parameter
    }
}

impl PartialEq<&TableWithUnitsFrozen> for TableWithUnitsFrozen {
    fn eq(&self, other: &&TableWithUnitsFrozen) -> bool {
        self == *other
    }
}

impl PartialEq<TableWithUnitsFrozen> for &TableWithUnitsFrozen {
    fn eq(&self, other: &TableWithUnitsFrozen) -> bool {
        *self == other
    }
}

impl TreePrint for TableWithUnitsFrozen {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{} ({}) Table with units {} rows",
            prefix,
            Self::branch_char(last),
            label,
            self.definition.description(),
            self.rows.len(),
        )?;

        let child_prefix = Self::child_prefix(prefix, last);

        // Print column units.
        writeln!(f, "{}{}units", child_prefix, Self::branch_char(false))?;
        let units_prefix = Self::child_prefix(&child_prefix, false);
        let mut units_iter = self.units.iter().enumerate().peekable();
        while let Some((j, unit)) = units_iter.next() {
            let is_last = units_iter.peek().is_none();
            let key = self
                .definition
                .keys()
                .nth(j)
                .map_or("Unknown", |key| key.as_str());
            writeln!(
                f,
                "{}{}{} \"{}\"",
                units_prefix,
                Self::branch_char(is_last),
                key,
                unit,
            )?;
        }

        // Print row data.
        writeln!(f, "{}{}data", child_prefix, Self::branch_char(false))?;
        let data_prefix = Self::child_prefix(&child_prefix, false);
        let mut rows_iter = self.rows.iter().enumerate().peekable();

        while let Some((i, row)) = rows_iter.next() {
            let is_last_row = rows_iter.peek().is_none();
            writeln!(
                f,
                "{}{}Row {}",
                data_prefix,
                Self::branch_char(is_last_row),
                i
            )?;

            let row_prefix = Self::child_prefix(&data_prefix, is_last_row);
            let mut cells_iter = row.iter().enumerate().peekable();
            while let Some((j, cell)) = cells_iter.next() {
                let is_last_cell = cells_iter.peek().is_none();
                let key = self
                    .definition
                    .keys()
                    .nth(j)
                    .map_or("Unknown", |key| key.as_str());
                writeln!(
                    f,
                    "{}{}{} \"{}\"",
                    row_prefix,
                    Self::branch_char(is_last_cell),
                    key,
                    cell,
                )?;
            }
        }

        writeln!(
            f,
            "{}{}Parameter \"{}\"",
            child_prefix,
            Self::branch_char(true),
            self.parameter
        )
    }
}
