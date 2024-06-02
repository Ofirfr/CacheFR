use std::result;

use tonic::{Request, Response, Status};

use crate::{
    commands::{
        get::get_from_map,
        int_operations::int_increment,
        list_operations::{list_append, list_remove},
        set::set_value_in_map,
        set_operations::{set_add, set_remove},
    },
    commands_proto::{
        self, commands_server::Commands, FrAtomicResponse, FrKey, FrResponse, IntCommand,
        SetRequest,
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
        let result = set_value_in_map(&self, key, value.clone(), only_if_not_exists).await;
        match result {
            Ok(value) => match return_value {
                true => Result::Ok(Response::new(FrResponse { value: None })),
                false => Result::Ok(Response::new(FrResponse { value: None })),
            },
            Err(e) => Result::Err(Status::already_exists(e)),
        }
    }

    async fn int_operation(
        &self,
        request: Request<IntCommand>,
    ) -> Result<Response<FrAtomicResponse>, Status> {
        let (key, command) = {
            let request = request.into_inner();
            (request.key.expect("key is required"), request.command)
        };
        match command {
            Some(commands_proto::int_command::Command::IncrementBy(increment_by)) => {
                let result = int_increment(&self, key, increment_by).await;
                return Result::Ok(Response::new(FrAtomicResponse {
                    value: result.map(|v| v.to_atomic_fr_value()),
                }));
            }
            _ => Result::Err(Status::unimplemented("Not implemented")),
        }
    }
    async fn list_operation(
        &self,
        request: Request<commands_proto::ListCommand>,
    ) -> Result<Response<FrAtomicResponse>, Status> {
        let (key, command) = {
            let request = request.into_inner();
            (request.key.expect("key is required"), request.command)
        };
        match command {
            Some(commands_proto::list_command::Command::Append(append_value)) => {
                let result = list_append(&self, key, append_value).await;
                return Result::Ok(Response::new(FrAtomicResponse {
                    value: result.map(|v| v.to_atomic_fr_value()),
                }));
            }
            _ => Result::Err(Status::unimplemented("Not implemented")),
        }
    }

    async fn set_operation(
        &self,
        request: Request<commands_proto::SetCommnad>,
    ) -> Result<Response<FrAtomicResponse>, Status> {
        let (key, command) = {
            let request = request.into_inner();
            (request.key.expect("key is required"), request.command)
        };
        match command {
            Some(commands_proto::set_commnad::Command::Add(add_value)) => {
                let result = set_add(&self, key, add_value).await;
                return Result::Ok(Response::new(FrAtomicResponse {
                    value: result.map(|v| v.to_atomic_fr_value()),
                }));
            }
            Some(commands_proto::set_commnad::Command::Remove(remove_value)) => {
                let result = set_remove(&self, key, remove_value).await;
                return Result::Ok(Response::new(FrAtomicResponse { value: result }));
            }
            _ => Result::Err(Status::unimplemented("Not implemented")),
        }
    }
}
