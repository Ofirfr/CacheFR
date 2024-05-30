use dashmap::mapref::one::RefMut;

use crate::{
    commands_proto::{FrKey},
    consts::NO_EXPIRY,
    value_structs::{CacheFRMap, StoredFrValueWithExpiry},
};
use std::{
    time::{self, UNIX_EPOCH},
};

pub async fn get_from_map(
    main_map: &CacheFRMap,
    key: FrKey,
) -> Option<RefMut<FrKey, StoredFrValueWithExpiry>> {
    let result: Option<RefMut<FrKey, StoredFrValueWithExpiry>> = {
        // read block for minimal blocking time
        main_map.get_mut(&key.clone())
    };
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
                std::mem::drop(map_value); // Drop pointer to avoid deadlock
                {
                    main_map.remove(&key);
                }
                None
            }
        }
        None => None,
    }
}
