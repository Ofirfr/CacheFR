use crate::{
    commands_proto::{self, FrKey, FrValue},
    consts::ERROR_EXPIRY,
    structs::{CacheFRMap, StoredFrValueWithExpiry},
};

use super::get::get_from_map;

pub async fn int_increment(main_map: &CacheFRMap, key: FrKey, amount: i32) -> Option<FrValue> {
    let maybe_old_value = get_from_map(main_map, key.clone()).await;

    match maybe_old_value {
        Some(old_value) => {
            let new_value = old_value.clone().value;
            let new_value = match new_value {
                Some(commands_proto::fr_value::Value::IntValue(old_int_value)) => FrValue {
                    value: Some(commands_proto::fr_value::Value::IntValue(
                        old_int_value + amount,
                    )),
                    expiry_timestamp_micros: old_value.expiry_timestamp_micros,
                },
                _ => FrValue {
                    value: Some(commands_proto::fr_value::Value::ErrValue(
                        "Current value is not an integer".to_string(),
                    )),
                    expiry_timestamp_micros: ERROR_EXPIRY,
                },
            };
            {
                main_map.insert(
                    key,
                    StoredFrValueWithExpiry::from_fr_value(new_value.clone()),
                );
            }

            Some(new_value)
        }
        None => None,
    }
}
