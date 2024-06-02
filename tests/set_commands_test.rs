use std::sync::Arc;

use cache_fr::{
    commands_proto::{
        self, commands_server::Commands as _, AtomicFrValue, FrKey, FrValue, SetCommnad, SetRequest,
    },
    consts::NO_EXPIRY,
    main_map_impl::CacheFRMapImpl,
    value_structs::{StoredAtomicValue, StoredFrValueWithExpiry},
};
use dashmap::DashMap;
use tonic::Request;

#[tokio::test]
async fn test_integration_set_add_and_remove() {
    let cache_fr_service: CacheFRMapImpl = Arc::new(DashMap::new());

    let key: FrKey = FrKey {
        key: Some(commands_proto::fr_key::Key::StringKey(
            "my best key".to_string(),
        )),
    };

    let initial_set_value = FrValue {
        value: Some(commands_proto::fr_value::Value::SetValue(
            commands_proto::SetValue { values: vec![] },
        )),
        expiry_timestamp_micros: NO_EXPIRY,
    };

    // Create the set
    let set_request = SetRequest {
        key: Some(key.clone()),
        value: Some(initial_set_value.clone()),
        only_if_not_exists: false,
        return_value: false,
    };
    // Create the set and check that it is created
    let _ = cache_fr_service.set(Request::new(set_request)).await;
    assert_eq!(
        cache_fr_service
            .get(Request::new(key.clone()))
            .await
            .unwrap()
            .into_inner()
            .value,
        Some(initial_set_value.clone())
    );

    let first_value = AtomicFrValue {
        value: Some(commands_proto::atomic_fr_value::Value::IntValue(0)),
    };
    let second_value = AtomicFrValue {
        value: Some(commands_proto::atomic_fr_value::Value::IntValue(1)),
    };
    let third_value = AtomicFrValue {
        value: Some(commands_proto::atomic_fr_value::Value::StringValue(
            "HELLO".to_string(),
        )),
    };

    // Add the values to the set
    let _ = cache_fr_service
        .set_operation(Request::new(SetCommnad {
            key: Some(key.clone()),
            command: Some(commands_proto::set_commnad::Command::Add(
                first_value.clone(),
            )),
        }))
        .await;

    let _ = cache_fr_service
        .set_operation(Request::new(SetCommnad {
            key: Some(key.clone()),
            command: Some(commands_proto::set_commnad::Command::Add(
                second_value.clone(),
            )),
        }))
        .await;

    let _ = cache_fr_service
        .set_operation(Request::new(SetCommnad {
            key: Some(key.clone()),
            command: Some(commands_proto::set_commnad::Command::Add(
                third_value.clone(),
            )),
        }))
        .await;

    // Test that the values are set correctly
    let set_result = StoredFrValueWithExpiry::from_fr_value(
        cache_fr_service
            .get(Request::new(key.clone()))
            .await
            .unwrap()
            .into_inner()
            .value
            .unwrap(),
    );
    let result_as_set = set_result.as_set().unwrap();
    assert_eq!(result_as_set.len(), 3);
    assert!(result_as_set.contains(&StoredAtomicValue::from_atomic_fr_value(first_value)));
    assert!(result_as_set.contains(&StoredAtomicValue::from_atomic_fr_value(second_value)));
    assert!(result_as_set.contains(&StoredAtomicValue::from_atomic_fr_value(third_value)));
}
