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

use core::constants::CONFIG_FILE;
use core::settings::Settings;
use serde_yaml;
use std::fs::File;
use std::path::Path;

/// Load and parse yaml configuarion file
pub fn load_config() -> Settings {
    use std::fs::copy;

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

/// Generate a new configuration file using defaults
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
