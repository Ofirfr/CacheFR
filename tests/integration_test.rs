use cache_fr::commands::get;
use cache_fr::commands::set;
use cache_fr::commands_proto;
use cache_fr::commands_proto::Key;
use cache_fr::commands_proto::Value;
use cache_fr::structs::CacheFRMap;
use std::{
    collections::HashMap,
    sync::Arc,
    thread,
    time::{self, UNIX_EPOCH},
};
use tokio::sync::RwLock;

#[tokio::test]
async fn test_integration_expiry_on_keys() {
    let map = HashMap::new();
    let mut main_map = CacheFRMap {
        map: Arc::new(RwLock::new(map)),
    };

    let now_plus_a_second: u64 = (time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros()
        + 1_000_000) as u64;

    let key = Key {
        key: Some(commands_proto::key::Key::StringKey(
            commands_proto::StrKey {
                key: "my best key".to_string(),
            },
        )),
    };

    let value = Value {
        value: Some(commands_proto::value::Value::StringValue(
            commands_proto::StrValue {
                value: "has the best value".to_string(),
            },
        )),
        expiry_timestamp_micros: now_plus_a_second,
    };

    // add key that expires in 1 second

    set::set_value_in_map(&mut main_map, key.clone(), value.clone(), true).await;
    // key should still exist
    assert_eq!(
        get::get_from_map(&mut main_map, key.clone()).await,
        Some(value.clone())
    );

    thread::sleep(time::Duration::from_secs(1));
    // key should be expired
    assert_eq!(get::get_from_map(&mut main_map, key.clone()).await, None);
}
