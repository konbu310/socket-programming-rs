use std::env;
#[macro_use]
extern crate log;

mod tcp_client;
mod tcp_server;
mod udp_client;
mod udp_server;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    // [ 0: file, 1: tcp|udp, 2: server|client, 3: addr:port ]
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        error!("Please specify [tcp|udp] [server|client] [addr:port].");
        std::process::exit(1);
    }
    let protocol: &str = &args[1];
    let role: &str = &args[2];
    let address = &args[3];
    match protocol {
        "tcp" => match role {
            "server" => {
                tcp_server::serve(address).unwrap_or_else(|e| error!("{}", e));
            }
            "client" => {
                // TODO: TCP Client
            }
            _ => {
                missing_role();
            }
        },
        "udp" => match role {
            "server" => {
                // TODO: UDP Server
            }
            "client" => {
                // TODO: UDP Client
            }
            _ => {
                missing_role();
            }
        },
        _ => {
            error!("Please specify tcp or udp on the 1st argument.");
            std::process::exit(1);
        }
    }
}

fn missing_role() {
    error!("Please specify server or client on the 2nd argument.");
    std::process::exit(1);
}
