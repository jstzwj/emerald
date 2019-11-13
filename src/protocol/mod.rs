
pub mod status;

use std::io::Write;

use async_trait::async_trait;
use tokio::net::{TcpStream};
use tokio::io::*;


#[async_trait]
pub trait Protocol {
    async fn deserialize(stream: &mut TcpStream) -> Self;
    async fn serialize<T:Write + Send>(&self, stream: &mut T);
}