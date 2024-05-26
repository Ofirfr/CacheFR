use std::collections::HashMap;

pub struct MainMap<'main_map> {
    pub map: &'main_map mut HashMap<MapKey<'main_map>, MapValue<'main_map>>,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct MapValue<'main_map> {
    pub value: &'main_map str,
    pub expiry_timestamp_micros: u128,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct MapKey<'main_map> {
    pub key: &'main_map str,
}
