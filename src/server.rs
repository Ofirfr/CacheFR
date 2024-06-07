use std::sync::Arc;

use cache_fr::{commands_proto, main_map_impl::CacheFRMapImpl};

use dashmap::DashMap;
use tonic::transport::Server;

use commands_proto::commands_server::CommandsServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting server...");
    let addr = "[::]:50051".parse()?;
    let cache_fr_service: CacheFRMapImpl = Arc::new(DashMap::new());

    Server::builder()
        .add_service(CommandsServer::new(cache_fr_service))
        .serve(addr)
        .await?;

    Ok(())
}
