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
            Color::from_rgb_f(0.0, 0.0, 0.0),
        );

        None
    }
}

impl<'a> MainMenu<'a> {
    pub fn new() -> Self {
        MainMenu { scenemanager: SceneManager::new() }
    }
}