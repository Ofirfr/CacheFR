use crate::{
    commands::get,
    commands_proto::{self, FrKey, FrValue},
    value_structs::{CacheFRMap, StoredFrValueWithExpiry},
};

pub async fn list_append(main_map: &CacheFRMap, key: FrKey, value: FrValue) -> Option<FrValue> {
    let maybe_old_value = get::get_from_map(&main_map, key.clone()).await;
    maybe_old_value.map(|mut old_value| {
        let old_value_as_list = old_value.as_mut_list().expect("Stored value is not a list");
        old_value_as_list.push(StoredFrValueWithExpiry::from_fr_value(value));

        FrValue {
            value: Some(commands_proto::fr_value::Value::ListValue(
                commands_proto::ListValue {
                    values: old_value_as_list
                        .iter()
                        .map(|value| StoredFrValueWithExpiry::to_fr_value(value))
                        .collect(),
                },
            )),
            expiry_timestamp_micros: old_value.expiry_timestamp_micros,
        }
    })
}
