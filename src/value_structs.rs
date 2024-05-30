use crate::commands_proto::{self, FrKey, FrValue};
use dashmap::{DashMap, DashSet};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

pub type CacheFRMap = Arc<DashMap<FrKey, StoredFrValueWithExpiry>>;

#[derive(Clone, Debug)]
pub struct WrappedDashSet {
    pub wrapped_set: DashSet<StoredFrValueWithExpiry>,
}

impl Eq for WrappedDashSet {}

impl PartialEq for WrappedDashSet {
    fn eq(&self, other: &Self) -> bool {
        for item in self.wrapped_set.iter() {
            if !other.wrapped_set.contains(item.key()) {
                return false;
            }
        }
        true
    }
}

impl Hash for WrappedDashSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.wrapped_set.len().hash(state);
        for value in self.wrapped_set.iter() {
            value.hash(state);
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum StoredFrValue {
    IntValue(i32),
    StringValue(String),
    SetValue(WrappedDashSet),
    ListValue(Vec<StoredFrValueWithExpiry>),
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct StoredFrValueWithExpiry {
    pub value: StoredFrValue,
    pub expiry_timestamp_micros: u64,
}

impl StoredFrValueWithExpiry {
    pub fn from_fr_value(fr_value: FrValue) -> StoredFrValueWithExpiry {
        match fr_value.value {
            Some(commands_proto::fr_value::Value::IntValue(v)) => StoredFrValueWithExpiry {
                value: StoredFrValue::IntValue(v),
                expiry_timestamp_micros: fr_value.expiry_timestamp_micros,
            },
            Some(commands_proto::fr_value::Value::StringValue(v)) => StoredFrValueWithExpiry {
                value: StoredFrValue::StringValue(v),
                expiry_timestamp_micros: fr_value.expiry_timestamp_micros,
            },
            Some(commands_proto::fr_value::Value::SetValue(v)) => {
                let new_set = DashSet::<StoredFrValueWithExpiry>::new();
                for value in v.values {
                    new_set.insert(StoredFrValueWithExpiry::from_fr_value(value));
                }
                StoredFrValueWithExpiry {
                    value: StoredFrValue::SetValue(WrappedDashSet {
                        wrapped_set: new_set,
                    }),
                    expiry_timestamp_micros: fr_value.expiry_timestamp_micros,
                }
            }
            Some(commands_proto::fr_value::Value::ListValue(v)) => {
                let mut new_list = Vec::<StoredFrValueWithExpiry>::new();
                for value in v.values {
                    new_list.push(StoredFrValueWithExpiry::from_fr_value(value));
                }
                StoredFrValueWithExpiry {
                    value: StoredFrValue::ListValue(new_list),
                    expiry_timestamp_micros: fr_value.expiry_timestamp_micros,
                }
            }
            _ => panic!("No value in fr_value"),
        }
    }

    pub fn to_fr_value(&self) -> FrValue {
        match &self.value {
            StoredFrValue::IntValue(v) => FrValue {
                value: Some(commands_proto::fr_value::Value::IntValue(*v)),
                expiry_timestamp_micros: self.expiry_timestamp_micros,
            },
            StoredFrValue::StringValue(v) => FrValue {
                value: Some(commands_proto::fr_value::Value::StringValue(v.clone())),
                expiry_timestamp_micros: self.expiry_timestamp_micros,
            },
            StoredFrValue::SetValue(v) => FrValue {
                value: Some(commands_proto::fr_value::Value::SetValue(
                    commands_proto::SetValue {
                        values: v
                            .wrapped_set
                            .iter()
                            .map(|value| StoredFrValueWithExpiry::to_fr_value(value.key()))
                            .collect(),
                    },
                )),
                expiry_timestamp_micros: self.expiry_timestamp_micros,
            },
            StoredFrValue::ListValue(v) => FrValue {
                value: Some(commands_proto::fr_value::Value::ListValue(
                    commands_proto::ListValue {
                        values: v
                            .iter()
                            .map(|value| StoredFrValueWithExpiry::to_fr_value(value))
                            .collect(),
                    },
                )),
                expiry_timestamp_micros: self.expiry_timestamp_micros,
            },
        }
    }

    pub fn as_int(&self) -> Result<&i32, &str> {
        if let StoredFrValueWithExpiry {
            value: StoredFrValue::IntValue(v),
            ..  // Ignore expiry_timestamp_micros
        } = self
        {
            Ok(v)
        } else {
            Err("Not IntValue!")
        }
    }

    pub fn as_string(&self) -> Result<&String, &str> {
        if let StoredFrValueWithExpiry {
            value: StoredFrValue::StringValue(v),
            ..  // Ignore expiry_timestamp_micros
        } = self
        {
            Ok(v)
        } else {
            Err("Not StringValue!")
        }
    }

    pub fn as_set(&self) -> Result<&DashSet<StoredFrValueWithExpiry>, &str> {
        if let StoredFrValueWithExpiry {
            value: StoredFrValue::SetValue(v),
            ..  // Ignore expiry_timestamp_micros
        } = self
        {
            Ok(&v.wrapped_set)
        } else {
            Err("Not SetValue!")
        }
    }

    pub fn as_list(&self) -> Result<&Vec<StoredFrValueWithExpiry>, &str> {
        if let StoredFrValueWithExpiry {
            value: StoredFrValue::ListValue(v),
            ..  // Ignore expiry_timestamp_micros
        } = self
        {
            Ok(v)
        } else {
            Err("Not ListValue!")
        }
    }

    pub fn as_mut_list(&mut self) -> Result<&mut Vec<StoredFrValueWithExpiry>, &str> {
        if let StoredFrValueWithExpiry {
            value: StoredFrValue::ListValue(v),
            ..  // Ignore expiry_timestamp_micros
        } = self
        {
            Ok(v)
        } else {
            Err("Not ListValue!")
        }
    }
}
