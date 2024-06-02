use std::fmt::format;

use crate::{
    commands::get,
    commands_proto::{self, AtomicFrValue, FrKey, FrValue},
    value_structs::{CacheFRMap, StoredAtomicValue, StoredFrValueWithExpiry},
};

pub async fn list_append(
    main_map: &CacheFRMap,
    key: FrKey,
    value: AtomicFrValue,
) -> Result<StoredAtomicValue, String> {
    let maybe_old_value = get::get_from_map_as_mut(&main_map, key.clone()).await;
    match maybe_old_value {
        Some(mut old_value) => {
            let stored_list = old_value
                .as_mut_list()
                .map_err(|e| format!("Error while parsing value to list: {}", e))?;
            let value_to_push = StoredAtomicValue::from_atomic_fr_value(value);
            stored_list.push(value_to_push.clone());
            Ok(value_to_push)
        }
        None => Err("Key does not exist".to_string()),
    }
}

pub async fn list_remove(
    main_map: &CacheFRMap,
    key: FrKey,
    index: i32,
) -> Result<StoredAtomicValue, String> {
    let maybe_old_value = get::get_from_map_as_mut(&main_map, key.clone()).await;
    match maybe_old_value {
        Some(mut old_value) => {
            let stored_list = old_value
                .as_mut_list()
                .map_err(|e| format!("Error while parsing value to list: {}", e))?;
            let removed_val = stored_list.remove(index as usize);
            Ok(removed_val)
        }
        None => Err("Key does not exist".to_string()),
    }
}
