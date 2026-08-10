use datastore::prelude::*;
use expression_engine::prelude::*;
use std::collections::BTreeMap;

/// Builds a set of frozen items covering every `ItemFrozen` variant, keyed by field name.
fn sample_items() -> BTreeMap<String, ItemFrozen> {
    let entry_1 = MapEntryFrozen::new_from_items(
        vec![
            (
                store_key!("field1").into(),
                MapItemFrozen::String(StringFrozen::new_from_editable(
                    &StringEditable::new_with_value(
                        StringDefinition::new("Field 1"),
                        "entry1-value".into(),
                    ),
                )),
            ),
            (
                store_key!("field2").into(),
                MapItemFrozen::Number(NumberFrozen::new(NumberDefinition::new("Field 2"))),
            ),
        ]
        .into_iter()
        .collect(),
    );

    let mut entries = BTreeMap::new();
    entries.insert(store_key!("entry1").into(), entry_1);

    let map_frozen = MapFrozen::new_from_items("A map", entries).expect("valid map schema");

    let table_frozen = TableFrozen::new_from_rows(
        TableDefinition::new(
            "A table",
            vec![(store_key!("col1"), NumberDefinition::new("Column 1"))],
        ),
        vec![vec!["1".into()]],
    );

    vec![
        (
            "string_field".to_string(),
            ItemFrozen::String(StringFrozen::new(StringDefinition::new("A string"))),
        ),
        (
            "number_field".to_string(),
            ItemFrozen::Number(NumberFrozen::new(NumberDefinition::new("A number"))),
        ),
        (
            "file_field".to_string(),
            ItemFrozen::File(FileFrozen::new(FileDefinition::new("A file", "txt", false))),
        ),
        (
            "choice_field".to_string(),
            ItemFrozen::Choice(ChoiceFrozen::new(ChoiceDefinition::new(
                "A choice",
                vec![ChoiceItemDefinition::new(store_key!("a"), "A")],
            ))),
        ),
        ("table_field".to_string(), ItemFrozen::Table(table_frozen)),
        ("map_field".to_string(), ItemFrozen::Map(map_frozen)),
    ]
    .into_iter()
    .collect()
}

#[test]
fn test_global_object_input_data() {
    // Why: Test that every basic item kind is preserved when converting a `GlobalObjectFrozen`
    // into its input representation.
    let items: BTreeMap<GlobalKey, ItemFrozen> = sample_items()
        .into_iter()
        .map(|(k, v)| (GlobalKey::new(format!("g_{k}").into()).unwrap(), v))
        .collect();
    let frozen = GlobalObjectFrozen::new_from_items("Test object", items);

    let input = GlobalObjectInputData::new(&frozen);
    let data = input.data();

    // Basic items should keep their key and be exposed as `Basic` entries.
    match data.get("g_string_field").unwrap() {
        ObjectItemInputData::Basic(basic) => assert_eq!(basic.data().as_ref(), ""),
        ObjectItemInputData::BasicWithUnits(_)
        | ObjectItemInputData::Table(_)
        | ObjectItemInputData::TableWithUnits(_) => {
            panic!("expected basic data")
        }
    }

    // Table items should keep their key and be exposed as `Table` entries.
    match data.get("g_table_field").unwrap() {
        ObjectItemInputData::Table(table) => {
            assert_eq!(table.data().len(), 1);
            assert_eq!(table.data()[0][0].as_ref(), "1");
        }
        ObjectItemInputData::Basic(_)
        | ObjectItemInputData::BasicWithUnits(_)
        | ObjectItemInputData::TableWithUnits(_) => {
            panic!("expected table data")
        }
    }

    // Map items should be flattened into `key[entry][field]` paths.
    match data.get("g_map_field[entry1][field1]").unwrap() {
        ObjectItemInputData::Basic(basic) => {
            assert_eq!(basic.data().as_ref(), "entry1-value");
        }
        ObjectItemInputData::BasicWithUnits(_)
        | ObjectItemInputData::Table(_)
        | ObjectItemInputData::TableWithUnits(_) => {
            panic!("expected basic data")
        }
    }
    match data.get("g_map_field[entry1][field2]").unwrap() {
        ObjectItemInputData::Basic(basic) => assert_eq!(basic.data().as_ref(), ""),
        ObjectItemInputData::BasicWithUnits(_)
        | ObjectItemInputData::Table(_)
        | ObjectItemInputData::TableWithUnits(_) => {
            panic!("expected basic data")
        }
    }

    // A map's flattened field should not remain accessible under its original path.
    assert!(data.get("g_map_field").is_none());

    // Every other item kind should be present under its own key.
    assert!(data.contains_key("g_number_field"));
    assert!(data.contains_key("g_file_field"));
    assert!(data.contains_key("g_choice_field"));

    // Six declared items minus the one map (flattened into two) plus its two flattened entries.
    assert_eq!(data.len(), 7);
}

