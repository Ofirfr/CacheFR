use cache_fr_client::{
    commands_proto::{AtomicFrValue, FrKey, FrValue, SetRequest},
    CommandsClientPool,
};
use stopwatch::Stopwatch;

pub mod commands_proto {
    tonic::include_proto!("commands_proto");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = CommandsClientPool::new("http://[::1]:50051").await;

    let stopwatch = Stopwatch::start_new();

    let num_operations = 10000;

    let mut handles = vec![];

    let num_of_threads = 5;

    for _ in 0..num_of_threads {
        let pool_clone = pool.clone();

        let handle = tokio::spawn(async move {
            for _ in 0..(num_operations / 5) {
                let key: u32 = 123;
                let value: u32 = 123;

                let request = SetRequest {
                    key: Some(FrKey {
                        key: Some(cache_fr_client::commands_proto::fr_key::Key::StringKey(
                            key.to_string(),
                        )),
                    }),
                    value: Some(FrValue {
                        value: Some(
                            cache_fr_client::commands_proto::fr_value::Value::AtomicValue(
                                AtomicFrValue {
                                    value: Some(
                                        cache_fr_client::commands_proto::atomic_fr_value::Value::IntValue(
                                            value as i32,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        expiry_timestamp_micros: 1,
                    }),
                    only_if_not_exists: false,
                    return_value: false,
                };

                let _ = pool_clone.set(request).await;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await?;
    }

    let elapsed_time = stopwatch.elapsed_ms();
    println!(
        "Elapsed time for {} gRPC SET operations: {} ms with {} threads",
        num_operations, elapsed_time, num_of_threads
    );

    Ok(())
}
