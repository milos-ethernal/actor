use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};

#[derive(Clone, Debug, Serialize_tuple, Deserialize_tuple, PartialEq, Eq)]
pub struct KeyValueType {
    pub key: u64,
    pub value: u64,
}