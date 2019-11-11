

use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) {
    // ...
}


pub fn app_loop() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:25565")?;

    println!("Server started.")

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }

    println!("Server stopped.")

    Ok(())
}