mod server;
use laminar::ErrorKind;

fn main() -> Result<(), ErrorKind> {
    server::server()
}