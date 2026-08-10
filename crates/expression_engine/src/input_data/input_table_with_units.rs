use datastore::definition::TableWithUnitsDefinition;
use shareable_string::ShareableString;

/// Represents input data for a table, including its definition and the associated data.
#[derive(Debug, Clone, PartialEq)]
pub struct TableWithUnitsInputData {
    /// The definition that describes the columns and constraints of this table.
    definition: TableWithUnitsDefinition,
    /// The name of the parameter variable that holds the row index during cell evaluation,
    /// or an empty string if no such parameter is used.
    parameter: ShareableString,
    /// The names of the units associated with each column in the table.
    units: Vec<ShareableString>,
    /// The raw cell data, one inner `Vec<ShareableString>` per row.
    data: Vec<Vec<ShareableString>>,
}

impl TableWithUnitsInputData {
    /// Creates a new `TableWithUnitsInputData` with the given definition, parameter name, and raw data.
    pub(crate) const fn new(
        definition: TableWithUnitsDefinition,
        parameter: ShareableString,
        units: Vec<ShareableString>,
        data: Vec<Vec<ShareableString>>,
    ) -> Self {
        Self {
            definition,
            parameter,
            units,
            data,
        }
    }

    /// Returns a reference to the definition of the table input data.
    #[must_use]
    pub const fn definition(&self) -> &TableWithUnitsDefinition {
        &self.definition
    }

    /// Returns a reference to the units of the table input data.
    #[must_use]
    pub fn units(&self) -> &[ShareableString] {
        &self.units
    }

    /// Returns a reference to the data of the table input data.
    #[must_use]
    pub fn data(&self) -> &[Vec<ShareableString>] {
        &self.data
    }

    /// Returns a reference to the parameter name of the table input data.
    ///
    /// When non-empty, this name is bound to the current row index while each
    /// row's cell expressions are evaluated.
    #[must_use]
    pub const fn parameter(&self) -> &ShareableString {
        &self.parameter
    }

    /// Returns a new `TableWithUnitsInputData` with strings laundered through the provided store.
    #[must_use]
    pub fn launder(&self, store: &shareable_string::SharedStringStore) -> Self {
        let laundered_definition = self.definition.launder(store);
        let laundered_units = self.units.iter().map(|unit| store.launder(unit)).collect();
        let laundered_data = self
            .data
            .iter()
            .map(|row| row.iter().map(|value| store.launder(value)).collect())
            .collect();

        Self {
            definition: laundered_definition,
            parameter: store.launder(&self.parameter),
            units: laundered_units,
            data: laundered_data,
        }
    }
}
