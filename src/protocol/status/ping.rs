use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

pub struct Ping {
    pub payload:i64
}

impl Ping {
    pub fn new() -> Ping {
        Ping{payload: 0}
    }

    pub fn deserialize(stream: TcpStream) -> Ping {

    }

    pub fn id(&self) -> u8 {
        return 0x01;
    }

    
}

pub struct Pong {
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