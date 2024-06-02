use std::{
    sync::Arc,
    thread,
    time::{self, UNIX_EPOCH},
};

use cache_fr::{
    commands_proto::{
        self, commands_server::Commands as _, AtomicFrValue, FrKey, FrResponse, FrValue, SetRequest,
    },
    main_map_impl::CacheFRMapImpl,
};
use dashmap::DashMap;
use tonic::{Code, Request};

#[tokio::test]
async fn test_integration_expiry_on_keys() {
    let cache_fr_service: CacheFRMapImpl = Arc::new(DashMap::new());

    let now_plus_a_second: u64 = (time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros()
        + 1_000_000) as u64;

    let key = FrKey {
        key: Some(commands_proto::fr_key::Key::StringKey(
            "my best key for expiry testing".to_string(),
        )),
    };

    let value = FrValue {
        value: Some(commands_proto::fr_value::Value::AtomicValue(
            AtomicFrValue {
                value: Some(commands_proto::atomic_fr_value::Value::IntValue(100)),
            },
        )),
        expiry_timestamp_micros: now_plus_a_second,
    };

    let set_request = SetRequest {
        key: Some(key.clone()),
        value: Some(value.clone()),
        only_if_not_exists: false,
        return_value: false,
    };
    // add key that expires in 1 second
    let _ = cache_fr_service.set(Request::new(set_request)).await;
    // key should still exist

    // match first_get
    assert_eq!(
        cache_fr_service
            .get(Request::new(key.clone()))
            .await
            .unwrap()
            .into_inner(),
        FrResponse {
            value: Some(value.clone()),
        }
    );

    thread::sleep(time::Duration::from_secs(1));

    // key should be expired
    assert_eq!(
        cache_fr_service
            .get(Request::new(key.clone()))
            .await
            .expect_err("Key should be expired")
            .code(),
        Code::NotFound
    );
    assert_eq!(0, cache_fr_service.len());
}
