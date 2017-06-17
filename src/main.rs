#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate kiss3d;
extern crate glfw;
extern crate nalgebra as na;
extern crate num_traits as num;
extern crate uuid;
extern crate byteorder;

mod version;
mod camera;
mod resource_manager;
mod block;
mod networking;
mod protocol;
mod client;

use client::*;

fn main() {
    env_logger::init().unwrap();

    let matches = clap_app!(litecraft =>
        (version: version::VERSION)
        (author: "Miguel Peláez <kernelfreeze@outlook.com>")
        (about: "Open source, clean room implementation of Minecraft Client")
        (@arg username: +required "Sets the user name")
        (@arg session: +required "Sets the user session id")
        (@arg server: -s ... "Join a server")
    ).get_matches();

    info!("Litecraft {} for Minecraft {}. Using protocol v{}",
          version::VERSION,
          version::MINECRAFT,
          version::PROTOCOL);

    let username = matches.value_of("username").unwrap();
    let session = matches.value_of("session").unwrap();
    let server = matches.value_of("server");

    let mut client = Client::new();
    client.run();
}
