use std::time::{self, UNIX_EPOCH};

use commands_proto::commands_client::CommandsClient;
use commands_proto::SetRequest;

pub mod commands_proto {
    tonic::include_proto!("commands_proto");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CommandsClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(SetRequest {
        key: "my_key".to_string(),
        value: "my_value".to_string(),
        only_if_not_exists: true,
        expiry_timestamp_micros: (time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros()
            + 1_000_000) as u64,
        return_value: true,
    });

    let response = client.set(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
