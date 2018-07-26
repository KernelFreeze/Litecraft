// The MIT License (MIT)
// Copyright © 2014-2018 Miguel Peláez <kernelfreeze@outlook.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation
// files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy,
// modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
// OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#![feature(box_syntax)]
#![deny(unused_must_use)]
#![deny(unused_imports)]

#[macro_use]
extern crate glium;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

extern crate cgmath;
extern crate image;
extern crate pretty_env_logger;
extern crate serde;
extern crate serde_yaml;
extern crate threadpool;
extern crate zip;

mod core;
mod gfx;
mod scenes;
mod tests;

use core::constants::*;
use core::settings::Settings;
use gfx::canvas::Canvas;

use std::fs::{copy, File};
use std::path::Path;

fn main() {
    println!(
        "Starting Litecraft {} for Minecraft {}...\n{}",
        LITECRAFT_VERSION, MINECRAFT_VERSION, ASCII_ART
    );

    pretty_env_logger::init();

    if !Path::new("resources").exists() {
        warn!("Resources path doesn't exist, please check that you have all required resources.");
    }

    Canvas::new(load_settings()).draw();
}

fn load_settings() -> Settings {
    match File::open(CONFIG_FILE) {
        Err(why) => {
            warn!("Can't read configuration file: {}", why);
            generate_config()
        },
        Ok(file) => match serde_yaml::from_reader(file) {
            Err(why) => {
                warn!("Can't parse configuration file: {}", why);
                warn!("Regenerating, old configuration placed at {}.bak", CONFIG_FILE);

                if let Err(error) = copy(CONFIG_FILE, format!("{}.bak", CONFIG_FILE)) {
                    warn!("Failed to copy old configuration to .bak file. {}", error);
                }

                generate_config()
            },
            Ok(settings) => settings,
        },
    }
}

fn generate_config() -> Settings {
    use std::io::prelude::*;

    let config = Settings::new();
    let path = Path::new(CONFIG_FILE);

    let serialized = serde_yaml::to_string(&config).expect("Couldn't serialize configuration");

    File::create(&path)
        .expect("Couldn't create configuration file")
        .write_all(serialized.as_bytes())
        .expect("Couldn't write to configuration file");

    config
}
