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

pub trait Component {
    fn draw_2d(&self, client: &Client, x: f32, y: f32, w: f32, h: f32, texture: &str) {
        let texture = client.get_resource_manager().get_texture(texture);

        client.get_core().draw_scaled_bitmap(
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
        client.get_core().draw_text(
            &client.get_resource_manager().get_minecraft_font(),
            color,
            x,
            y,
            FontAlign::Centre,
            text,
        );
    }

    fn draw_litecraft_text(&self, client: &Client, color: Color, text: &str, x: f32, y: f32) {
        client.get_core().draw_text(
            &client.get_resource_manager().get_litecraft_font(),
            color,
            x,
            y,
            FontAlign::Centre,
            text,
        );
    }
}

#[derive(Debug)]
pub enum ContainerPosition {
    UpLeft,
    UpCenter,
    UpRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    BottomLeft,
    BottonCenter,
    BottonRight
}

pub trait Element : Component {
    fn get_position(&self, client: &Client, position: &ContainerPosition, x: f32, y: f32, w: f32, h: f32, scale: u8) -> (f32, f32, f32, f32) {
        let mut x = x;
        let mut y = y;

        match *position {
            ContainerPosition::UpLeft => (),
            ContainerPosition::UpCenter => {
                x += (client.get_display().get_width() / 2 - (w as i32 / 2)) as f32;
            },
            ContainerPosition::UpRight => {
                x += client.get_display().get_width() as f32 - w;
            },
            ContainerPosition::MiddleLeft => {
                y += (client.get_display().get_height() / 2 - (h as i32 / 2)) as f32;
            },
            ContainerPosition::MiddleCenter => {
                x += (client.get_display().get_width() / 2 - (w as i32 / 2)) as f32;
                y += (client.get_display().get_height() / 2 - (h as i32 / 2)) as f32;
            },
            ContainerPosition::MiddleRight => {
                x += client.get_display().get_width() as f32 - w;
                y += (client.get_display().get_height() / 2 - (h as i32 / 2)) as f32;
            },
            ContainerPosition::BottomLeft => {
                y += client.get_display().get_height() as f32 - h;
            },
            ContainerPosition::BottonCenter => {
                x += (client.get_display().get_width() / 2 - (w as i32 / 2)) as f32;
                y += client.get_display().get_height() as f32 - h;
            },
            ContainerPosition::BottonRight => {
                x += client.get_display().get_width() as f32 - w;
                y += client.get_display().get_height() as f32 - h;
            },
        };
        self.get_scale(position, x, y, w, h, scale)
    }

    fn get_scale(&self, position: &ContainerPosition, x: f32, y: f32, w: f32, h: f32, scale: u8) -> (f32, f32, f32, f32) {
        let scale = scale as f32 * 100f32;

        match *position {
            ContainerPosition::UpLeft => (x, y, w + scale, h + scale),
            ContainerPosition::UpCenter => (x + scale, y, w + scale, h + scale),
            ContainerPosition::UpRight => (x + scale, y, w, h + scale),
            ContainerPosition::MiddleLeft => (x, y + scale, w + scale, h + scale),
            ContainerPosition::MiddleCenter => (x + scale, y + scale, w + scale, h + scale),
            ContainerPosition::MiddleRight => (x, y + scale, w + scale, h + scale),
            ContainerPosition::BottomLeft => (x, y + scale, w + scale, h),
            ContainerPosition::BottonCenter => (x + scale, y + scale, w + scale, h),
            ContainerPosition::BottonRight => (x + scale, y + scale, w, h),
        }
    }

    fn draw(&self, client: &Client);
}

#[derive(Debug)]
pub struct Button<'a> {
    texture: &'a str,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    position: ContainerPosition
}

impl<'a> Button<'a> {
    fn new(texture: &str, x: f32, y: f32, height: f32,
            width: f32, position: ContainerPosition) -> Button {
        Button {
            texture,
            x,
            y,
            width,
            height,
            position,
        }
    }
}

impl<'a> Component for Button<'a> {}

impl<'a> Element for Button<'a> {
    fn draw(&self, client: &Client) {
        let (x, y, w, h) = self.get_position(client, &self.position, self.x, self.y,
                                self.width, self.height, client.scale());

        self.draw_2d(client, x, y, w, h, &self.texture);
    }
}

pub struct SceneManager<'a> {
    elements: Vec<&'a Element>
}

impl<'a> SceneManager<'a> {
    pub fn new() -> Self {
        SceneManager{elements: Vec::new()}
    }
}