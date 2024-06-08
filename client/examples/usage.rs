use cache_fr_client::{commands_proto::FrKey, CommandsClientPool};
use std::time::SystemTime;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = CommandsClientPool::new("http://[::1]:50051").await;

    let key = FrKey {
        key: Some(cache_fr_client::commands_proto::fr_key::Key::StringKey(
            "example".to_string(),
        )),
    };
    let start_time = SystemTime::now();
    let response = pool.get(key).await?;
    println!("GET Response: {:?}", response);
    let end_time = SystemTime::now();
    println!(
        "GET time: {:?}",
        end_time.duration_since(start_time).unwrap()
    );
    Ok(())
}
