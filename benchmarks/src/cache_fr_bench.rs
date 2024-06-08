use cache_fr_client::{
    commands_proto::commands_client::CommandsClient,
    commands_proto::{AtomicFrValue, FrKey, FrValue, SetRequest},
    CommandsClientPool,
};
use std::time::{self, UNIX_EPOCH};

use rand::Rng;
use stopwatch::Stopwatch;

pub mod commands_proto {
    tonic::include_proto!("commands_proto");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = CommandsClientPool::new("http://[::1]:50051").await;

    let mut rng = rand::thread_rng();
    let stopwatch = Stopwatch::start_new();

    let num_operations = 10000;

    let mut total_server_time_micro = 0;

    for _ in 0..num_operations {
        let key: u32 = rng.gen();
        let value: u32 = rng.gen();

        let request = SetRequest {
            key: Some(FrKey {
                key: Some(cache_fr_client::commands_proto::fr_key::Key::StringKey(
                    key.to_string(),
                )),
            }),
            value: Some(FrValue {
                value: Some(
                    cache_fr_client::commands_proto::fr_value::Value::AtomicValue(AtomicFrValue {
                        value: Some(
                            cache_fr_client::commands_proto::atomic_fr_value::Value::IntValue(
                                value as i32,
                            ),
                        ),
                    }),
                ),
                expiry_timestamp_micros: 1,
            }),
            only_if_not_exists: false,
            return_value: false,
        };

        let stopwatch_server = Stopwatch::start_new();

        let _ = pool.set(request).await?;

        total_server_time_micro += stopwatch_server.elapsed().as_micros();
    }

    let elapsed_time = stopwatch.elapsed_ms();
    println!(
        "Elapsed time for {} gRPC SET operations: {} ms",
        num_operations, elapsed_time
    );

    println!(
        "Average server time per operation: {} micro seconds",
        total_server_time_micro / num_operations
    );

    Ok(())
}
