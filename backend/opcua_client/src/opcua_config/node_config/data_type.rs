use serde::{Deserialize,Serialize};
#[derive(Deserialize,Serialize,Debug,Clone)]
pub enum DataType{
    Boolean,
    Int16,
    Int32,
    Int64,
    UInt16,
    UInt32,
    UInt64,
    Float,
    Double,
}