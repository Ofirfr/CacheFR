use crate::{
    commands_proto::{AtomicFrValue, FrKey},
    value_structs::{CacheFRMap, StoredAtomicValue},
};

use super::get;

pub async fn set_add(
    main_map: &CacheFRMap,
    key: FrKey,
    value: AtomicFrValue,
) -> Result<StoredAtomicValue, String> {
    let maybe_old_value = get::get_from_map_as_mut(&main_map, key.clone()).await;
    match maybe_old_value {
        Some(mut old_value) => {
            let old_value_as_set = old_value
                .as_mut_set()
                .map_err(|e| format!("Error while parsing value to mut set: {}", e))?;
            let value_to_insert = StoredAtomicValue::from_atomic_fr_value(value);
            old_value_as_set.insert(value_to_insert.clone());
            Ok(value_to_insert)
        }
        None => Err("Key does not exist".to_string()),
    }
}

pub async fn set_remove(
    main_map: &CacheFRMap,
    key: FrKey,
    value: AtomicFrValue,
) -> Result<StoredAtomicValue, String> {
    let maybe_old_value = get::get_from_map_as_mut(&main_map, key.clone()).await;
    match maybe_old_value {
        Some(mut old_value) => {
            let old_value_as_set = old_value
                .as_mut_set()
                .map_err(|e| format!("Error while parsing value to mut set: {}", e))?;
            let removed_value =
                old_value_as_set.remove(&StoredAtomicValue::from_atomic_fr_value(value));
            match removed_value {
                Some(v) => Ok(v),
                None => Err("Value does not exist".to_string()),
            }
        }
        None => Err("Key does not exist".to_string()),
    }
}
