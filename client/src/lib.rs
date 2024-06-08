pub mod commands_proto {
    tonic::include_proto!("commands_proto");
}

use commands_proto::commands_client::CommandsClient;
use commands_proto::{FrKey, FrResponse, SetRequest};

use bb8::{Pool, PooledConnection};
use std::error::Error;
use tonic::transport::{Channel, Endpoint};

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
        let pool = Pool::builder()
            .min_idle(8)
            .max_size(32)
            .build(manager)
            .await
            .unwrap();

        CommandsClientPool { pool }
    }

    pub async fn get(&self, key: FrKey) -> Result<FrResponse, Box<dyn Error>> {
        let conn: PooledConnection<'_, TonicConnectionManager> = self.pool.get().await?;
        let mut client = CommandsClient::new(conn.clone());

        let request = tonic::Request::new(key);
        let response = client.get(request).await?;

        Ok(response.into_inner())
    }

    pub async fn set(&self, request: SetRequest) -> Result<FrResponse, Box<dyn Error>> {
        let mut conn: PooledConnection<'_, TonicConnectionManager> = self.pool.get().await?;
        let mut client = CommandsClient::new(conn.clone());

        let request = tonic::Request::new(request);
        let response = client.set(request).await?;

        Ok(response.into_inner())
    }

    // Implement other methods similarly
}
