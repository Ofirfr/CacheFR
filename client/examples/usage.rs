use client::{commands_proto::FrKey, CommandsClientPool};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = CommandsClientPool::new("http://[::1]:50051").await;

    let key = FrKey {
        key: Some(client::commands_proto::fr_key::Key::StringKey(
            "example".to_string(),
        )),
    };

    let response = pool.get(key).await?;
    println!("GET Response: {:?}", response);

    Ok(())
}
