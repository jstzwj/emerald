extern crate tokio;
extern crate bytes;

mod protocol;
mod server;

use futures::executor::block_on;


fn main() -> Result<(), std::boxed::Box<(dyn std::error::Error + 'static)>> {
    
    let future = server::app_loop(); // Nothing is printed
    block_on(future)
}
