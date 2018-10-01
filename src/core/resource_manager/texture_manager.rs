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

use glium::texture::{CompressedSrgbTexture2d, RawImage2d};
use glium::Display;

use std::collections::HashMap;
use std::io::Cursor;
use std::sync::mpsc::{channel, Receiver, Sender};

use conrod::image::{Id, Map};
use threadpool::ThreadPool;

use image::{self, ImageBuffer};

pub type UiTexture = (Id, (f64, f64));

/// RGBA image loaded async
struct RGBAImageData {
    resource: Resource,
    data: Vec<u8>,
    dimensions: (u32, u32),
    ui: bool,
}

/// Texture tracker and loader
pub struct TextureManager {
    pending: u16,
    ui_images: Map<CompressedSrgbTexture2d>,
    ui_textures: HashMap<Resource, UiTexture>,
    textures: HashMap<Resource, CompressedSrgbTexture2d>,
    sender: Sender<RGBAImageData>,
    receiver: Receiver<RGBAImageData>,
    pool: ThreadPool,
}

impl TextureManager {
    /// Start texture manager
    pub fn new() -> TextureManager {
        info!("Starting texture manager...");

        let (sender, receiver) = channel();

        TextureManager {
            textures: HashMap::new(),
            ui_textures: HashMap::new(),

            pool: ThreadPool::new(8),

            ui_images: Map::<CompressedSrgbTexture2d>::new(),

            pending: 0,

            sender,
            receiver,
        }
    }

    // Check if we need to load another texture
    pub fn loaded(&self) -> bool { self.pending == 0 }

    /// Get a texture
    pub fn get(&self, name: &Resource) -> Option<&CompressedSrgbTexture2d> { self.textures.get(name) }

    /// Get a UI texture
    pub fn get_ui(&self, name: &Resource) -> Option<UiTexture> { self.ui_textures.get(name).cloned() }

    /// Request texture load
    pub fn load(&mut self, resource: Resource) { self.do_load(resource, false); }

    /// Request texture load for use in user interface
    pub fn load_ui(&mut self, resource: Resource) { self.do_load(resource, true); }

    /// Upload pending textures to OpenGL
    pub fn tick(&mut self, display: &Display) {
        if let Ok(image) = self.receiver.try_recv() {
            debug!("Uploading texture {} to GPU", &image.resource);

            // Parse texture from raw data
            let texture = RawImage2d::from_raw_rgba(image.data, image.dimensions);
            let texture = CompressedSrgbTexture2d::new(display, texture);
            let texture = texture.expect("Failed to send texture to GPU.");

            // Check if texture is needed for 3D or for user interface
            if image.ui {
                debug!("Loaded UI texture {}", &image.resource);

                // Add size and id to texture map
                self.ui_textures.insert(
                    image.resource,
                    (
                        // Get conrod texture Id
                        self.ui_images.insert(texture),
                        // Get image size
                        (f64::from(image.dimensions.0), f64::from(image.dimensions.1)),
                    ),
                );
            } else {
                debug!("Loaded texture {}", &image.resource);

                // Add to texture map
                self.textures.insert(image.resource, texture);
            }

            self.pending -= 1;
        }
    }

    /// Load texture async
    fn do_load(&mut self, resource: Resource, ui: bool) {
        // Prevent load twice a texture
        if !ui && self.get(&resource).is_some() {
            warn!("Texture {} is already loaded!", resource);
            return;
        }

        // Prevent load twice an UI texture
        if ui && self.get_ui(&resource).is_some() {
            warn!("UI texture {} is already loaded!", resource);
            return;
        }

        let sender = self.sender.clone();

        self.pending += 1;

        // Load image in other thread
        self.pool.execute(move || {
            info!("Loading texture '{}'.", resource);

            /// Get failback texture
            fn failback_texture() -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
                ImageBuffer::from_fn(16, 16, |x, y| {
                    if x % 2 == y % 2 {
                        image::Rgba([0u8, 0, 0, 255])
                    } else {
                        image::Rgba([158u8, 0, 123, 255])
                    }
                })
            }

            // Try to load and decode texture or use failback
            let image = if let Ok(data) = resource.load_binary() {
                // Decode image data
                if let Ok(data) = image::load(Cursor::new(data), image::PNG) {
                    data.to_rgba()
                } else {
                    info!("Failed to decode texture '{}'. Using failback", resource);

                    failback_texture()
                }
            } else {
                info!("Failed to load texture '{}'. Using failback", resource);

                failback_texture()
            };

            let dimensions = image.dimensions();

            // Reverse texture
            let data = image
                .into_raw()
                .chunks(dimensions.0 as usize * 4)
                .rev()
                .flat_map(|row| row.iter())
                .cloned()
                .collect();

            sender
                .send(RGBAImageData {
                    data,
                    dimensions,
                    resource,
                    ui,
                })
                .expect("Failed to send decoded texture to main thread");
        });
    }

    /// Get user interface manager
    pub fn image_map(&self) -> &Map<CompressedSrgbTexture2d> { &self.ui_images }
}
