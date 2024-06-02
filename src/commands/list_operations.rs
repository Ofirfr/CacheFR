use crate::{
    commands::get,
    commands_proto::{self, AtomicFrValue, FrKey, FrValue},
    value_structs::{CacheFRMap, StoredAtomicValue, StoredFrValueWithExpiry},
};

pub async fn list_append(
    main_map: &CacheFRMap,
    key: FrKey,
    value: AtomicFrValue,
) -> Option<StoredAtomicValue> {
    let maybe_old_value = get::get_from_map(&main_map, key.clone()).await;
    maybe_old_value.map(|mut old_value| {
        let old_value_as_list = old_value.as_mut_list().expect("Stored value is not a list");
        let value_to_insert = StoredAtomicValue::from_atomic_fr_value(value);
        old_value_as_list.push(value_to_insert.clone());
        value_to_insert
    })
}

pub async fn list_remove(
    main_map: &CacheFRMap,
    key: FrKey,
    index: i32,
) -> Option<StoredAtomicValue> {
    let maybe_old_value = get::get_from_map(&main_map, key.clone()).await;
    let old_value = &mut maybe_old_value.expect("Key does not exist");
    let old_value_as_list = old_value.as_mut_list();
    if let Ok(old_value_as_list) = old_value_as_list {
        let removed_val = old_value_as_list.remove(index as usize);
        Some(removed_val)
    } else {
        None
    }
}
