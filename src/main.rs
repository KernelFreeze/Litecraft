#![deny(unused_imports)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;
extern crate badlog;
extern crate allegro;

mod scenes;
mod client;

fn main() {
    badlog::init_from_env("LOG_LEVEL");

    let matches = clap_app!(litecraft =>
        (version: client::VERSION)
        (author: "Litecraft Team")
        (about: "Open source, clean room implementation of Minecraft Client")
        (@arg session: +required "Sets the user session ID")
        (@arg server: -s ... "Auto-join a server")
    ).get_matches();

    // Litecraft is love, Litecraft is life!
    println!(
        r"  _    _ _                    __ _   
 | |  (_) |_ ___ __ _ _ __ _ / _| |_ 
 | |__| |  _/ -_) _| '_/ _` |  _|  _|
 |____|_|\__\___\__|_| \__,_|_|  \__|
                                     "
    );
    println!("{} for Minecraft {}", client::VERSION, client::MINECRAFT);
    info!("Starting engine...");

    client::run(matches.value_of("session").unwrap());
}
