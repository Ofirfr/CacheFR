#[cfg(test)]
mod tests {
    extern crate cache_fr;
    use cache_fr::commands::get;
    use cache_fr::commands::int_operations::int_increment;
    use cache_fr::commands::set;
    use cache_fr::commands_proto;
    use cache_fr::commands_proto::FrKey;
    use cache_fr::commands_proto::FrValue;
    use cache_fr::consts::NO_EXPIRY;
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

        let key = FrKey {
            key: Some(commands_proto::fr_key::Key::StringKey(
                "my best key".to_string(),
            )),
        };

        let value = FrValue {
            value: Some(commands_proto::fr_value::Value::StringValue(
                "has the best value".to_string(),
            )),
            expiry_timestamp_micros: now_plus_a_second,
        };

        // add key that expires in 1 second
        set::set_value_in_map(&mut main_map, key.clone(), value.clone(), false).await;
        // key should still exist
        assert_eq!(
            get::get_from_map(&mut main_map, key.clone()).await,
            Some(value.clone())
        );

        thread::sleep(time::Duration::from_secs(1));

        // key should be expired
        assert_eq!(get::get_from_map(&mut main_map, key.clone()).await, None);
        assert_eq!(0, main_map.map.read().await.keys().len());
    }

    #[tokio::test]
    async fn test_integration_int_increment() {
        let map = HashMap::new();
        let mut main_map = CacheFRMap {
            map: Arc::new(RwLock::new(map)),
        };
        let key: FrKey = FrKey {
            key: Some(commands_proto::fr_key::Key::StringKey(
                "my best key".to_string(),
            )),
        };
        let value = FrValue {
            value: Some(commands_proto::fr_value::Value::IntValue(100)),
            expiry_timestamp_micros: NO_EXPIRY,
        };
        // Set value and check that it is set
        set::set_value_in_map(&mut main_map, key.clone(), value.clone(), false).await;
        assert_eq!(
            get::get_from_map(&mut main_map, key.clone()).await,
            Some(value.clone())
        );

        // Increment value multiple times and check that it is incremented right
        let mut sum = 100;
        for i in 0..100 {
            sum += i;
            int_increment(&mut main_map, key.clone(), i).await;
        }

        assert_eq!(
            get::get_from_map(&mut main_map, key.clone()).await,
            Some(FrValue {
                value: Some(commands_proto::fr_value::Value::IntValue(sum)),
                expiry_timestamp_micros: NO_EXPIRY
            })
        );
    }
}
