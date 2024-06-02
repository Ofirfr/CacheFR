use std::sync::Arc;

use cache_fr::{
    commands_proto::{
        self, commands_server::Commands as _, AtomicFrValue, FrKey, FrValue, SetRequest,
    },
    consts::NO_EXPIRY,
    main_map_impl::CacheFRMapImpl,
};
use dashmap::DashMap;
use tonic::Request;

#[tokio::test]
async fn test_integration_int_increment() {
    let cache_fr_service: CacheFRMapImpl = Arc::new(DashMap::new());

    let key: FrKey = FrKey {
        key: Some(commands_proto::fr_key::Key::StringKey(
            "my best key".to_string(),
        )),
    };

    let initial_int_value = 100;
    let value = FrValue {
        value: Some(commands_proto::fr_value::Value::AtomicValue({
            AtomicFrValue {
                value: Some(commands_proto::atomic_fr_value::Value::IntValue(
                    initial_int_value,
                )),
            }
        })),
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

    // Increment value multiple times in different threads
    let mut handles = vec![];
    let num_of_threads = 10;
    for _ in 0..num_of_threads {
        // Clone items for each thread
        let arc_cache_fr_service: CacheFRMapImpl = Arc::clone(&cache_fr_service);
        let key_clone = key.clone();

        let handle = tokio::spawn(async move {
            for j in 0..num_of_threads {
                let key_clone = key_clone.clone();
                let int_increment_command = commands_proto::IntCommand {
                    key: Some(key_clone),
                    command: Some(commands_proto::int_command::Command::IncrementBy(j)),
                };
                let _ = arc_cache_fr_service
                    .int_operation(Request::new(int_increment_command))
                    .await;
            }
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        let _ = handle.await;
    }

    let expected_sum =
        initial_int_value + num_of_threads * num_of_threads * (num_of_threads - 1) / 2;
    // println!("{}", expected_sum);

    // Get value and check that the value was incremented correctly
    assert_eq!(
        cache_fr_service
            .get(Request::new(key.clone()))
            .await
            .unwrap()
            .into_inner()
            .value,
        Some(FrValue {
            value: Some(commands_proto::fr_value::Value::AtomicValue({
                AtomicFrValue {
                    value: Some(commands_proto::atomic_fr_value::Value::IntValue(
                        expected_sum,
                    )),
                }
            })),
            expiry_timestamp_micros: NO_EXPIRY
        })
    );
}
