use tonic::{Request, Response, Status};

use crate::{
    commands::{
        get::get_from_map,
        int_operations::int_increment,
        list_operations::{
            list_append, list_first_appearance, list_remove_by_index, list_remove_by_value,
        },
        set::set_value_in_map,
        set_operations::{set_add, set_contains, set_remove},
    },
    commands_proto::{
        self, atomic_fr_value, commands_server::Commands, AtomicFrValue, FrAtomicResponse, FrKey,
        FrResponse, IntCommand, SetRequest,
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
        let (key, value, only_if_not_exists, return_value) = {
            let request = request.into_inner();
            (
                request
                    .key
                    .ok_or(Status::invalid_argument("key is required"))?,
                request
                    .value
                    .ok_or(Status::invalid_argument("value is required"))?,
                request.only_if_not_exists,
                request.return_value,
            )
        };
        let result = set_value_in_map(&self, key, value.clone(), only_if_not_exists).await;
        match result {
            Ok(()) => match return_value {
                true => Result::Ok(Response::new(FrResponse { value: Some(value) })),
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
            (
                request
                    .key
                    .ok_or(Status::invalid_argument("key is required"))?,
                request.command,
            )
        };
        match command {
            Some(commands_proto::int_command::Command::IncrementBy(increment_by)) => {
                let result = int_increment(&self, key, increment_by).await;
                match result {
                    Ok(atomic_value) => Result::Ok(Response::new(FrAtomicResponse {
                        value: Some(atomic_value.to_atomic_fr_value()),
                    })),
                    Err(e) => Result::Err(Status::aborted(e)),
                }
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
            (
                request
                    .key
                    .ok_or(Status::invalid_argument("key is required"))?,
                request.command,
            )
        };
        match command {
            Some(commands_proto::list_command::Command::Append(append_value)) => {
                let result = list_append(&self, key, append_value).await;
                match result {
                    Ok(atomic_value) => Result::Ok(Response::new(FrAtomicResponse {
                        value: Some(atomic_value.to_atomic_fr_value()),
                    })),
                    Err(e) => Result::Err(Status::aborted(e)),
                }
            }
            Some(commands_proto::list_command::Command::RemoveAt(remove_by_index)) => {
                let result = list_remove_by_index(&self, key, remove_by_index).await;
                match result {
                    Ok(atomic_value) => Result::Ok(Response::new(FrAtomicResponse {
                        value: Some(atomic_value.to_atomic_fr_value()),
                    })),
                    Err(e) => Result::Err(Status::aborted(e)),
                }
            }
            Some(commands_proto::list_command::Command::RemoveAll(remove_by_value)) => {
                let result = list_remove_by_value(&self, key, remove_by_value).await;
                match result {
                    Ok(removed_count) => Result::Ok(Response::new(FrAtomicResponse {
                        value: Some(AtomicFrValue {
                            value: Some(atomic_fr_value::Value::IntValue(removed_count)),
                        }),
                    })),
                    Err(e) => Result::Err(Status::aborted(e)),
                }
            }
            Some(commands_proto::list_command::Command::FirstAppearance(searched_value)) => {
                let result = list_first_appearance(&self, key, searched_value).await;
                match result {
                    Ok(first_index) => Result::Ok(Response::new(FrAtomicResponse {
                        value: Some(first_index.to_atomic_fr_value()),
                    })),
                    Err(e) => Result::Err(Status::aborted(e)),
                }
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
            (
                request
                    .key
                    .ok_or(Status::invalid_argument("key is required"))?,
                request.command,
            )
        };
        match command {
            Some(commands_proto::set_commnad::Command::Add(add_value)) => {
                let result = set_add(&self, key, add_value).await;
                match result {
                    Ok(atomic_value) => Result::Ok(Response::new(FrAtomicResponse {
                        value: Some(atomic_value.to_atomic_fr_value()),
                    })),
                    Err(e) => Result::Err(Status::aborted(e)),
                }
            }
            Some(commands_proto::set_commnad::Command::Remove(remove_value)) => {
                let result = set_remove(&self, key, remove_value).await;
                match result {
                    Ok(atomic_value) => Result::Ok(Response::new(FrAtomicResponse {
                        value: Some(atomic_value.to_atomic_fr_value()),
                    })),
                    Err(e) => Result::Err(Status::aborted(e)),
                }
            }
            Some(commands_proto::set_commnad::Command::Contains(searched_value)) => {
                let result = set_contains(&self, key, searched_value).await;
                match result {
                    Ok(contains) => Result::Ok(Response::new(FrAtomicResponse {
                        value: Some(AtomicFrValue {
                            value: Some(atomic_fr_value::Value::BoolValue(contains)),
                        }),
                    })),
                    Err(e) => Result::Err(Status::aborted(e)),
                }
            }
            _ => Result::Err(Status::unimplemented("Not implemented")),
        }
    }
}
