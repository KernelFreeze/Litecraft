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

use allegro::bitmap::Bitmap;
use client::Client;
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TextureType {
    // Add all known textures here and in fmt::Display
    Logo,
}

impl fmt::Display for TextureType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Logo => return write!(f, "logo"),
        }
        write!(f, "unknown")
    }
}

pub struct ResourceManager {
    textures: HashMap<TextureType, Bitmap>,
    dynamic_textures: HashMap<&'static str, Bitmap>,
}

impl ResourceManager {
    pub fn new() -> ResourceManager {
        let manager = ResourceManager { textures: HashMap::new(), dynamic_textures: HashMap::new() };

        manager
    }

    pub fn get_dynamic_texture(&self, name: &str) -> &Bitmap {
        self.dynamic_textures.get(name).unwrap()
    }

    pub fn get_texture(&self, name: TextureType) -> &Bitmap {
        self.textures.get(&name).unwrap()
    }

    pub fn load(client: &mut Client) {
        info!("Loading Resource Manager");

        ResourceManager::load_litecraft_texture(client, TextureType::Logo); 
    }

    fn load_minecraft_texture(client: &mut Client, name: TextureType) {
        ResourceManager::load_texture(client, "minecraft", name);
    }

    fn load_litecraft_texture(client: &mut Client, name: TextureType) {
        ResourceManager::load_texture(client, "litecraft", name);
    }

    fn load_texture(client: &mut Client, domain: &str, name: TextureType) {
        info!("Loading {} texture '{}'", domain, name);

        let bmp = Bitmap::load(
            &client.core,
            ResourceManager::get_asset(domain, "textures", &name.to_string()[..], "png").as_str(),
        );

        match bmp {
            Ok(texture) => client.resource_manager.textures.insert(name, texture),
            Err(error) => {
                error!("I can't load texture '{}'. Error: {:?}", name, error);
                panic!("Fatal error. See log for details.");
            }
        };
    }

    fn get_asset_path(domain: &str, class: &str, path: &str, extension: &str) -> PathBuf {
        let path = PathBuf::from(format!(
            "./assets/{}/{}/{}.{}",
            domain,
            class,
            path,
            extension
        ));
        fs::canonicalize(path).unwrap()
    }

    fn get_asset(domain: &str, class: &str, path: &str, extension: &str) -> String {
        ResourceManager::get_asset_path(domain, class, path, extension)
            .into_os_string()
            .into_string()
            .unwrap()
    }
}