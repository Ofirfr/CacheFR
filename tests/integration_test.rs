#[cfg(test)]
mod tests {
    extern crate cache_fr;
    use cache_fr::commands_proto;
    use cache_fr::commands_proto::commands_server::Commands;
    use cache_fr::commands_proto::FrKey;
    use cache_fr::commands_proto::FrResponse;
    use cache_fr::commands_proto::FrValue;
    use cache_fr::commands_proto::SetRequest;
    use cache_fr::consts::NO_EXPIRY;
    use cache_fr::main_map_impl::CacheFRMapImpl;
    use cache_fr::value_structs::StoredFrValueWithExpiry;
    use dashmap::DashMap;
    use std::{
        sync::Arc,
        thread,
        time::{self, UNIX_EPOCH},
    };
    use tonic::Code;
    use tonic::Request;
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
            value: Some(commands_proto::fr_value::Value::StringValue(
                "has the best value".to_string(),
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
        cache_fr_service.set(Request::new(set_request)).await;
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
                success: true
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
            value: Some(commands_proto::fr_value::Value::IntValue(initial_int_value)),
            expiry_timestamp_micros: NO_EXPIRY,
        };

        let set_request = SetRequest {
            key: Some(key.clone()),
            value: Some(value.clone()),
            only_if_not_exists: false,
            return_value: false,
        };

        // Set value and check that it is set
        cache_fr_service.set(Request::new(set_request)).await;
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
        for i in 0..num_of_threads {
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
            handle.await;
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
                value: Some(commands_proto::fr_value::Value::IntValue(expected_sum)),
                expiry_timestamp_micros: NO_EXPIRY
            })
        );
    }
    #[tokio::test]
    async fn test_integration_list_appending() {
        let cache_fr_service: CacheFRMapImpl = Arc::new(DashMap::new());

        let key: FrKey = FrKey {
            key: Some(commands_proto::fr_key::Key::StringKey(
                "my best key".to_string(),
            )),
        };

        let initial_list_value: Vec<FrValue> = vec![];

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
        cache_fr_service.set(Request::new(set_request)).await;
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
        for i in 0..num_of_threads {
            // Clone items for each thread
            let arc_cache_fr_service: CacheFRMapImpl = Arc::clone(&cache_fr_service);
            let key_clone = key.clone();

            let handle = tokio::spawn(async move {
                for j in 0..num_of_threads {
                    let key_clone = key_clone.clone();
                    let list_append_command = commands_proto::ListCommand {
                        key: Some(key_clone),
                        command: Some(commands_proto::list_command::Command::Append(FrValue {
                            value: Some(commands_proto::fr_value::Value::IntValue(j)),
                            expiry_timestamp_micros: NO_EXPIRY,
                        })),
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
            handle.await;
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
}
