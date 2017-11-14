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
use scenes::gui::SceneManager;

use allegro::Color;

pub struct MainMenu<'a> {
    scenemanager: SceneManager<'a>,
}

impl<'a> Component for MainMenu<'a> {}

impl<'a> Scene for MainMenu<'a> {
    fn draw(&self, client: &mut Client) -> Option<Box<Scene>> {
        client.get_core().clear_to_color(
            Color::from_rgb_f(1.0, 1.0, 1.0),
        );

        let w = client.get_display().get_width() as f32;
        let h = client.get_display().get_height() as f32;

        match client.get_timer().get_count() / 1000 % 9 {
            0 => self.draw_2d(client, 0.0, 0.0, w, h, "menu_1"),
            1 => self.draw_2d(client, 0.0, 0.0, w, h, "menu_2"),
            2 => self.draw_2d(client, 0.0, 0.0, w, h, "menu_3"),
            4 => self.draw_2d(client, 0.0, 0.0, w, h, "menu_4"),
            5 => self.draw_2d(client, 0.0, 0.0, w, h, "menu_5"),
            6 => self.draw_2d(client, 0.0, 0.0, w, h, "menu_6"),
            7 => self.draw_2d(client, 0.0, 0.0, w, h, "menu_7"),
            8 => self.draw_2d(client, 0.0, 0.0, w, h, "menu_8"),
            _ => self.draw_2d(client, 0.0, 0.0, w, h, "menu_9"),
        }

        None
    }
}

impl<'a> MainMenu<'a> {
    pub fn new() -> Self {
        MainMenu { scenemanager: SceneManager::new() }
    }
}