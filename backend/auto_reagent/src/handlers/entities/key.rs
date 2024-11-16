#[derive(Clone)]
pub struct JWTKEY {
    key: String
}

impl JWTKEY {
    pub fn new(key:String) -> JWTKEY{
        JWTKEY { key }
    }
    pub fn get_key(&self) ->&[u8]{
        return self.key.as_bytes();
    }
}