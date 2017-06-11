#![feature(proc_macro)]
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate kiss3d;
extern crate glfw;
extern crate nalgebra as na;
extern crate num_traits as num;

extern crate uuid;

mod version;
mod camera;
mod resource_manager;
mod block;
mod protocol;
mod client;

use client::*;

fn main() {
    env_logger::init().unwrap();
    info!("Litecraft {} for Minecraft {}. Using protocol v{}",
          version::VERSION,
          version::MINECRAFT,
          version::PROTOCOL);
    let mut client = Client::new();
    client.run();
}
