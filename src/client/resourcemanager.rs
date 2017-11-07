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

pub struct ResourceManager {
    textures: Vec<Bitmap>,
}

impl ResourceManager {
    pub fn new() -> ResourceManager {
        let manager = ResourceManager { textures: Vec::new() };

        manager
    }

    pub fn load(client: &mut Client) {
        info!("Loading Resource Manager");

        //ResourceManager::load_minecraft_texture(client, "");
    }

    fn load_minecraft_texture(client: &mut Client, name: &str) {
        ResourceManager::load_texture(client, "minecraft", name);
    }

    fn load_texture(client: &mut Client, domain: &str, name: &str) {
        client.resource_manager.textures.push(Bitmap::load(
            &client.core,
            ResourceManager::get_asset_path(domain, "texture", name, "png").as_str(),
        ).unwrap());
    }

    fn get_asset_path(domain: &str, class: &str, path: &str, extension: &str) -> String {
        format!("assets/{}/{}/{}.{}", domain, class, path, extension)
    }
}