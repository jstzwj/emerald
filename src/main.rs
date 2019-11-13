#![warn(rust_2018_idioms)]

extern crate tokio;

mod server;

use futures::executor::block_on;


fn main() -> std::io::Result<()> {
    
    let future = server::app_loop(); // Nothing is printed
    block_on(future);
    Ok(())
}
