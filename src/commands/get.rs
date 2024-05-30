use crate::{
    commands_proto::{FrKey, FrValue},
    consts::NO_EXPIRY,
    value_structs::{CacheFRMap, StoredFrValueWithExpiry},
};
use std::{
    sync::Arc,
    time::{self, UNIX_EPOCH},
};

pub async fn read_from_map_block(
    main_map: &CacheFRMap,
    key: FrKey,
) -> Option<StoredFrValueWithExpiry> {
    let main_map_clone = Arc::clone(&main_map);
    let result = main_map_clone.get(&key);
    match result {
        Some(map_value) => Some(map_value.to_owned()),
        None => None,
    }
}

pub async fn get_from_map(main_map: &CacheFRMap, key: FrKey) -> Option<StoredFrValueWithExpiry> {
    let result: Option<StoredFrValueWithExpiry> = read_from_map_block(main_map, key.clone()).await;
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
                Some(map_value)
            } else {
                // key has expired
                {
                    main_map.remove(&key);
                }
                None
            }
        }
        None => None,
    }
}
