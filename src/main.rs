// Uncomment this block to pass the first stage
use std::env;


mod server;
mod threading;

use server::Server;

fn main() {
    let args: Vec<String> = env::args().collect();

    let server = Server::setup(
        "127.0.0.1:4221",
        4,
        &args,
    );

    server.run();
}
