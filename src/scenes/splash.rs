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

use client::Client;
use scenes::scene::Scene;
use scenes::gui::Component;
use client::resourcemanager::TextureType;
use scenes::mainmenu::MainMenu;

use allegro::Color;

pub struct SplashScreen;

impl Component for SplashScreen {}

impl Scene for SplashScreen {
    fn draw(&self, client: &mut Client) -> Option<Box<Scene>> {
        client.get_core().clear_to_color(Color::from_rgb_f(0.2, 0.41, 0.62));

        let x = (client.get_display().get_width() / 2) as f32;
        let y = (client.get_display().get_height() / 2) as f32;
        let color = Color::from_rgb_f(1f32, 1f32, 1f32);

        self.draw_2d(client, x - 100.0, y - 130.0, 200.0, 200.0, &TextureType::Logo);
        self.draw_litecraft_text(client, color, "Starting Litecraft...", x, y + 100.0);

        if !client.get_resource_manager_mut().load_assets() {            
            return Some(Box::new(MainMenu::new()));
        }
        None
    }
}

impl SplashScreen {
    pub fn new() -> Self {
        info!("Starting splash screen...");
        SplashScreen {}
    }
}