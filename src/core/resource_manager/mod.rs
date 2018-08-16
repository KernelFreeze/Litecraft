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

pub mod resource;
pub mod resource_type;
pub mod shader_manager;
pub mod texture_manager;

use glium::Display;

use core::resource_manager::shader_manager::ShaderManager;
use core::resource_manager::texture_manager::TextureManager;
use core::settings::Settings;

use gfx::shapes::Shapes;

use std::time::Instant;

lazy_static! {
    /// Time since client start
    static ref STARTED: Instant = Instant::now();
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
        ResourceManager {
            shapes: Shapes::new(display),
            texture_manager: TextureManager::new(settings),
            shader_manager: ShaderManager::new(settings),
        }
    }

    /// Get time since application start
    pub fn time() -> f32 {
        let dur = STARTED.elapsed();
        dur.as_secs() as f32 + dur.subsec_nanos() as f32 / 1_000_000_000.0
    }

    /// Tick all resource managers
    pub fn tick(&mut self, display: &Display) { self.texture_manager.tick(display); }

    /// Get VAOs and VBOs
    pub fn shapes(&self) -> &Shapes { &self.shapes }

    /// Get texture manager
    pub fn textures(&self) -> &TextureManager { &self.texture_manager }

    /// Get shader manager
    pub fn shaders(&self) -> &ShaderManager { &self.shader_manager }

    /// Get texture manager
    pub fn textures_mut(&mut self) -> &mut TextureManager { &mut self.texture_manager }

    /// Get shader manager
    pub fn shaders_mut(&mut self) -> &mut ShaderManager { &mut self.shader_manager }
}
