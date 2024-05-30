use tonic::{Request, Response, Status};

use crate::{
    commands::{
        get::get_from_map, int_operations::int_increment, list_operations::list_append,
        set::set_value_in_map,
    },
    commands_proto::{
        self, commands_server::Commands, FrKey, FrResponse, IntCommand, SetRequest,
    },
    value_structs::CacheFRMap,
};

pub type CacheFRMapImpl = CacheFRMap;

#[tonic::async_trait]
impl Commands for CacheFRMapImpl {
    async fn get(&self, request: Request<FrKey>) -> Result<Response<FrResponse>, Status> {
        let key = request.into_inner();
        let maybe_value = get_from_map(&self, key).await;
        match maybe_value {
            Some(value) => Result::Ok(Response::new(FrResponse {
                success: true,
                value: Some(value.to_fr_value()),
            })),
            None => Result::Err(Status::not_found("Key not found")),
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
        set_value_in_map(&self, key, value.clone(), only_if_not_exists).await;
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
                let result = int_increment(&self, key, increment_by).await;
                return Result::Ok(Response::new(FrResponse {
                    success: true,
                    value: result,
                }));
            }
            _ => Result::Err(Status::unimplemented("Not implemented")),
        }
    }
    async fn list_operation(
        &self,
        request: Request<commands_proto::ListCommand>,
    ) -> Result<Response<FrResponse>, Status> {
        let (key, command) = {
            let request = request.into_inner();
            (request.key.expect("key is required"), request.command)
        };
        match command {
            Some(commands_proto::list_command::Command::Append(append_value)) => {
                let result = list_append(&self, key, append_value).await;
                return Result::Ok(Response::new(FrResponse {
                    success: true,
                    value: result,
                }));
            }
            _ => Result::Err(Status::unimplemented("Not implemented")),
        }
    }
}