#[test]
fn test_parameter_object_input_data() {
    // Why: Test that `ParameterObjectInputData` extracts basic and table items the same
    // way as the global variant.
    let items: BTreeMap<ParameterKey, ItemFrozen> = sample_items()
        .into_iter()
        .map(|(k, v)| (ParameterKey::new(format!("p_{k}").into()).unwrap(), v))
        .collect();
    let frozen = ParameterObjectFrozen::new_from_items("Test object", items);

    let input = ParameterObjectInputData::new(&frozen);
    let data = input.data();

    match data.get("p_string_field").unwrap() {
        ObjectItemInputData::Basic(basic) => assert_eq!(basic.data().as_ref(), ""),
        ObjectItemInputData::BasicWithUnits(_)
        | ObjectItemInputData::Table(_)
        | ObjectItemInputData::TableWithUnits(_) => {
            panic!("expected table data")
        }
    }
    match data.get("p_table_field").unwrap() {
        ObjectItemInputData::Table(table) => assert_eq!(table.data().len(), 1),
        ObjectItemInputData::Basic(_)
        | ObjectItemInputData::BasicWithUnits(_)
        | ObjectItemInputData::TableWithUnits(_) => {
            panic!("expected table data")
        }
    }
    assert!(data.get("p_map_field[entry1][field1]").is_some());
}

#[test]
fn test_variable_object_input_data() {
    // Why: Test that `VariableObjectInputData` extracts basic and table items the same
    // way as the global variant.
    let items: BTreeMap<VariableKey, ItemFrozen> = sample_items()
        .into_iter()
        .map(|(k, v)| (VariableKey::new(format!("v_{k}").into()).unwrap(), v))
        .collect();
    let frozen = VariableObjectFrozen::new_from_items("Test object", items);

    let input = VariableObjectInputData::new(&frozen);
    let data = input.data();

    match data.get("v_string_field").unwrap() {
        ObjectItemInputData::Basic(basic) => assert_eq!(basic.data().as_ref(), ""),
        ObjectItemInputData::BasicWithUnits(_)
        | ObjectItemInputData::Table(_)
        | ObjectItemInputData::TableWithUnits(_) => {
            panic!("expected basic data")
        }
    }
    match data.get("v_table_field").unwrap() {
        ObjectItemInputData::Table(table) => assert_eq!(table.data().len(), 1),
        ObjectItemInputData::Basic(_)
        | ObjectItemInputData::BasicWithUnits(_)
        | ObjectItemInputData::TableWithUnits(_) => {
            panic!("expected table data")
        }
    }
    assert!(data.get("v_map_field[entry1][field1]").is_some());
}

#[test]
fn test_global_object_input_data_empty() {
    // Why: An object with no items should produce an empty input data map.
    let frozen = GlobalObjectFrozen::new(GlobalObjectDefinition::builder("Empty object").finish());
    let input = GlobalObjectInputData::new(&frozen);

    assert!(input.data().is_empty());
}
