use crate::{
    commands_proto::{self, FrKey, FrValue},
    value_structs::{CacheFRMap, StoredFrValue},
};

use super::get::get_from_map;

pub async fn int_increment(main_map: &CacheFRMap, key: FrKey, amount: i32) -> Option<FrValue> {
    let maybe_old_value = get_from_map(main_map, key.clone()).await;

    match maybe_old_value {
        Some(mut old_value) => {
            let new_int = old_value.as_int().expect("Not an int") + amount;
            let new_value = { StoredFrValue::IntValue(new_int) };
            old_value.value = new_value.clone();
            // {
            //     main_map.insert(
            //         key,
            //         StoredFrValueWithExpiry::from_fr_value(new_value.clone()),
            //     );
            // }

            Some(FrValue {
                value: Some(commands_proto::fr_value::Value::IntValue(new_int)),
                expiry_timestamp_micros: old_value.expiry_timestamp_micros,
            })
        }
        None => None,
    }
}
