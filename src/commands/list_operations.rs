use crate::{
    commands::get,
    commands_proto::{FrKey, FrValue},
    value_structs::CacheFRMap,
};

pub async fn list_append(main_map: CacheFRMap, key: FrKey, value: FrValue) -> Option<FrValue> {
    let maybe_old_value = get::get_from_map(&main_map, key.clone()).await;
    // maybe_old_value.ma
    todo!();
}
