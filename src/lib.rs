mod structs;

use std::time::{self, UNIX_EPOCH};

pub use structs::{MainMap, MapKey, MapValue};
pub fn add_to_map<'main_map>(
    main_map: &mut MainMap<'main_map>,
    key: &'main_map str,
    value: &'main_map str,
    only_if_not_exists: bool,
    expiry_timestamp_micros: u128,
) -> bool {
    let map_key = MapKey::<'main_map> { key };

    if only_if_not_exists && main_map.map.contains_key(&map_key) {
        return false;
    }
    let value = MapValue::<'main_map> {
        value,
        expiry_timestamp_micros,
    };
    main_map.map.insert(map_key, value);
    true
}

pub fn get_from_map<'main_map>(
    main_map: &mut MainMap<'main_map>,
    key: &'main_map str,
) -> Option<MapValue<'main_map>> {
    let map_key = MapKey::<'main_map> { key };
    let result = main_map.map.get(&map_key).map(|x| *x);
    match result {
        Some(map_value) => {
            if map_value.expiry_timestamp_micros
                > time::SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_micros()
            {
                // key is alive
                Some(map_value)
            } else {
                // key has expired
                main_map.map.remove(&map_key);
                None
            }
        }
        None => None,
    }
}
