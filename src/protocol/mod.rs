use tokio::net::{TcpStream};
use tokio::io::*;
pub mod status;

use bytes::{ByteOrder, BigEndian};

pub async fn read_long(stream: &mut TcpStream) -> i64 {
    let mut buf :[u8;4] = [0; 4];
    stream.read_exact(&mut buf).await.expect("failed to read packet from socket");
    return BigEndian::read_i64(&buf);
}

trait Protocol {
    fn deserialize(stream: &mut TcpStream) -> Self;
    fn serialize(&self) ->  Vec<u8>;
}