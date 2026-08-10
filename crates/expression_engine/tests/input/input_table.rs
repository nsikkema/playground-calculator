use datastore::prelude::*;
use expression_engine::prelude::*;
use std::collections::BTreeMap;

/// Builds a `GlobalObjectInputData` with a single table item and returns its
/// `TableInputData`, extracted via the crate's public conversion path (`new` on
/// `TableInputData` is crate-private, so this is how tests must construct instances).
fn table_data_for(table: TableFrozen) -> TableInputData {
    let mut items = BTreeMap::new();
    items.insert(
        GlobalKey::new("g_field".into()).unwrap(),
        ItemFrozen::Table(table),
    );
    let frozen = GlobalObjectFrozen::new_from_items("Test object", items);
    let input = GlobalObjectInputData::new(&frozen);

    match input.data().get("g_field").unwrap() {
        ObjectItemInputData::Table(table) => table.clone(),
        ObjectItemInputData::Basic(_)
        | ObjectItemInputData::BasicWithUnits(_)
        | ObjectItemInputData::TableWithUnits(_) => {
            panic!("expected table data")
        }
    }
}

fn sample_table_frozen() -> TableFrozen {
    let definition = TableDefinition::new(
        "A table",
        vec![
            (store_key!("col1"), NumberDefinition::new("Column 1")),
            (store_key!("col2"), NumberDefinition::new("Column 2")),
        ],
    );

    let row_1: Vec<ShareableString> = vec!["1".into(), "2".into()];
    let row_2: Vec<ShareableString> = vec!["3".into(), "4".into()];

    TableFrozen::new_from_rows(definition, vec![row_1, row_2])
}

#[test]
fn test_table_input_data() {
    // Why: Test table input data extraction and accessors.
    let data = table_data_for(sample_table_frozen());

    assert_eq!(data.definition().description().as_ref(), "A table");
    assert_eq!(data.definition().count(), 2);
    assert_eq!(data.data().len(), 2);
    assert_eq!(data.data()[0][0].as_ref(), "1");
    assert_eq!(data.data()[1][1].as_ref(), "4");
}

#[test]
fn test_table_input_data_empty() {
    // Why: Test table input data extraction with no rows.
    let definition = TableDefinition::new(
        "An empty table",
        vec![(store_key!("col1"), NumberDefinition::new("Column 1"))],
    );
    let data = table_data_for(TableFrozen::new(definition));

    assert_eq!(data.data().len(), 0);
}

#[test]
fn test_table_input_data_equality() {
    // Why: Test that two table input data items with the same content are considered
    // equal and differ when their definitions diverge.
    let data_1 = table_data_for(sample_table_frozen());
    let data_2 = table_data_for(sample_table_frozen());
    assert_eq!(data_1, data_2);

    let different_definition = TableDefinition::new(
        "A different table",
        vec![(store_key!("col1"), NumberDefinition::new("Column 1"))],
    );
    let data_3 = table_data_for(TableFrozen::new(different_definition));
    assert_ne!(data_1, data_3);
}

#[test]
fn test_table_input_data_launder() {
    // Why: Laundering should replace strings in both the definition and row values with
    // interned instances from the store while preserving content.
    let store = SharedStringStore::new();
    let data = table_data_for(sample_table_frozen());

    let laundered = data.launder(&store);

    assert_eq!(laundered.definition().description().as_ref(), "A table");
    assert_eq!(laundered.data().len(), 2);
    assert_eq!(laundered.data()[0][0].as_ref(), "1");
    assert!(store.contains("1"));
    assert!(store.contains("4"));
}
