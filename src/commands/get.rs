pub use crate::structs::{CacheFRKey, CacheFRMap, CacheFRValue};
use std::time::{self, UNIX_EPOCH};

pub async fn get_from_map(main_map: &CacheFRMap, key: String) -> Option<CacheFRValue> {
    let map_key = CacheFRKey { key: key };
    let result = main_map.map.read().await.get(&map_key).map(|v| v.clone());
    match result {
        Some(map_value) => {
            if map_value.expiry_timestamp_micros
                > time::SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_micros() as u64
            {
                // key is alive
                Some(map_value.clone())
            } else {
                // key has expired
                main_map.map.write().await.remove(&map_key);
                None
            }
        }
        None => None,
    }
}
