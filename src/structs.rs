use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::commands_proto::{FrKey, FrValue};
pub struct CacheFRMap {
    pub map: Arc<RwLock<HashMap<FrKey, FrValue>>>,
}
