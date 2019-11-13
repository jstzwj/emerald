use std::io::Write;
use std::marker::Send;

use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

use async_trait::async_trait;


use super::super::Protocol;
use crate::serialization::reader::*;


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
}

#[async_trait]
impl Protocol for Ping {
    async fn serialize<T:Write + Send>(&self, stream: &mut T) {

    }

    async fn deserialize(stream: &mut TcpStream) -> Ping {
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

    
}

#[async_trait]
impl Protocol for Pong {
    async fn serialize<T:Write + Send>(&self, stream: &mut T) {

    }


    async fn deserialize(stream: &mut TcpStream) -> Pong {
        Pong{ payload: read_long(stream).await }
    }
}
