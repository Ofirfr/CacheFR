use crate::structs::{add_to_map, get_from_map, CacheFRKey, CacheFRMap, CacheFRValue};
use std::{
    collections::HashMap,
    thread,
    time::{self, UNIX_EPOCH},
};

#[test]
fn test_integration_expiry_on_keys() {
    let mut map = HashMap::new();
    let mut main_map = CacheFRMap { map: &mut map };

    let key = "my_best_key";
    let value = "has_the_best_value";

    let now_plus_a_second = time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros()
        + 1_000_000;

    // add key that expires in 1 second
    add_to_map(&mut main_map, key, &value, true, now_plus_a_second);
    // key should still exist
    assert_eq!(
        get_from_map(&mut main_map, key),
        Some(CacheFRValue {
            value,
            expiry_timestamp_micros: now_plus_a_second
        })
    );

    thread::sleep(time::Duration::from_secs(1));
    // key should be expired
    assert_eq!(get_from_map(&mut main_map, key), None);
}
