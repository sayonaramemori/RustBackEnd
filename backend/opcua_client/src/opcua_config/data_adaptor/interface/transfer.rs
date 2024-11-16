pub trait InstructionInfo : Send + Sync{
    fn get_target(&self) -> String;
    fn get_value(&self) -> String;
}