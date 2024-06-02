use crate::{
    commands_proto::{AtomicFrValue, FrKey, FrValue},
    value_structs::{CacheFRMap, StoredAtomicValue, StoredFrValueWithExpiry},
};

use super::get;

pub async fn set_add(
    main_map: &CacheFRMap,
    key: FrKey,
    value: AtomicFrValue,
) -> Option<StoredAtomicValue> {
    let maybe_old_value = get::get_from_map(&main_map, key.clone()).await;
    maybe_old_value.map(|mut old_value| {
        let old_value_as_set = old_value.as_mut_set().expect("Stored value is not a set");
        let value_to_insert = StoredAtomicValue::from_atomic_fr_value(value);
        old_value_as_set.insert(value_to_insert.clone());
        value_to_insert
    })
}

pub async fn set_remove(
    main_map: &CacheFRMap,
    key: FrKey,
    value: AtomicFrValue,
) -> Option<AtomicFrValue> {
    let maybe_old_value = get::get_from_map(&main_map, key.clone()).await;
    maybe_old_value.map(|mut old_value| {
        let old_value_as_set = old_value.as_mut_set().expect("Stored value is not a set");
        let removed_value = old_value_as_set
            .remove(&StoredAtomicValue::from_atomic_fr_value(value))
            .expect("Value not found in set");
        removed_value.to_atomic_fr_value()
    })
}
