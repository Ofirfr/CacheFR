use commands_proto::commands_client::CommandsClient;
use commands_proto::{AtomicFrValue, FrKey, FrResponse, FrValue, SetRequest};
use tonic::transport::Channel;

use bb8::{Pool, PooledConnection};
use tonic::transport::Endpoint;

pub mod commands_proto {
    tonic::include_proto!("commands_proto");
}

pub struct TonicConnectionManager {
    addr: String,
}

impl TonicConnectionManager {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
}

#[tonic::async_trait]
impl bb8::ManageConnection for TonicConnectionManager {
    type Connection = Channel;
    type Error = tonic::transport::Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let endpoint = Endpoint::from_shared(self.addr.clone())?;
        let channel = endpoint.connect().await?;
        Ok(channel)
    }

    async fn is_valid(&self, _conn: &mut Self::Connection) -> Result<(), Self::Error> {
        Ok(())
    }

    fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
        false
    }
}

#[derive(Clone)]
pub struct CommandsClientPool {
    pool: Pool<TonicConnectionManager>,
}

impl CommandsClientPool {
    pub async fn new(addr: &str) -> Self {
        let manager = TonicConnectionManager::new(addr.to_string());
        let pool = Pool::builder().build(manager).await.unwrap();
        CommandsClientPool { pool }
    }

    pub async fn get(&self, key: FrKey) -> Result<FrResponse, Box<dyn std::error::Error>> {
        let mut conn: PooledConnection<'_, TonicConnectionManager> = self.pool.get().await?;
        let mut client = CommandsClient::new(conn.clone());

        let request = tonic::Request::new(key);
        let response = client.get(request).await?;

        Ok(response.into_inner())
    }

    pub async fn set(&self, request: SetRequest) -> Result<FrResponse, Box<dyn std::error::Error>> {
        let mut conn: PooledConnection<'_, TonicConnectionManager> = self.pool.get().await?;
        let mut client = CommandsClient::new(conn.clone());

        let request = tonic::Request::new(request);
        let response = client.set(request).await?;

        Ok(response.into_inner())
    }

    // Implement other methods similarly
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = CommandsClientPool::new("http://[::1]:50051").await;

    let key = FrKey {
        key: Some(commands_proto::fr_key::Key::StringKey(
            "example".to_string(),
        )),
    };
    let value = FrValue {
        value: Some(commands_proto::fr_value::Value::AtomicValue({
            AtomicFrValue {
                value: Some(commands_proto::atomic_fr_value::Value::IntValue(100)),
            }
        })),
        expiry_timestamp_micros: 1,
    };
    let request = SetRequest {
        key: Some(key.clone()),
        value: Some(value),
        only_if_not_exists: false,
        return_value: false,
    };

    let time_before = std::time::Instant::now();

    let response = pool.set(request.clone()).await?;

    println!("SET Response: {:?}", response);

    let response1 = pool.get(key.clone()).await?;
    let response2 = pool.get(key.clone()).await?;
    let response3 = pool.get(key.clone()).await?;
    let response4 = pool.get(key.clone()).await?;
    let response5 = pool.get(key.clone()).await?;

    println!("GET Response: {:?}", response1);
    println!("GET Response: {:?}", response2);
    println!("GET Response: {:?}", response3);
    println!("GET Response: {:?}", response4);
    println!("GET Response: {:?}", response5);

    let time_after = std::time::Instant::now();

    println!("Time elapsed: {:?}", time_after - time_before);

    Ok(())
}
