/*
   Copyright 2017 Miguel Peláez <kernelfreeze@greenlab.games>
   Copyright 2017 Raúl Salas <raulsalas.martin@greenlab.games>
   
   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at
       http://www.apache.org/licenses/LICENSE-2.0
   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

#![deny(unused_imports)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;
extern crate rand;
extern crate badlog;
extern crate allegro;

mod scenes;
mod client;

use rand::Rng;

fn main() {
    badlog::init_from_env("LOG_LEVEL");

    let matches = clap_app!(litecraft =>
        (version: client::VERSION)
        (author: "Litecraft Team")
        (about: "Open source, clean room implementation of Minecraft Client")
        (@arg session: +required "Sets the user session ID")
        (@arg server: -s ... "Auto-join a server")
        (@arg path: -p ... "Litecraft home, must have all resources available")
    ).get_matches();

    // Epic hardcoded quotes!
    let hello = [
        // Litecrafty
        "Litecraft is love, Litecraft is life!",
        "Now available on bluray",
        "Very fast!",
        "Sky is the Limit (y = 255)",
        "Open Source!",
        "Less bugs!",

        // Nice...
        "Knowledge is having the right answer. Intelligence is asking the right question",
        "Wake me up when it's all over",
        "A person who never made a mistake never tried anything new",
        "There is nothing permanent except change",
        "If you cannot do great things, do small things in a great way",
        "The journey of a thousand miles begins with one step",

        // Random stuff
        "Citrate Caffeine 1 oz\nExtract Vanilla 1 oz\nFlavouring 2.5 oz",
        "Triskaidekaphobic, 13"
    ];

    println!(
        r"  _    _ _                    __ _   
 | |  (_) |_ ___ __ _ _ __ _ / _| |_ 
 | |__| |  _/ -_) _| '_/ _` |  _|  _|
 |____|_|\__\___\__|_| \__,_|_|  \__|
                                     "
    );
    println!("{}\n", rand::thread_rng().choose(&hello).unwrap());
    println!(
        "{} for Minecraft Modern {}",
        client::VERSION,
        client::MINECRAFT
    );
    info!("Starting engine...");

    client::run(matches.value_of("session").unwrap());
}
