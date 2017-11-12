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
use client::allegro_font::Font;
use client::allegro_font::*;
use client::allegro_ttf::{TtfAddon, TtfFlags};
use allegro::Flag;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TextureType {
    // Add all known textures here and in fmt::Display
    Logo,
    SplashBackground,
}

impl fmt::Display for TextureType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Add texture file names here
            TextureType::Logo => write!(f, "logo"),
            TextureType::SplashBackground => write!(f, "background"),
        }
    }
}

pub struct ResourceManager {
    textures: HashMap<TextureType, Bitmap>,
    dynamic_textures: HashMap<&'static str, Bitmap>,
    minecraft_font: Option<Font>,
    litecraft_font: Option<Font>,
    load_queue: VecDeque<Box<Fn()>>,
}

impl ResourceManager {
    pub fn new() -> ResourceManager {
        ResourceManager {
            textures: HashMap::new(),
            dynamic_textures: HashMap::new(),
            minecraft_font: None,
            litecraft_font: None,
            load_queue: VecDeque::new(),
        }
    }

    pub fn get_minecraft_font(&self) -> &Font {
        match self.minecraft_font {
            Some(ref font) => font,
            None => panic!("I don't have a font!"),
        }
    }

    pub fn get_litecraft_font(&self) -> &Font {
        match self.litecraft_font {
            Some(ref font) => font,
            None => panic!("I don't have a font!"),
        }
    }

    pub fn get_dynamic_texture(&self, name: &str) -> &Bitmap {
        self.dynamic_textures.get(name).unwrap()
    }

    pub fn get_texture(&self, name: &TextureType) -> &Bitmap {
        self.textures.get(&name).unwrap()
    }

    pub fn load(client: &mut Client) {
        info!("Loading Resource Manager");


        let font_addon = FontAddon::init(&client.core).unwrap();
        let ttf_addon = TtfAddon::init(&font_addon).unwrap();

        info!("Loading default font");
        let font = ttf_addon
            .load_ttf_font(
                ResourceManager::get_asset("litecraft", "fonts", String::from("minecraft"), "ttf")
                    .as_str(),
                16,
                TtfFlags::zero(),
            )
            .unwrap();
        client.resource_manager.minecraft_font = Some(font);
        info!("Loading litecraft font");
        let font = ttf_addon
            .load_ttf_font(
                ResourceManager::get_asset("litecraft", "fonts", String::from("litecraft"), "ttf")
                    .as_str(),
                22,
                TtfFlags::zero(),
            )
            .unwrap();
        client.resource_manager.litecraft_font = Some(font);

        ResourceManager::load_litecraft_texture(client, TextureType::Logo);
        ResourceManager::load_litecraft_texture(client, TextureType::SplashBackground);

        // Set our awesome logo ;3
        let logo = client.resource_manager.get_texture(&TextureType::Logo);
        client.display.set_icon(logo);

        //self.load_queue.push_back
    }

    pub fn load_assets(&mut self) -> bool {
        match self.load_queue.pop_front() {
            Some(load) => {
                load();
                true
            }
            None => false,
        }
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
            ResourceManager::get_asset(domain, "textures", name.to_string(), "png").as_str(),
        );

        match bmp {
            Ok(texture) => client.resource_manager.textures.insert(name, texture),
            Err(error) => {
                error!("I can't load texture '{}'. Error: {:?}", name, error);
                panic!("Fatal error. See log for details.");
            }
        };
    }

    fn get_asset_path(domain: &str, class: &str, path: String, extension: &str) -> PathBuf {
        let path = PathBuf::from(format!(
            "./assets/{}/{}/{}.{}",
            domain,
            class,
            path,
            extension
        ));
        fs::canonicalize(path).unwrap()
    }

    fn get_asset(domain: &str, class: &str, path: String, extension: &str) -> String {
        ResourceManager::get_asset_path(domain, class, path, extension)
            .into_os_string()
            .into_string()
            .unwrap()
    }
}