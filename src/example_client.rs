use std::time::{self, UNIX_EPOCH};

use commands_proto::commands_client::CommandsClient;
use commands_proto::SetRequest;
use commands_proto::{Key, StrKey};

use crate::commands_proto::{StrValue, Value};

pub mod commands_proto {
    tonic::include_proto!("commands_proto");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CommandsClient::connect("http://[::1]:50051").await?;

    let now_plus_a_second = (time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros()
        + 1_000_000) as u64;

    let request = tonic::Request::new(SetRequest {
        key: Some(Key {
            key: Some(commands_proto::key::Key::StringKey(StrKey {
                key: "my_key".to_string(),
            })),
        }),
        value: Some(Value {
            value: Some(commands_proto::value::Value::StringValue(StrValue {
                value: "my_key".to_string(),
            })),
            expiry_timestamp_micros: now_plus_a_second,
        }),
        only_if_not_exists: true,
        return_value: true,
    });

    let response = client.set(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
