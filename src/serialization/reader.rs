
use tokio::io::AsyncReadExt;
use bytes::Buf;
use std::io::Cursor;
use bytes::{ByteOrder, BigEndian};

use std::marker::Unpin;

pub async fn read_boolean<T>(stream: &mut T) -> bool
    where T: Unpin + AsyncReadExt {
    let mut buf :[u8;1] = [0; 1];
    stream.read_exact(&mut buf).await.expect("failed to read boolean from socket");
    return match buf[0]
    {
        0x01 => true,
        0x00 => false,
        _ => true
    };
}

pub async fn read_byte<T>(stream: &mut T) -> i8
    where T: Unpin + AsyncReadExt{
    let mut buf :[u8;1] = [0; 1];
    stream.read_exact(&mut buf).await.expect("failed to read byte from socket");
    let mut cbuf = Cursor::new(buf);
    return cbuf.get_i8();
}

pub async fn read_ubyte<T>(stream: &mut T) -> u8
    where T: Unpin + AsyncReadExt{
    let mut buf :[u8;1] = [0; 1];
    stream.read_exact(&mut buf).await.expect("failed to read ubyte from socket");
    return buf[0];
}

pub async fn read_long<T>(stream: &mut T) -> i64
    where T: Unpin + AsyncReadExt{
    let mut buf :[u8;4] = [0; 4];
    stream.read_exact(&mut buf).await.expect("failed to read long from socket");
    return BigEndian::read_i64(&buf);
}