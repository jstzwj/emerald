extern crate tokio;

use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

fn incoming_dispatch(stream: &TcpStream) {
    let mut buf: [u8; 1] = [0; 1];
    tokio::io::read_exact(stream, buf)
        .and_then(|(_stream, buf)|{
            if buf[0] == 0x01 {
                println!("ping");
            }
        });
}

pub fn app_loop() -> std::io::Result<()> {
    let addr_str = "127.0.0.1:25565";
    let addr = addr_str.parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    // Here we convert the `TcpListener` to a stream of incoming connections
    // with the `incoming` method. We then define how to process each element in
    // the stream with the `for_each` combinator method
    let server = listener
        .incoming()
        .for_each(|stream| {
            // TODO: Process stream
            incoming_dispatch(&stream);
            Ok(())
        })
        .map_err(|err| {
            // Handle error by printing to STDOUT.
            println!("accept error = {:?}", err);
        });

    println!("server running on {:?}", addr_str);

    // Start the server
    //
    // This does a few things:
    //
    // * Start the Tokio runtime
    // * Spawns the `server` task onto the runtime.
    // * Blocks the current thread until the runtime becomes idle, i.e. all
    //   spawned tasks have completed.
    tokio::run(server);

    Ok(())
}
