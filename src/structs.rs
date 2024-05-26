use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
pub struct CacheFRMap {
    pub map: Arc<RwLock<HashMap<CacheFRKey, CacheFRValue>>>,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct CacheFRValue {
    pub value: String,
    pub expiry_timestamp_micros: u64,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct CacheFRKey {
    pub key: String,
}
