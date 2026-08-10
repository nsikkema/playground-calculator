use crate::definition::TableWithUnitsDefinition;
use crate::frozen::TableWithUnitsFrozen;
use crate::traits::TreePrint;
use errors::StoreError;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;

/// Represents a table of number-with-units data in the editable data.
///
/// Cell values are stored as plain strings (`rows`). Units are stored once per
/// column (`units`), in definition column order, rather than per cell.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableWithUnitsEditable {
    /// Definition metadata for this table value.
    definition: TableWithUnitsDefinition,
    /// Parameter key associated with this table instance.
    parameter: ShareableString,
    /// Row data; each inner `Vec` holds one plain-string value per column.
    rows: Vec<Vec<ShareableString>>,
    /// One unit string per column, in definition column order.
    units: Vec<ShareableString>,
}

impl TableWithUnitsEditable {
    /// Creates a new `TableWithUnitsEditable` from a `TableWithUnitsFrozen`.
    #[must_use]
    pub fn new(frozen_table: &TableWithUnitsFrozen) -> Self {
        Self {
            definition: frozen_table.definition().clone(),
            parameter: frozen_table.parameter().clone(),
            rows: frozen_table.rows().to_vec(),
            units: frozen_table.units().to_vec(),
        }
    }

    /// Converts this `TableWithUnitsEditable` into a `TableWithUnitsFrozen`.
    #[must_use]
    pub fn freeze(&self) -> TableWithUnitsFrozen {
        TableWithUnitsFrozen::new_from_editable(self)
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

    /// Sets the value of a cell (value only; use [`set_column_unit`] to change units).
    ///
    /// # Errors
    ///
    /// Returns `StoreError::KeyNotFound` if `column_name` does not match a column
    /// in the table's definition, or `StoreError::IndexNotFound` if `row` or the
    /// resolved column index is not present in the row data.
    ///
    /// [`set_column_unit`]: Self::set_column_unit
    pub fn set_cell<S: Into<ShareableString>, V: Into<ShareableString>>(
        &mut self,
        row: usize,
        column_name: S,
        value: V,
    ) -> Result<(), StoreError> {
        let col_name = column_name.into();
        let Some(column_index) = self.definition.get_column_index_by_name(col_name) else {
            return Err(StoreError::KeyNotFound);
        };

        if let Some(row_data) = self.rows.get_mut(row) {
            if let Some(column_data) = row_data.get_mut(column_index) {
                *column_data = value.into();
                Ok(())
            } else {
                Err(StoreError::IndexNotFound)
            }
        } else {
            Err(StoreError::IndexNotFound)
        }
    }

    /// Sets the unit for a column by index.
    ///
    /// # Errors
    ///
    /// Returns `StoreError::IndexNotFound` if `column` does not identify a unit.
    pub fn set_unit_by_index<U: Into<ShareableString>>(
        &mut self,
        column: usize,
        unit: U,
    ) -> Result<(), StoreError> {
        let Some(column_unit) = self.units.get_mut(column) else {
            return Err(StoreError::IndexNotFound);
        };
        *column_unit = unit.into();
        Ok(())
    }

    /// Sets the unit for a column by name.
    ///
    /// # Errors
    ///
    /// Returns `StoreError::KeyNotFound` if `column_name` does not match a column
    /// in the table's definition.
    pub fn set_unit_by_name<S: Into<ShareableString>, U: Into<ShareableString>>(
        &mut self,
        column_name: S,
        unit: U,
    ) -> Result<(), StoreError> {
        let col_name = column_name.into();
        let column_index = self
            .definition
            .get_column_index_by_name(col_name)
            .ok_or(StoreError::KeyNotFound)?;
        self.set_unit_by_index(column_index, unit)
    }

    /// Sets the unit for a column by name.
    ///
    /// # Errors
    ///
    /// Returns `StoreError::KeyNotFound` if `column_name` does not match a column
    /// in the table's definition.
    pub fn set_column_unit<S: Into<ShareableString>, U: Into<ShareableString>>(
        &mut self,
        column_name: S,
        unit: U,
    ) -> Result<(), StoreError> {
        self.set_unit_by_name(column_name, unit)
    }

    /// Adds a new row initialised from the columns' default values.
    pub fn add_row(&mut self, row: usize) {
        let full_row: Vec<ShareableString> = self
            .definition
            .iter()
            .map(|(_, col_def)| col_def.default_value())
            .collect();
        if row < self.rows.len() {
            self.rows.insert(row, full_row);
        } else {
            self.rows.push(full_row);
        }
    }

    /// Removes a row.
    pub fn remove_row(&mut self, row: usize) {
        if self.rows.is_empty() {
            return;
        }

        if row < self.rows.len() {
            self.rows.remove(row);
        } else {
            self.rows.pop();
        }
    }

    /// Sets the parameter value for the table.
    pub fn set_parameter<S: Into<ShareableString>>(&mut self, parameter: S) {
        self.parameter = parameter.into();
    }

    /// Returns a reference to the parameter value for the table.
    #[must_use]
    pub const fn parameter(&self) -> &ShareableString {
        &self.parameter
    }
}

impl PartialEq<&TableWithUnitsEditable> for TableWithUnitsEditable {
    fn eq(&self, other: &&TableWithUnitsEditable) -> bool {
        self == *other
    }
}

impl PartialEq<TableWithUnitsEditable> for &TableWithUnitsEditable {
    fn eq(&self, other: &TableWithUnitsEditable) -> bool {
        *self == other
    }
}

impl TreePrint for TableWithUnitsEditable {
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
