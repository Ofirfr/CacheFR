use std::time::{self, UNIX_EPOCH};

use commands_proto::commands_client::CommandsClient;
use commands_proto::FrKey;
use commands_proto::SetRequest;

use crate::commands_proto::FrValue;

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
        key: Some(FrKey {
            key: Some(commands_proto::fr_key::Key::StringKey("my_key".to_string())),
        }),
        value: Some(FrValue {
            value: Some(commands_proto::fr_value::Value::StringValue(
                "my_value".to_string(),
            )),
            expiry_timestamp_micros: now_plus_a_second,
        }),
        only_if_not_exists: true,
        return_value: true,
    });

    let response = client.set(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
