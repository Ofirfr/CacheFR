use crate::{
    commands_proto::FrKey,
    value_structs::{CacheFRMap, StoredAtomicValue},
};

use super::get::get_from_map_as_mut;

pub async fn int_increment(
    main_map: &CacheFRMap,
    key: FrKey,
    amount: i32,
) -> Result<StoredAtomicValue, String> {
    let maybe_old_value = get_from_map_as_mut(main_map, key.clone()).await;

    match maybe_old_value {
        Some(mut old_value) => {
            let stored_int = old_value
                .as_mut_int()
                .map_err(|e| format!("Error while parsing value to int: {}", e))?;
            *stored_int += amount;

            Ok(StoredAtomicValue::IntValue(*stored_int))
        }

        None => Err("Key does not exist".to_string()),
    }
}
