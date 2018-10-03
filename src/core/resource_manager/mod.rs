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
use core::resource_manager::shader_manager::ShaderManager;
use core::resource_manager::texture_manager::TextureManager;

use core::settings::Settings;

use gfx::shapes::Shapes;

use glium::Display;

use conrod::text::Font;
use smallvec::SmallVec;

use std::error::Error;
use std::sync::Mutex;
use std::time::Instant;

pub mod resource;
pub mod resource_type;
pub mod shader_manager;
pub mod texture_manager;

lazy_static! {
    /// Time since client start
    static ref STARTED: Instant = Instant::now();

    /// Enabled resource packs
    static ref RESOURCE_PACKS: Mutex<SmallVec<[String; 4]>> = Mutex::new(SmallVec::new());
}

/// Assets and resources manager
pub struct ResourceManager {
    texture_manager: TextureManager,
    shader_manager: ShaderManager,
    shapes: Shapes,
}

impl ResourceManager {
    /// Create Litecraft's resource manager
    pub fn new(display: &Display, settings: &Settings) -> ResourceManager {
        let mut resourcepacks = RESOURCE_PACKS
            .lock()
            .expect("Failed to lock resourcepacks manager!");

        // Get enabled resourcepacks from config
        *resourcepacks = SmallVec::from_vec(settings.resourcepacks().clone());

        // Bind vertex data
        let shapes = Shapes::new(display).expect("Failed to create required vertex data");

        ResourceManager {
            shapes,
            texture_manager: TextureManager::new(),
            shader_manager: ShaderManager::new(),
        }
    }

    /// Get time since application start
    pub fn time() -> f32 {
        let dur = STARTED.elapsed();
        dur.as_secs() as f32 + dur.subsec_nanos() as f32 / 1_000_000_000.0
    }

    /// Get enabled resourcepacks
    pub fn resourcepacks() -> SmallVec<[String; 4]> {
        RESOURCE_PACKS
            .lock()
            .expect("Failed to lock resourcepacks manager!")
            .clone()
    }

    /// Get and load a font file
    pub fn font(resource: &Resource) -> Result<Font, Box<Error>> {
        use conrod::text::FontCollection;

        info!("Loading font file '{}'", resource);

        let data = resource.load_binary()?;
        let collection = FontCollection::from_bytes(data)?;

        collection.into_font().or_else(|_| {
            use std::io::{Error, ErrorKind};

            Err(From::from(Error::new(
                ErrorKind::Other,
                "Failed to decode font file",
            )))
        })
    }

    /// Tick all resource managers
    #[inline]
    pub fn tick(&mut self, display: &Display) { self.texture_manager.tick(display); }

    /// Get vertex data
    #[inline]
    pub fn shapes(&self) -> &Shapes { &self.shapes }

    /// Get texture manager
    #[inline]
    pub fn textures(&self) -> &TextureManager { &self.texture_manager }

    /// Get shader manager
    #[inline]
    pub fn shaders(&self) -> &ShaderManager { &self.shader_manager }

    /// Get texture manager
    #[inline]
    pub fn textures_mut(&mut self) -> &mut TextureManager { &mut self.texture_manager }

    /// Get shader manager
    #[inline]
    pub fn shaders_mut(&mut self) -> &mut ShaderManager { &mut self.shader_manager }

    /// Check if resource manager is loaded
    #[inline]
    pub fn loaded(&self) -> bool { self.texture_manager.loaded() }
}
