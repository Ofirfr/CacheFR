use std::sync::Arc;

use structs::CacheFRMap;
use tonic::{transport::Server, Request, Response, Status};
pub mod commands;
pub mod structs;

use commands::get::get_from_map;
use commands::set::set_value_in_map;

use commands_proto::commands_server::{Commands, CommandsServer};
use commands_proto::{Key, SetRequest, SetResponse, Value};
use std::collections::HashMap;
use tokio::sync::RwLock;

pub mod commands_proto {
    tonic::include_proto!("commands_proto");
}

#[tonic::async_trait]
impl Commands for CacheFRMap {
    async fn get(&self, request: Request<Key>) -> Result<Response<Value>, Status> {
        let key = request.into_inner();
        Result::Ok(Response::new(get_from_map(&self, key).await.unwrap()))
    }
    async fn set(&self, request: Request<SetRequest>) -> Result<Response<SetResponse>, Status> {
        let (key, value, only_if_not_exists, return_value) = {
            let request = request.into_inner();
            (
                request.key.expect("key is required"),
                request.value.expect("Value is required"),
                request.only_if_not_exists,
                request.return_value,
            )
        };
        match set_value_in_map(&self, key, value, only_if_not_exists).await {
            (true, value) => Ok(Response::new(SetResponse {
                success: true,
                value: {
                    if return_value {
                        Some(value)
                    } else {
                        None
                    }
                },
            })),
            (false, _) => Ok(Response::new(SetResponse {
                success: false,
                value: None,
            })),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let cache_fr_service = CacheFRMap {
        map: Arc::new(RwLock::new(HashMap::new())),
    };

    Server::builder()
        .add_service(CommandsServer::new(cache_fr_service))
        .serve(addr)
        .await?;

    Ok(())
}
