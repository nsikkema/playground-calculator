use datastore::prelude::*;

#[test]
fn test_table_with_units_definition() {
    let table_def = TableWithUnitsDefinition::new(
        "Measurements",
        vec![
            (
                store_key!("length"),
                NumberWithUnitsDefinition::new_with_default(
                    "Length",
                    "1.0",
                    units::UnitId::Length_Meter,
                ),
            ),
            (
                store_key!("duration"),
                NumberWithUnitsDefinition::new("Duration", units::UnitId::Time_Second),
            ),
        ],
    );

    assert_eq!(table_def.description(), "Measurements");
    assert_eq!(table_def.count(), 2);
    assert!(table_def.contains_key(store_key!("length")));
    assert!(table_def.contains_key_str("duration"));
    assert_eq!(
        table_def
            .get_by_index(0)
            .map(NumberWithUnitsDefinition::description),
        Some("Length".into())
    );
    assert_eq!(
        table_def
            .get_str("duration")
            .map(NumberWithUnitsDefinition::preferred_units),
        Some(units::UnitId::Time_Second)
    );
    assert_eq!(
        table_def.get_column_index_by_name(store_key!("duration")),
        Some(1)
    );
}
