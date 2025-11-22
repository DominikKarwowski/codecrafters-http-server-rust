mod endpoints;

use codecrafters_http_server::{start_http_server, ServerSettings};
use std::env;

fn main() {
    println!("Logs from your program will appear here!");

    let server_settings = ServerSettings::from_env_args(env::args());

    start_http_server(server_settings, endpoints::handler);
}
