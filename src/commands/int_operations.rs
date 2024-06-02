use crate::{
    commands_proto::{self, AtomicFrValue, FrKey, FrValue},
    value_structs::{CacheFRMap, StoredAtomicValue, StoredFrValue},
};

use super::get::get_from_map;

pub async fn int_increment(
    main_map: &CacheFRMap,
    key: FrKey,
    amount: i32,
) -> Option<StoredAtomicValue> {
    let maybe_old_value = get_from_map(main_map, key.clone()).await;

    match maybe_old_value {
        Some(mut old_value) => {
            let stored_int = old_value.as_mut_int().expect("Not an int");
            *stored_int += amount;

            Some(StoredAtomicValue::IntValue((*stored_int)))
        }

        None => None,
    }
}
