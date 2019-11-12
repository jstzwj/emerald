struct Ping {
    pub payload:i64
}

impl Ping {
    pub fn new() -> Ping {
        Ping{payload: 0}
    }

    pub fn id(&self) -> u8 {
        return 0x01;
    }
}

struct Pong {
    pub payload:i64
}

impl Pong {
    pub fn new() -> Pong {
        Pong{payload: 0}
    }
    
    pub fn id(&self) -> u8 {
        return 0x01;
    }
}