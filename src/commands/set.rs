pub use crate::structs::{CacheFRKey, CacheFRMap, CacheFRValue};
pub async fn set_value_in_map<'a>(
    main_map: &CacheFRMap,
    key: String,
    value: String,
    only_if_not_exists: bool,
    expiry_timestamp_micros: u64,
) -> bool {
    let map_key = CacheFRKey { key };

    if only_if_not_exists && main_map.map.read().await.contains_key(&map_key) {
        return false;
    }
    let value = CacheFRValue {
        value,
        expiry_timestamp_micros,
    };
    main_map.map.write().await.insert(map_key, value);
    true
}
