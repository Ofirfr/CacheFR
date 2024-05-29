use crate::{
    commands_proto::{FrKey, FrValue},
    consts::NO_EXPIRY,
    structs::CacheFRMap,
};
use std::time::{self, UNIX_EPOCH};

pub async fn read_from_map_block(main_map: &CacheFRMap, key: FrKey) -> Option<FrValue> {
    let read_guard = main_map.map.read().await;
    read_guard.get(&key).map(|map_value| map_value.clone())
}

pub async fn get_from_map(main_map: &CacheFRMap, key: FrKey) -> Option<FrValue> {
    let result: Option<FrValue> = read_from_map_block(main_map, key.clone()).await;
    match result {
        Some(map_value) => {
            if map_value.expiry_timestamp_micros == NO_EXPIRY
                || map_value.expiry_timestamp_micros
                    > time::SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_micros() as u64
            {
                // key is alive
                Some(map_value.clone())
            } else {
                // key has expired
                {
                    main_map.map.write().await.remove(&key);
                }
                None
            }
        }
        None => None,
    }
}
