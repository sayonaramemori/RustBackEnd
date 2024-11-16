use crate::opcua_config::data_adaptor::interface::transfer::InstructionInfo;
use serde::Deserialize;
#[derive(Deserialize,Debug,Clone)]
pub struct Instruction {
    pub target: String,
    pub value: String,
}

impl InstructionInfo for Instruction {
    fn get_target(&self) -> String {
        return self.target.clone();
    }
    fn get_value(&self) -> String {
        return self.value.clone();
    }
}