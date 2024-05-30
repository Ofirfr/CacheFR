use crate::{
    commands_proto::{FrKey, FrValue},
    consts::ERROR_EXPIRY,
    structs::{CacheFRMap, StoredFrValueWithExpiry},
};
pub async fn set_value_in_map<'a>(
    main_map: &CacheFRMap,
    key: FrKey,
    value: FrValue,
    only_if_not_exists: bool,
) -> (bool, FrValue) {
    // Check only-if-exists-constraint
    if only_if_not_exists && main_map.contains_key(&key) {
        return (
            false,
            FrValue {
                value: Some(crate::commands_proto::fr_value::Value::ErrValue(
                    String::from("Key already exists"),
                )),
                expiry_timestamp_micros: ERROR_EXPIRY,
            },
        );
    }

    // Write block for minimal blocking time
    {
        main_map.insert(
            key.clone(),
            StoredFrValueWithExpiry::from_fr_value(value.clone()),
        );
    }
    return (true, value);
}
