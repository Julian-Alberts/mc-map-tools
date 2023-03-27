use std::collections::HashMap;

use crate::{data::{item::*, FieldError}, nbt::Tag};

mod_try_from_tag!({
    Item: [
        "Count" => set_count test(10_i8 => count = 10),
        "id" => set_id test("test_id".to_string() => id = "test_id".to_string()),
        "tag" => set_tag test(HashMap::new() => tag = Some(HashMap::new())),
    ],
    ItemWithSlot: parse_item_with_slot ? [ Item, ],
});

fn parse_item_with_slot(
    builder: &mut ItemWithSlotBuilder,
    mut nbt_data: HashMap<String, Tag>,
) -> Result<(), ItemWithSlotError> {
    add_data_to_builder!(builder, nbt_data => [
        "Slot": set_slot,
    ]);
    builder.set_item(nbt_data.try_into().map_err(|e| FieldError::new("<internal> item",e))?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![
        ("Slot", Tag::Byte(0)),
        ("Count", Tag::Byte(10)),
        ("id", Tag::String("test_id".to_string())),
        ("tag", Tag::Compound(HashMap::new())),
    ] => Ok(ItemWithSlot {
        slot: 0,
        item: Item {
            count: 10,
            id: "test_id".to_string(),
            tag: Some(HashMap::new()),
        },
    }); "Success")]
    #[test_case(vec![
        ("Count", Tag::Byte(10)),
        ("id", Tag::String("test_id".to_string())),
        ("tag", Tag::Compound(HashMap::new())),
    ] => Err(ItemWithSlotError::ItemWithSlotBuilder(ItemWithSlotBuilderError::UnsetSlot)); "Missing slot")]
    #[test_case(vec![
        ("Slot", Tag::Byte(0)),
        ("id", Tag::String("test_id".to_string())),
        ("tag", Tag::Compound(HashMap::new())),
    ] => Err(ItemWithSlotError::ItemField(FieldError::new("<internal> item", ItemError::ItemBuilder(ItemBuilderError::UnsetCount)))); "Missing count")]
    #[test_case(vec![
        ("Slot", Tag::Byte(0)),
        ("Count", Tag::Byte(10)),
        ("tag", Tag::Compound(HashMap::new())),
    ] => Err(ItemWithSlotError::ItemField(FieldError::new("<internal> item", ItemError::ItemBuilder(ItemBuilderError::UnsetId)))); "Missing id")]
    #[test_case(vec![
        ("Slot", Tag::Byte(0)),
        ("Count", Tag::Byte(10)),
        ("id", Tag::String("test_id".to_string())),
    ] => Ok(ItemWithSlot {
        slot: 0,
        item: Item {
            count: 10,
            id: "test_id".to_string(),
            tag: None,
        },
    }); "Success without tag")]
    fn test_parse_item_with_slot(nbt_data: Vec<(&str, Tag)>) -> Result<ItemWithSlot, ItemWithSlotError> {
        let nbt_data = Tag::Compound(HashMap::from_iter(nbt_data.into_iter().map(|(k, v)| (k.to_string(), v))));
        nbt_data.try_into()
    }

}