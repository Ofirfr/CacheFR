use crate::commands_proto::{Key, Value};
pub use crate::structs::CacheFRMap;
use std::time::{self, UNIX_EPOCH};

pub async fn get_from_map(main_map: &CacheFRMap, key: Key) -> Option<Value> {
    let read_guard = main_map.map.read().await;
    let result = read_guard.get(&key);
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
                main_map.map.write().await.remove(&key);
                None
            }
        }
        None => None,
    }
}
