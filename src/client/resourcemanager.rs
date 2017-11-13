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
use client::allegro_font::Font;
use client::allegro_font::*;
use client::allegro_ttf::{TtfAddon, TtfFlags};
use allegro::Flag;

trait Loadable {
    fn load(&self, client: &mut Client);
}

struct LitecraftTexture {
    name: &'static str,
}

impl Loadable for LitecraftTexture {
    fn load(&self, client: &mut Client) {
        ResourceManager::load_litecraft_texture(client, self.name);
    }
}

impl LitecraftTexture {
    fn new(name: &'static str) -> Self {
        LitecraftTexture { name }
    }
}

struct MinecraftTexture {
    name: &'static str,
}

impl Loadable for MinecraftTexture {
    fn load(&self, client: &mut Client) {
        ResourceManager::load_minecraft_texture(client, self.name);
    }
}

impl MinecraftTexture {
    fn new(name: &'static str) -> Self {
        MinecraftTexture { name }
    }
}

pub struct ResourceManager<'a> {
    textures: HashMap<&'a str, Bitmap>,
    dynamic_textures: HashMap<&'static str, Bitmap>,
    minecraft_font: Option<Font>,
    litecraft_font: Option<Font>,
    load_queue: Vec<Box<Loadable>>,
}

impl<'a> ResourceManager<'a> {
    pub fn new() -> ResourceManager<'a> {
        let q: Vec<Box<Loadable>> = vec![
            Box::new(LitecraftTexture::new("menu_1")),
            Box::new(LitecraftTexture::new("menu_2")),
            Box::new(LitecraftTexture::new("menu_3")),
            Box::new(LitecraftTexture::new("menu_4")),
        ];
        ResourceManager {
            textures: HashMap::new(),
            dynamic_textures: HashMap::new(),
            minecraft_font: None,
            litecraft_font: None,
            load_queue: q,
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

    pub fn get_texture(&self, name: &str) -> &Bitmap {
        self.textures.get(name).unwrap()
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

        ResourceManager::load_litecraft_texture(client, "logo");
        ResourceManager::load_litecraft_texture(client, "background");

        // Set our awesome logo ;3
        let logo = client.resource_manager.get_texture("logo");
        client.display.set_icon(logo);
    }

    pub fn load_assets(client: &mut Client) -> bool {
        match client.resource_manager.load_queue.pop() {
            Some(loadable) => {
                loadable.load(client);
                true
            }
            None => false,
        }
    }

    fn load_minecraft_texture(client: &mut Client, name: &'static str) {
        ResourceManager::load_texture(client, "minecraft", name);
    }

    fn load_litecraft_texture(client: &mut Client, name: &'static str) {
        ResourceManager::load_texture(client, "litecraft", name);
    }

    fn load_texture(client: &mut Client, domain: &str, name: &'static str) {
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