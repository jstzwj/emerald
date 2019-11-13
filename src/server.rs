
use std::error::Error;
use std::io;
use std::sync::Arc;
use std::sync::Mutex;
use std::net::SocketAddr;

use tokio::prelude::*;
use tokio::net::{TcpListener, TcpStream};


struct Shared {
}

impl Shared {
    /// Create a new, empty, instance of `Shared`.
    fn new() -> Self {
        Shared {
        }
    }

}

pub async fn app_loop() -> Result<(), Box<dyn Error>> {
    let state = Arc::new(Mutex::new(Shared::new()));
    let addr_str = "127.0.0.1:25565";
    let mut listener = TcpListener::bind(&addr_str).await?;

    loop {
        // Asynchronously wait for an inbound TcpStream.
        let (stream, addr) = listener.accept().await?;

        // Clone a handle to the `Shared` state for the new connection.
        let state = Arc::clone(&state);

        // Spawn our handler to be run asynchronously.
        tokio::spawn(async move {
            if let Err(e) = process(state, stream, addr).await {
                println!("an error occured; error = {:?}", e);
            }
        });
    }

    println!("server running on {:?}", addr_str);

    Ok(())
}


/// Process an individual chat client
async fn process(
    state: Arc<Mutex<Shared>>,
    stream: TcpStream,
    addr: SocketAddr,
) -> Result<(), Box<dyn Error>> {

    Ok(())
}