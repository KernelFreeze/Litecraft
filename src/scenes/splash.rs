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

use allegro::Color;

pub struct SplashScreen;

impl Component for SplashScreen {

}

impl Scene for SplashScreen {
    fn draw(&self, client: &Client) {
        client.core.clear_to_color(Color::from_rgb_f(1.0, 1.0, 1.0));
        /*client.core.draw_text(
            &client.font,
            Color::from_rgb_f(0.5, 0.5, 0.5),
            (client.display.get_width() / 2) as f32,
            (client.display.get_height() / 2) as f32,
            FontAlign::Centre,
            "Potato!",
        );*/
        self.draw_centered(client, "logo", 200, 200);
    }
}

impl SplashScreen {
    pub fn new() -> Self {
        info!("Starting splash screen...");
        SplashScreen {}
    }
}