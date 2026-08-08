use datastore::prelude::*;
use expression_engine::prelude::*;
use std::collections::BTreeMap;

/// Builds a `GlobalObjectInputData` with a single basic item and returns its
/// `BasicInputData`, extracted via the crate's public conversion path (`new` on
/// `BasicInputData` is crate-private, so this is how tests must construct instances).
fn basic_data_for(item: ItemFrozen) -> BasicInputData {
    let mut items = BTreeMap::new();
    items.insert(GlobalKey::new("g_field".into()).unwrap(), item);
    let frozen = GlobalObjectFrozen::new_from_items("Test object", items);
    let input = GlobalObjectInputData::new(&frozen);

    match input.data().get("g_field").unwrap() {
        ObjectItemInputData::Basic(basic) => basic.clone(),
        ObjectItemInputData::BasicWithUnits(_) | ObjectItemInputData::Table(_) => {
            panic!("expected basic data")
        }
    }
}

#[test]
fn test_basic_input_data_string() {
    // Why: Test basic input data extraction from a string item.
    let data = basic_data_for(ItemFrozen::String(StringFrozen::new_from_editable(
        &StringEditable::new_with_value(
            StringDefinition::new("A string parameter"),
            "hello".into(),
        ),
    )));

    assert_eq!(
        data.definition(),
        &BasicDefinition::String(StringDefinition::new("A string parameter"))
    );
    assert_eq!(data.data().as_ref(), "hello");
}

#[test]
fn test_basic_input_data_number() {
    // Why: Test basic input data extraction from a number item, using its default value.
    let data = basic_data_for(ItemFrozen::Number(NumberFrozen::new(
        NumberDefinition::new_with_default("A number parameter", "42"),
    )));

    assert_eq!(
        data.definition(),
        &BasicDefinition::Number(NumberDefinition::new_with_default(
            "A number parameter",
            "42"
        ))
    );
    assert_eq!(data.data().as_ref(), "42");
}

#[test]
fn test_basic_input_data_file() {
    // Why: Test basic input data extraction from a file item, using its default value.
    let data = basic_data_for(ItemFrozen::File(FileFrozen::new(
        FileDefinition::new_with_default("A file parameter", "txt", false, "test.txt"),
    )));

    assert_eq!(
        data.definition(),
        &BasicDefinition::File(FileDefinition::new_with_default(
            "A file parameter",
            "txt",
            false,
            "test.txt"
        ))
    );
    assert_eq!(data.data().as_ref(), "test.txt");
}

#[test]
fn test_basic_input_data_choice() {
    // Why: Test basic input data extraction from a choice item, using its default value.
    let choices = vec![
        ChoiceItemDefinition::new(store_key!("a"), "A"),
        ChoiceItemDefinition::new(store_key!("b"), "B"),
    ];
    let data = basic_data_for(ItemFrozen::Choice(ChoiceFrozen::new(
        ChoiceDefinition::new_with_default("A choice parameter", choices.clone(), "a"),
    )));

    assert_eq!(
        data.definition(),
        &BasicDefinition::Choice(ChoiceDefinition::new_with_default(
            "A choice parameter",
            choices,
            "a"
        ))
    );
    assert_eq!(data.data().as_ref(), "a");
}

#[test]
fn test_basic_input_data_equality() {
    // Why: Test that two basic input data items with the same content are considered
    // equal and differ when their values diverge.
    let data_1 = basic_data_for(ItemFrozen::String(StringFrozen::new_from_editable(
        &StringEditable::new_with_value(StringDefinition::new("A string"), "hello".into()),
    )));
    let data_2 = basic_data_for(ItemFrozen::String(StringFrozen::new_from_editable(
        &StringEditable::new_with_value(StringDefinition::new("A string"), "hello".into()),
    )));
    let data_3 = basic_data_for(ItemFrozen::String(StringFrozen::new_from_editable(
        &StringEditable::new_with_value(StringDefinition::new("A string"), "world".into()),
    )));

    assert_eq!(data_1, data_2);
    assert_ne!(data_1, data_3);
}

#[test]
fn test_basic_input_data_launder() {
    // Why: Laundering should replace strings with interned instances from the store while
    // preserving the definition and value contents.
    let store = SharedStringStore::new();
    let data = basic_data_for(ItemFrozen::String(StringFrozen::new_from_editable(
        &StringEditable::new_with_value(
            StringDefinition::new("A string parameter"),
            "hello".into(),
        ),
    )));

    let laundered = data.launder(&store);

    assert_eq!(laundered.data().as_ref(), "hello");
    assert_eq!(laundered.definition(), data.definition());
    assert!(store.contains("hello"));
}
