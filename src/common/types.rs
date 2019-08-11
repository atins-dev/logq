use hashbrown::HashMap;
use ordered_float::OrderedFloat;
use std::path::PathBuf;
use chrono;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub(crate) enum Value {
    Int(i32),
    Float(OrderedFloat<f32>),
    Boolean(bool),
    String(String),
    Null,
    DateTime(chrono::DateTime::<chrono::offset::FixedOffset>),
}

pub(crate) type Tuple = Vec<Value>;
pub(crate) type VariableName = String;
pub(crate) type Variables = HashMap<VariableName, Value>;

pub(crate) fn empty_variables() -> Variables {
    Variables::default()
}

pub(crate) fn merge(left: Variables, right: Variables) -> Variables {
    left.into_iter().chain(right).collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DataSource {
    File(PathBuf),
    Stdin,
}
