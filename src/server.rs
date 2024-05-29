use std::sync::Arc;

use cache_fr::commands::int_operations::int_increment;
use cache_fr::commands_proto::fr_key::Key;
use cache_fr::consts::ERROR_EXPIRY;
use cache_fr::structs::CacheFRMap;
use cache_fr::{commands, commands_proto};
use commands::get::get_from_map;
use commands::set::set_value_in_map;

use tonic::{transport::Server, Request, Response, Status};

use commands_proto::commands_server::{Commands, CommandsServer};
use commands_proto::{FrKey, FrResponse, FrValue, IntCommand, SetRequest};
use std::collections::HashMap;
use tokio::sync::RwLock;

struct CacheFRMapImpl {
    cache_fr_map: CacheFRMap,
}

#[tonic::async_trait]
impl Commands for CacheFRMapImpl {
    async fn get(&self, request: Request<FrKey>) -> Result<Response<FrResponse>, Status> {
        let key = request.into_inner();
        let maybe_value = get_from_map(&self.cache_fr_map, key).await;
        match maybe_value {
            Some(value) => Result::Ok(Response::new(FrResponse {
                success: true,
                value: Some(value),
            })),
            None => Result::Ok(Response::new(FrResponse {
                success: false,
                value: Some(FrValue {
                    value: Some(commands_proto::fr_value::Value::ErrValue(String::from(
                        "Key not found",
                    ))),
                    expiry_timestamp_micros: ERROR_EXPIRY,
                }),
            })),
        }
    }
    async fn set(&self, request: Request<SetRequest>) -> Result<Response<FrResponse>, Status> {
        // extract key from request
        let (key, value, only_if_not_exists, return_value) = {
            let request = request.into_inner();
            (
                request.key.expect("key is required"),
                request.value.expect("Value is required"),
                request.only_if_not_exists,
                request.return_value,
            )
        };
        //
        set_value_in_map(&self.cache_fr_map, key, value.clone(), only_if_not_exists).await;
        match return_value {
            true => Result::Ok(Response::new(FrResponse {
                success: true,
                value: Some(value),
            })),
            false => Result::Ok(Response::new(FrResponse {
                success: true,
                value: None,
            })),
        }
    }

    async fn int_operation(
        &self,
        request: Request<IntCommand>,
    ) -> Result<Response<FrResponse>, Status> {
        let (key, command) = {
            let request = request.into_inner();
            (request.key.expect("key is required"), request.command)
        };
        match command {
            Some(commands_proto::int_command::Command::IncrementBy(increment_by)) => {
                let result = int_increment(&self.cache_fr_map, key, increment_by).await;
                return Result::Ok(Response::new(FrResponse {
                    success: true,
                    value: result,
                }));
            }
            _ => Result::Err(Status::unimplemented("Not implemented")),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting server...");
    let addr = "[::1]:50051".parse()?;
    let cache_fr_service = CacheFRMapImpl {
        cache_fr_map: CacheFRMap {
            map: Arc::new(RwLock::new(HashMap::new())),
        },
    };

    Server::builder()
        .add_service(CommandsServer::new(cache_fr_service))
        .serve(addr)
        .await?;

    Ok(())
}
