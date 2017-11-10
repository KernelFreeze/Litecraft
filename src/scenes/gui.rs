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

use allegro::core::BitmapDrawingFlags;
use allegro::{Flag, Color};
use client::Client;
use allegro::bitmap_like::BitmapLike;
use client::allegro_font::{FontDrawing, FontAlign};
use client::resourcemanager::TextureType;

pub trait Component {
    fn draw_centered(&self, client: &Client, name: &str, w: i32, h: i32) {
        let x = (client.display.get_width() / 2 - (w / 2)) as f32;
        let y = (client.display.get_height() / 2 - (h / 2)) as f32;

        self.draw_2d(client, name, x, y, w, h);
    }

    fn draw_2d(&self, client: &Client, name: &str,  x: f32, y: f32, w: i32, h: i32) {
        let w = w as f32;
        let h = h as f32;

        let texture = client.resource_manager.get_texture(TextureType::Logo);

        client.core.draw_scaled_bitmap(
            texture,
            0f32, 0f32,                              // source origin
            texture.get_width() as f32,              // source width
            texture.get_height() as f32,             // source height
            x, y,                                    // target origin
            w, h,                                    // target dimensions
            BitmapDrawingFlags::zero()               // flags
        );
    }

    fn draw_text(&self, client: &Client, color: Color, text: &str, x: f32, y: f32) {
        client.core.draw_text(
            &client.font,
            color,
            x,
            y,
            FontAlign::Centre,
            text,
        );
    }
}