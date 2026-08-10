use datastore::prelude::*;
use units::UnitId;

#[test]
fn test_table_with_units_frozen() {
    let table = TableWithUnitsFrozen::new(TableWithUnitsDefinition::new(
        "Measurements",
        vec![
            (
                store_key!("length"),
                NumberWithUnitsDefinition::new_with_default("Length", "1.0", UnitId::Length_Meter),
            ),
            (
                store_key!("duration"),
                NumberWithUnitsDefinition::new("Duration", UnitId::Time_Second),
            ),
        ],
    ));

    assert_eq!(table.definition().description().as_ref(), "Measurements");
    assert_eq!(table.row_count(), 0);
    assert_eq!(table.column_count(), 2);
    assert_ne!(table.hash(), [0u8; 32]);
    assert_eq!(table.parameter().as_ref(), "");
    assert_eq!(
        table.unit_by_name("length").expect("length unit").as_ref(),
        UnitId::Length_Meter.string_id().as_str()
    );
    assert_eq!(
        table.unit_by_index(1).expect("duration unit").as_ref(),
        UnitId::Time_Second.string_id().as_str()
    );
}

#[test]
fn test_table_with_units_frozen_from_rows_hashes_values_and_units() {
    let definition = TableWithUnitsDefinition::new(
        "Measurements",
        vec![
            (
                store_key!("length"),
                NumberWithUnitsDefinition::new_with_default("Length", "1.0", UnitId::Length_Meter),
            ),
            (
                store_key!("duration"),
                NumberWithUnitsDefinition::new("Duration", UnitId::Time_Second),
            ),
        ],
    );
    let table = TableWithUnitsFrozen::new_from_rows(
        definition.clone(),
        vec![vec!["3.0".into(), "4.0".into()]],
        vec![
            UnitId::Length_Meter.string_id().into(),
            UnitId::Time_Second.string_id().into(),
        ],
    );
    let different_units = TableWithUnitsFrozen::new_from_rows(
        definition,
        vec![vec!["3.0".into(), "4.0".into()]],
        vec![
            UnitId::Length_Centimeter.string_id().into(),
            UnitId::Time_Second.string_id().into(),
        ],
    );

    assert_eq!(table.row_count(), 1);
    assert_eq!(table.rows().len(), 1);
    assert_eq!(
        table
            .cell_by_name(0, "length")
            .expect("length cell")
            .as_ref(),
        "3.0"
    );
    assert_eq!(
        table
            .unit_by_name("duration")
            .expect("duration unit")
            .as_ref(),
        UnitId::Time_Second.string_id().as_str()
    );
    assert_ne!(table.hash(), different_units.hash());
}

#[test]
fn test_table_with_units_frozen_equality() {
    let table_one = TableWithUnitsFrozen::new(TableWithUnitsDefinition::new(
        "Measurements",
        vec![
            (
                store_key!("length"),
                NumberWithUnitsDefinition::new_with_default("Length", "1.0", UnitId::Length_Meter),
            ),
            (
                store_key!("duration"),
                NumberWithUnitsDefinition::new("Duration", UnitId::Time_Second),
            ),
        ],
    ));
    let table_two = TableWithUnitsFrozen::new(TableWithUnitsDefinition::new(
        "Measurements",
        vec![
            (
                store_key!("length"),
                NumberWithUnitsDefinition::new_with_default("Length", "1.0", UnitId::Length_Meter),
            ),
            (
                store_key!("duration"),
                NumberWithUnitsDefinition::new("Duration", UnitId::Time_Second),
            ),
        ],
    ));
    let different = TableWithUnitsFrozen::new(TableWithUnitsDefinition::new(
        "Other measurements",
        vec![(
            store_key!("length"),
            NumberWithUnitsDefinition::new("Length", UnitId::Length_Meter),
        )],
    ));

    assert_eq!(table_one, table_two);
    assert_eq!(&table_one, table_two);
    assert_ne!(table_one, different);
    assert_ne!(table_one, &different);
}
