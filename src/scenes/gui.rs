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
use smallvec::SmallVec;

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
        let (mut x, mut y, w, h) = self.get_scale(position, x, y, w, h, scale);

        match *position {
            ContainerPosition::UpLeft => (),
            ContainerPosition::UpCenter => {
                x += (client.get_display().get_width() / 2) as f32;
            },
            ContainerPosition::UpRight => {
                x += client.get_display().get_width() as f32 - w;
            },
            ContainerPosition::MiddleLeft => {
                y += (client.get_display().get_height() / 2) as f32;
            },
            ContainerPosition::MiddleCenter => {
                x += (client.get_display().get_width() / 2) as f32;
                y += (client.get_display().get_height() / 2) as f32;
            },
            ContainerPosition::MiddleRight => {
                x += client.get_display().get_width() as f32 - w;
                y += (client.get_display().get_height() / 2) as f32;
            },
            ContainerPosition::BottomLeft => {
                y += client.get_display().get_height() as f32 - h;
            },
            ContainerPosition::BottonCenter => {
                x += (client.get_display().get_width() / 2) as f32;
                y += client.get_display().get_height() as f32 - h;
            },
            ContainerPosition::BottonRight => {
                x += client.get_display().get_width() as f32 - w;
                y += client.get_display().get_height() as f32 - h;
            },
        };
        (x, y, w, h)
    }

    fn get_scale(&self, position: &ContainerPosition, x: f32, y: f32, w: f32, h: f32, scale: u8) -> (f32, f32, f32, f32) {
        let scale = scale as f32 / 2.0;

        match *position {
            ContainerPosition::UpLeft => (x, y, w * scale, h * scale),
            ContainerPosition::UpCenter => (x - (w * scale / 2.0), y, w * scale, h * scale),
            ContainerPosition::UpRight => (x - (w * scale / 2.0), y, w, h * scale),
            ContainerPosition::MiddleLeft => (x, y - (h * scale / 2.0), w * scale, h * scale),
            ContainerPosition::MiddleCenter => (x - (w * scale / 2.0), y - (h * scale / 2.0), w * scale, h * scale),
            ContainerPosition::MiddleRight => (x, y - (h * scale / 2.0), w * scale, h * scale),
            ContainerPosition::BottomLeft => (x, y - (h * scale / 2.0), w * scale, h),
            ContainerPosition::BottonCenter => (x - (w * scale / 2.0), y - (h * scale / 2.0), w * scale, h),
            ContainerPosition::BottonRight => (x - (w * scale / 2.0), y - (h * scale / 2.0), w, h),
        }
    }

    fn draw(&self, client: &Client);
}

enum ButtonStatus {
    Active,
    Disabled,
    Hover,
    Clicked
}

pub struct Button<'a> {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    position: ContainerPosition,
    status: ButtonStatus,
    text: &'a str
}

impl<'a> Button<'a> {
    pub fn new(x: f32, y: f32, width: f32, position: ContainerPosition, text: &'a str) -> Button {
        Button {
            x,
            y,
            width,
            height: width / 10.0,
            position,
            status: ButtonStatus::Active,
            text,
        }
    }
}

impl<'a> Component for Button<'a> {}

impl<'a> Element for Button<'a> {
    fn draw(&self, client: &Client) {
        let (x, y, w, h) = self.get_position(client, &self.position, self.x, self.y,
                                self.width, self.height, client.scale());

        let texture = client.get_resource_manager().get_texture("gui/widgets");
        let tuple: (f32, f32, f32, f32);

        match self.status {
            ButtonStatus::Clicked | ButtonStatus::Active => { 
                tuple = (
                            0.0,
                            texture.get_height() as f32 * 66.0 / 256.0,
                            texture.get_width() as f32 * 200.0 / 256.0,
                            texture.get_height() as f32 * 86.0 / 256.0,
                        );
            },
            ButtonStatus::Disabled => {
                tuple = (
                            0.0,
                            texture.get_height() as f32 * 46.0 / 256.0,
                            texture.get_width() as f32 * 200.0 / 256.0,
                            texture.get_height() as f32 * 66.0 / 256.0,
                        );
            },
            ButtonStatus::Hover => {
                tuple = (
                            0.0,
                            texture.get_height() as f32 * 86.0 / 256.0,
                            texture.get_width() as f32 * 200.0 / 256.0,
                            texture.get_height() as f32 * 106.0 / 256.0,
                        );
            },
        }

        let (sx, sy, sw, sh) = tuple;

        client.get_core().draw_scaled_bitmap(
            texture,
            sx, sy,                              // source origin
            sw - sx, sh - sy,                    // source dimensions
            x, y,                                // target origin
            w, h,                                // target dimensions
            BitmapDrawingFlags::zero()           // flags
        );

        self.draw_text(client, Color::from_rgb(224, 224, 224),
                    self.text, x + w / 2.0, y  + h / 8.0);
    }
}

// Rectangle
pub struct Rectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    position: ContainerPosition,
    color: Color
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32,
            height: f32, position: ContainerPosition, color: Color) -> Rectangle {
        Rectangle {
            x,
            y,
            width,
            height,
            position,
            color,
        }
    }
}

impl Component for Rectangle {}

impl Element for Rectangle {
    fn draw(&self, client: &Client) {
        let (x, y, w, h) = self.get_position(client, &self.position, self.x, self.y,
                                self.width, self.height, client.scale());

        client.get_primitives().draw_filled_rectangle(x, y, x + w, y + h, self.color);
    }
}

// Image
pub struct Image<'a> {
    texture: &'a str,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    position: ContainerPosition
}

impl<'a> Image<'a> {
    pub fn new(texture: &str, x: f32, y: f32, width: f32,
            height: f32, position: ContainerPosition) -> Image {
        Image {
            texture,
            x,
            y,
            width,
            height,
            position,
        }
    }
}

impl<'a> Component for Image<'a> {}

impl<'a> Element for Image<'a> {
    fn draw(&self, client: &Client) {
        let (x, y, w, h) = self.get_position(client, &self.position, self.x, self.y,
                                self.width, self.height, client.scale());

        self.draw_2d(client, x, y, w, h, &self.texture);
    }
}

pub struct SceneManager<'a> {
    images: SmallVec<[Image<'a>; 8]>,
    buttons: SmallVec<[Button<'a>; 8]>,
    rectangles: SmallVec<[Rectangle; 8]>,
}

impl<'a> SceneManager<'a> {
    pub fn new() -> Self {
        SceneManager{
            images: SmallVec::new(),
            buttons: SmallVec::new(),
            rectangles: SmallVec::new(),
        }
    }

    pub fn add_image(&mut self, e: Image<'a>) {
        self.images.push(e);
    }

    pub fn add_rectangle(&mut self, e: Rectangle) {
        self.rectangles.push(e);
    }

    pub fn add_button(&mut self, e: Button<'a>) {
        self.buttons.push(e);
    }

    pub fn render(&self, client: &Client) {
        for e in &self.rectangles {
            e.draw(client);
        }
        for e in &self.images {
            e.draw(client);
        }
        for e in &self.buttons {
            e.draw(client);
        }
    }
}