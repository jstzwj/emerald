use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

use super::super::*;

pub struct Ping {
    pub payload:i64
}

impl Ping {
    pub fn new() -> Ping {
        Ping{payload: 0}
    }

    pub fn id(&self) -> u8 {
        return 0x01;
    }

    pub async fn deserialize(stream: &mut TcpStream) -> Ping {
        Ping{ payload: read_long(stream).await }
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

    pub async fn deserialize(stream: &mut TcpStream) -> Ping {
        Ping{ payload: read_long(stream).await }
    }
}
