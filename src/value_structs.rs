use crate::commands_proto::{self, atomic_fr_value, AtomicFrValue, FrKey, FrValue};
use dashmap::{DashMap, DashSet};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

pub type CacheFRMap = Arc<DashMap<FrKey, StoredFrValueWithExpiry>>;

#[derive(Clone, Debug)]
pub struct WrappedDashSet {
    pub wrapped_set: DashSet<StoredAtomicValue>,
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
pub enum StoredAtomicValue {
    IntValue(i32),
    StringValue(String),
    BoolValue(bool),
}

impl StoredAtomicValue {
    pub fn from_atomic_fr_value(atomic_fr_value: AtomicFrValue) -> StoredAtomicValue {
        match atomic_fr_value.value {
            Some(commands_proto::atomic_fr_value::Value::IntValue(v)) => {
                StoredAtomicValue::IntValue(v)
            }
            Some(commands_proto::atomic_fr_value::Value::StringValue(v)) => {
                StoredAtomicValue::StringValue(v)
            }
            Some(commands_proto::atomic_fr_value::Value::BoolValue(v)) => {
                StoredAtomicValue::BoolValue(v)
            }
            _ => panic!("Not an atomic value!"),
        }
    }
    pub fn to_atomic_fr_value(&self) -> AtomicFrValue {
        match self {
            StoredAtomicValue::IntValue(v) => AtomicFrValue {
                value: Some(atomic_fr_value::Value::IntValue(*v)),
            },
            StoredAtomicValue::StringValue(v) => AtomicFrValue {
                value: Some(atomic_fr_value::Value::StringValue(v.clone())),
            },
            StoredAtomicValue::BoolValue(v) => AtomicFrValue {
                value: Some(atomic_fr_value::Value::BoolValue(*v)),
            },
            _ => panic!("Not an atomic value!"),
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum StoredFrValue {
    AtomicValue(StoredAtomicValue),
    SetValue(WrappedDashSet),
    ListValue(Vec<StoredAtomicValue>),
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct StoredFrValueWithExpiry {
    pub value: StoredFrValue,
    pub expiry_timestamp_micros: u64,
}

impl StoredFrValueWithExpiry {
    pub fn from_fr_value(fr_value: FrValue) -> StoredFrValueWithExpiry {
        match fr_value.value {
            Some(commands_proto::fr_value::Value::AtomicValue(v)) => match v.value {
                Some(commands_proto::atomic_fr_value::Value::IntValue(v)) => {
                    StoredFrValueWithExpiry {
                        value: StoredFrValue::AtomicValue(StoredAtomicValue::IntValue(v)),
                        expiry_timestamp_micros: fr_value.expiry_timestamp_micros,
                    }
                }
                Some(commands_proto::atomic_fr_value::Value::StringValue(v)) => {
                    StoredFrValueWithExpiry {
                        value: StoredFrValue::AtomicValue(StoredAtomicValue::StringValue(v)),
                        expiry_timestamp_micros: fr_value.expiry_timestamp_micros,
                    }
                }
                _ => panic!("Invalid value type"),
            },
            Some(commands_proto::fr_value::Value::SetValue(v)) => {
                let new_set = DashSet::<StoredAtomicValue>::new();
                for value in v.values {
                    new_set.insert(StoredAtomicValue::from_atomic_fr_value(value));
                }
                StoredFrValueWithExpiry {
                    value: StoredFrValue::SetValue(WrappedDashSet {
                        wrapped_set: new_set,
                    }),
                    expiry_timestamp_micros: fr_value.expiry_timestamp_micros,
                }
            }
            Some(commands_proto::fr_value::Value::ListValue(v)) => {
                let mut new_list = Vec::<StoredAtomicValue>::new();
                for value in v.values {
                    new_list.push(StoredAtomicValue::from_atomic_fr_value(value));
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
            StoredFrValue::AtomicValue(v) => {
                let atomic_value = match v {
                    StoredAtomicValue::IntValue(v) => AtomicFrValue {
                        value: Some(commands_proto::atomic_fr_value::Value::IntValue(*v)),
                    },
                    StoredAtomicValue::StringValue(v) => AtomicFrValue {
                        value: Some(commands_proto::atomic_fr_value::Value::StringValue(
                            v.clone(),
                        )),
                    },
                    StoredAtomicValue::BoolValue(v) => AtomicFrValue {
                        value: Some(commands_proto::atomic_fr_value::Value::BoolValue(*v)),
                    },
                };
                FrValue {
                    value: Some(commands_proto::fr_value::Value::AtomicValue(atomic_value)),
                    expiry_timestamp_micros: self.expiry_timestamp_micros,
                }
            }
            StoredFrValue::SetValue(v) => FrValue {
                value: Some(commands_proto::fr_value::Value::SetValue(
                    commands_proto::SetValue {
                        values: v
                            .wrapped_set
                            .iter()
                            .map(|value| StoredAtomicValue::to_atomic_fr_value(value.key()))
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
                            .map(|value| StoredAtomicValue::to_atomic_fr_value(value))
                            .collect(),
                    },
                )),
                expiry_timestamp_micros: self.expiry_timestamp_micros,
            },
        }
    }

    pub fn as_int(&self) -> Result<&i32, &str> {
        if let StoredFrValueWithExpiry {
            value: StoredFrValue::AtomicValue(StoredAtomicValue::IntValue(v)),
            ..  // Ignore expiry_timestamp_micros
        } = self
        {
            Ok(v)
        } else {
            Err("Not IntValue!")
        }
    }

    pub fn as_mut_int(&mut self) -> Result<&mut i32, &str> {
        if let StoredFrValueWithExpiry {
            value: StoredFrValue::AtomicValue(StoredAtomicValue::IntValue(v)),
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
            value: StoredFrValue::AtomicValue(StoredAtomicValue::StringValue(v)),
            ..  // Ignore expiry_timestamp_micros
        } = self
        {
            Ok(v)
        } else {
            Err("Not IntValue!")
        }
    }

    pub fn as_bool(&self) -> Result<bool, &str> {
        if let StoredFrValueWithExpiry {
            value: StoredFrValue::AtomicValue(StoredAtomicValue::BoolValue(v)),
            ..  // Ignore expiry_timestamp_micros
        } = self
        {
            Ok(*v)
        } else {
            Err("Not IntValue!")
        }
    }

    pub fn as_set(&self) -> Result<&DashSet<StoredAtomicValue>, &str> {
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

    pub fn as_mut_set(&mut self) -> Result<&mut DashSet<StoredAtomicValue>, &str> {
        if let StoredFrValueWithExpiry {
            value: StoredFrValue::SetValue(v),
            ..  // Ignore expiry_timestamp_micros
        } = self
        {
            Ok(&mut v.wrapped_set)
        } else {
            Err("Not SetValue!")
        }
    }

    pub fn as_list(&self) -> Result<&Vec<StoredAtomicValue>, &str> {
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

    pub fn as_mut_list(&mut self) -> Result<&mut Vec<StoredAtomicValue>, &str> {
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
