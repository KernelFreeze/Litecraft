#![deny(unused_imports)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

extern crate config;
extern crate rand;
extern crate colored_logger;
extern crate allegro;
extern crate os_type;
extern crate dirs;
extern crate smallvec;
extern crate event_dispatcher;

mod scenes;
mod client;
mod configuration;

use rand::Rng;
use dirs::Directories;
use std::env::set_current_dir;

fn main() {
    colored_logger::init().unwrap();

    let matches = clap_app!(litecraft =>
        (version: client::VERSION)
        (author: "Litecraft Team")
        (about: "Open source, clean room implementation of Minecraft Client")
        (@arg session: +required "Sets the user session ID")
        (@arg server: -s +takes_value "Auto-join a server")
        (@arg path: -p --path +takes_value "Litecraft home, must have all resources available")
    ).get_matches();

    // Epic hardcoded quotes!
    let hello = [
        // Litecrafty
        "Litecraft is love, Litecraft is life!",
        "Now available on bluray",
        "Very fast!",
        "Sky is the Limit (y = 255)",
        "Open Source!",
        "Less bugs! (I hope...)",
        "Works on Linux!",

        // Nice...
        "Knowledge is having the right answer. Intelligence is asking the right question",
        "Wake me up when it's all over",
        "A person who never made a mistake never tried anything new",
        "There is nothing permanent except change",
        "If you cannot do great things, do small things in a great way",
        "The journey of a thousand miles begins with one step",

        // Random stuff
        "Citrate Caffeine 1 oz\nExtract Vanilla 1 oz\nFlavouring 2.5 oz",
        "Triskaidekaphobic, 13",
    ];

    println!(
        r"  _    _ _                    __ _   
 | |  (_) |_ ___ __ _ _ __ _ / _| |_ 
 | |__| |  _/ -_) _| '_/ _` |  _|  _|
 |____|_|\__\___\__|_| \__,_|_|  \__|
                                     "
    );
    println!("{}\n", rand::thread_rng().choose(&hello).unwrap());

    let os = os_type::current_platform();
    println!(
        "{} for Minecraft Modern {}, running on {:?} {}",
        client::VERSION,
        client::MINECRAFT,
        os.os_type,
        os.version
    );
    info!("Starting Soulsand engine...");

    // Set litecraft path
    let path;
    if let Some(i) = matches.value_of("path") {
        path = String::from(i);
    } else {
        let data = Directories::with_prefix("litecraft", "Litecraft").unwrap();
        path = String::from(data.config_home().to_str().unwrap());
    };

    info!("Using home {}", path);
    assert!(set_current_dir(&path).is_ok());

    client::run(matches.value_of("session").unwrap());
}
