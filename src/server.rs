use std::sync::Arc;

use structs::CacheFRMap;
use tonic::{transport::Server, Request, Response, Status};
mod commands;
mod structs;

use commands::get::get_from_map;
use commands::set::set_value_in_map;

use commands_proto::commands_server::{Commands, CommandsServer};
use commands_proto::{SetRequest, SetResponse, StrKey, StrValue};
use std::collections::HashMap;
use tokio::sync::RwLock;

pub mod commands_proto {
    tonic::include_proto!("commands_proto");
}

#[tonic::async_trait]
impl Commands for CacheFRMap {
    async fn get(&self, request: Request<StrKey>) -> Result<Response<StrValue>, Status> {
        let key = request.into_inner().key;
        match get_from_map(&self, key).await {
            Some(value) => Ok(Response::new(StrValue {
                value: value.value.to_string(),
            })),
            None => Ok(Response::new(StrValue {
                value: "".to_string(),
            })),
        }
    }
    async fn set(&self, request: Request<SetRequest>) -> Result<Response<SetResponse>, Status> {
        let (key, value, only_if_not_exists, expiry_timestamp_micros) = {
            let request = request.into_inner();
            (
                request.key,
                request.value,
                request.only_if_not_exists,
                request.expiry_timestamp_micros,
            )
        };
        match set_value_in_map(
            &self,
            key,
            value,
            only_if_not_exists,
            expiry_timestamp_micros,
        )
        .await
        {
            true => Ok(Response::new(SetResponse {
                success: true,
                value: None,
            })),
            false => Ok(Response::new(SetResponse {
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
