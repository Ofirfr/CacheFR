use cache_fr::{add_to_map, get_from_map, MainMap, MapKey, MapValue};

use std::collections::HashMap;

fn main() {
    let mut map: HashMap<MapKey, MapValue> = HashMap::new();
    let mut main_map = MainMap { map: &mut map };
}
