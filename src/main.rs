extern crate tokio;
extern crate bytes;
extern crate async_trait;

mod protocol;
mod serialization;
mod server;
mod router;

use futures::executor::block_on;


#[tokio::main]
pub async fn main() -> Result<(), std::boxed::Box<(dyn std::error::Error + 'static)>> {
    
    let future = server::app_loop();
    block_on(future)
}
