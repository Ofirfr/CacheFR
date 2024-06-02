use std::sync::Arc;

use cache_fr::{
    commands_proto::{
        self, commands_server::Commands as _, AtomicFrValue, FrKey, FrValue, SetRequest,
    },
    consts::NO_EXPIRY,
    main_map_impl::CacheFRMapImpl,
    value_structs::StoredFrValueWithExpiry,
};
use dashmap::DashMap;
use tonic::Request;

#[tokio::test]
async fn test_integration_list_appending() {
    let cache_fr_service: CacheFRMapImpl = Arc::new(DashMap::new());

    let key: FrKey = FrKey {
        key: Some(commands_proto::fr_key::Key::StringKey(
            "my best key".to_string(),
        )),
    };

    let initial_list_value: Vec<AtomicFrValue> = vec![];

    let value = FrValue {
        value: Some(commands_proto::fr_value::Value::ListValue(
            commands_proto::ListValue {
                values: initial_list_value.clone(),
            },
        )),
        expiry_timestamp_micros: NO_EXPIRY,
    };

    let set_request = SetRequest {
        key: Some(key.clone()),
        value: Some(value.clone()),
        only_if_not_exists: false,
        return_value: false,
    };

    // Set value and check that it is set
    let _ = cache_fr_service.set(Request::new(set_request)).await;
    assert_eq!(
        cache_fr_service
            .get(Request::new(key.clone()))
            .await
            .unwrap()
            .into_inner()
            .value,
        Some(value.clone())
    );

    // Append to list multiple times from different threads
    let mut handles = vec![];
    let num_of_threads = 10;
    for _ in 0..num_of_threads {
        // Clone items for each thread
        let arc_cache_fr_service: CacheFRMapImpl = Arc::clone(&cache_fr_service);
        let key_clone = key.clone();

        let handle = tokio::spawn(async move {
            for j in 0..num_of_threads {
                let key_clone = key_clone.clone();
                let list_append_command = commands_proto::ListCommand {
                    key: Some(key_clone),
                    command: Some(commands_proto::list_command::Command::Append(
                        AtomicFrValue {
                            value: Some(commands_proto::atomic_fr_value::Value::IntValue(j)),
                        },
                    )),
                };
                let _ = arc_cache_fr_service
                    .list_operation(Request::new(list_append_command))
                    .await;
            }
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        let _ = handle.await;
    }

    // println!("{}", expected_sum);
    let list_result = StoredFrValueWithExpiry::from_fr_value(
        cache_fr_service
            .get(Request::new(key.clone()))
            .await
            .unwrap()
            .into_inner()
            .value
            .unwrap(),
    );
    let expected_len = num_of_threads * num_of_threads + initial_list_value.len() as i32;
    // Check that the list was appended correctly
    assert_eq!(list_result.as_list().unwrap().len() as i32, expected_len);
}
