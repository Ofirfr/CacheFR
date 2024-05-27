use crate::commands_proto::{ErrValue, Key, Value};
pub use crate::structs::CacheFRMap;
pub async fn set_value_in_map<'a>(
    main_map: &CacheFRMap,
    key: Key,
    value: Value,
    only_if_not_exists: bool,
) -> (bool, Value) {
    if only_if_not_exists && main_map.map.read().await.contains_key(&key) {
        return (
            false,
            Value {
                value: Some(crate::commands_proto::value::Value::ErrValue(ErrValue {
                    err_message: "Key already exists, command set to not override".to_string(),
                })),
                expiry_timestamp_micros: 0,
            },
        );
    }
    main_map.map.write().await.insert(key.clone(), value);
    (
        true,
        main_map
            .map
            .read()
            .await
            .get(&key)
            .expect("There should be a value")
            .clone(),
    )
}
