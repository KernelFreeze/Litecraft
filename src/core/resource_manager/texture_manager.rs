// The MIT License (MIT)
// Copyright © 2014-2018 Miguel Peláez <kernelfreeze@outlook.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation
// files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy,
// modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
// OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use core::resource_manager::resource::Resource;
use core::settings::Settings;
use threadpool::ThreadPool;

use image;

use glium::texture::{CompressedSrgbTexture2d, RawImage2d};
use glium::uniforms::Sampler;
use glium::Display;

use std::collections::HashMap;
use std::io::Cursor;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;

pub struct RGBAImageData {
    pub resource: Resource,
    pub data: Vec<u8>,
    pub dimensions: (u32, u32),
}

pub struct TextureManager {
    pending: u16,
    textures: HashMap<Resource, CompressedSrgbTexture2d>,
    sender: Sender<RGBAImageData>,
    receiver: Receiver<RGBAImageData>,
}

impl TextureManager {
    /// Start texture manager
    pub fn new() -> TextureManager {
        info!("Starting texture manager...");

        let (sender, receiver) = channel();

        TextureManager {
            pending: 0,
            textures: HashMap::new(),
            sender,
            receiver,
        }
    }

    /// Upload pending textures to OpenGL
    pub fn tick(&mut self, display: &Display) {
        if let Ok(image) = self.receiver.try_recv() {
            let texture = RawImage2d::from_raw_rgba(image.data, image.dimensions);
            let texture = CompressedSrgbTexture2d::new(display, texture);
            let texture = texture.expect(&format!("Failed to send texture to GPU."));

            debug!("Loaded texture {}", &image.resource);

            self.textures.insert(image.resource, texture);
            self.pending -= 1;
        }
    }

    /// Load texture async
    pub fn load(&mut self, resource: Resource, settings: Arc<Settings>, pool: &ThreadPool) {
        if let Some(_) = self.get(&resource) {
            warn!("Texture {} is already loaded!", resource);
            return;
        }

        let sender = self.sender.clone();

        self.pending += 1;

        pool.execute(move || {
            let data = resource.load_binary(settings);
            let image = image::load(Cursor::new(data), image::PNG).expect("Failed to decode a texture");
            let image = image.to_rgba();

            let dimensions = image.dimensions();
            let data = image.into_raw();

            // Reverse texture
            let data = data
                .chunks(dimensions.0 as usize * 4)
                .rev()
                .flat_map(|row| row.iter())
                .map(|p| p.clone())
                .collect();

            sender
                .send(RGBAImageData {
                    data,
                    dimensions,
                    resource,
                })
                .expect("Failed to send decoded texture to main thread");
        });
    }

    // Check if we need to load another texture
    pub fn loaded(&self) -> bool { self.pending <= 0 }

    /// Get a texture
    pub fn get(&self, name: &Resource) -> Option<&CompressedSrgbTexture2d> { self.textures.get(name) }
}
