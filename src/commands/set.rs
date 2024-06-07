use crate::{
    commands_proto::{FrKey, FrValue},
    value_structs::{CacheFRMap, StoredFrValueWithExpiry},
};

pub async fn set_value_in_map(
    main_map: &CacheFRMap,
    key: FrKey,
    value: FrValue,
    only_if_not_exists: bool,
) -> Result<(), String> {
    // Check only-if-exists-constraint
    if only_if_not_exists && main_map.contains_key(&key) {
        return Err("Key already exists".to_string());
    }
    let value_to_store = StoredFrValueWithExpiry::from_fr_value(value);

    // Write block for minimal blocking time
    {
        main_map.insert(key, value_to_store);
    }
    Ok(())
}
