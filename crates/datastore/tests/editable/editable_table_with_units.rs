use datastore::prelude::*;
use units::UnitId;

#[test]
fn test_editable_table_with_units_round_trip() {
    let frozen = TableWithUnitsFrozen::new(TableWithUnitsDefinition::new(
        "Measurements",
        vec![
            (
                store_key!("length"),
                NumberWithUnitsDefinition::new_with_default("Length", "1.0", UnitId::Length_Meter),
            ),
            (
                store_key!("duration"),
                NumberWithUnitsDefinition::new_with_default("Duration", "2.0", UnitId::Time_Second),
            ),
        ],
    ));
    let mut editable = frozen.thaw();

    assert_eq!(editable.row_count(), 0);
    assert_eq!(editable.column_count(), 2);

    editable.add_row(0);
    assert_eq!(editable.row_count(), 1);
    assert_eq!(
        editable
            .cell_by_name(0, "length")
            .expect("length cell")
            .as_ref(),
        "1.0"
    );
    assert_eq!(
        editable.unit_by_index(0).expect("length unit").as_ref(),
        UnitId::Length_Meter.string_id().as_str()
    );

    editable.set_cell(0, "length", "5.5").expect("set cell");
    editable
        .set_unit_by_name("length", UnitId::Length_Centimeter.string_id())
        .expect("set length unit");
    editable.set_parameter("measurements");
    assert_eq!(editable.parameter().as_ref(), "measurements");
    assert_eq!(
        editable
            .row(0)
            .expect("first row")
            .first()
            .expect("first cell")
            .as_ref(),
        "5.5"
    );
    assert_eq!(
        editable
            .unit_by_name("length")
            .expect("length unit")
            .as_ref(),
        UnitId::Length_Centimeter.string_id().as_str()
    );

    let frozen = editable.freeze();
    assert_eq!(frozen.row_count(), 1);
    assert_eq!(frozen.parameter().as_ref(), "measurements");
    assert_eq!(
        frozen.unit_by_index(0).expect("length unit").as_ref(),
        UnitId::Length_Centimeter.string_id().as_str()
    );
    assert_ne!(
        frozen.hash(),
        TableWithUnitsFrozen::new(TableWithUnitsDefinition::new(
            "Measurements",
            vec![
                (
                    store_key!("length"),
                    NumberWithUnitsDefinition::new_with_default(
                        "Length",
                        "1.0",
                        UnitId::Length_Meter
                    ),
                ),
                (
                    store_key!("duration"),
                    NumberWithUnitsDefinition::new_with_default(
                        "Duration",
                        "2.0",
                        UnitId::Time_Second
                    ),
                ),
            ],
        ))
        .hash()
    );
}

#[test]
fn test_editable_table_with_units_cell_mutation_errors() {
    let mut editable = TableWithUnitsFrozen::new(TableWithUnitsDefinition::new(
        "Measurements",
        vec![
            (
                store_key!("length"),
                NumberWithUnitsDefinition::new_with_default("Length", "1.0", UnitId::Length_Meter),
            ),
            (
                store_key!("duration"),
                NumberWithUnitsDefinition::new_with_default("Duration", "2.0", UnitId::Time_Second),
            ),
        ],
    ))
    .thaw();
    editable.add_row(0);

    editable.set_cell(0, "duration", "4.0").expect("set cell");
    editable
        .set_unit_by_index(1, UnitId::Time_Second.string_id())
        .expect("set unit");
    assert_eq!(
        editable
            .cell_by_name(0, "duration")
            .expect("duration cell")
            .as_ref(),
        "4.0"
    );
    assert_eq!(
        editable
            .unit_by_name("duration")
            .expect("duration unit")
            .as_ref(),
        UnitId::Time_Second.string_id().as_str()
    );

    assert_eq!(
        editable.set_cell(0, "unknown", "5"),
        Err(StoreError::KeyNotFound)
    );
    assert_eq!(
        editable.set_cell(1, "duration", "5"),
        Err(StoreError::IndexNotFound)
    );
    assert_eq!(
        editable.set_unit_by_index(8, UnitId::Time_Second.string_id()),
        Err(StoreError::IndexNotFound)
    );
    assert_eq!(
        editable.set_unit_by_name("unknown", UnitId::Time_Second.string_id()),
        Err(StoreError::KeyNotFound)
    );
}

#[test]
fn test_editable_table_with_units_row_removal() {
    let mut editable = TableWithUnitsFrozen::new(TableWithUnitsDefinition::new(
        "Measurements",
        vec![
            (
                store_key!("length"),
                NumberWithUnitsDefinition::new_with_default("Length", "1.0", UnitId::Length_Meter),
            ),
            (
                store_key!("duration"),
                NumberWithUnitsDefinition::new_with_default("Duration", "2.0", UnitId::Time_Second),
            ),
        ],
    ))
    .thaw();
    editable.add_row(0);
    editable.add_row(1);
    editable.remove_row(0);

    assert_eq!(editable.row_count(), 1);
    editable.remove_row(8);
    assert_eq!(editable.row_count(), 0);
    editable.remove_row(0);
    assert_eq!(editable.row_count(), 0);
}
