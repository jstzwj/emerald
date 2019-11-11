mod server;

fn main() -> std::io::Result<()> {
    server::app_loop()
}
