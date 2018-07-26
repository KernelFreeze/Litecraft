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

use glium::glutin::dpi::LogicalSize;
use glium::{Display, DrawParameters};
use serde_yaml;
use threadpool::ThreadPool;

use core::constants::CONFIG_FILE;
use core::resource_manager::resource::Resource;
use core::resource_manager::shader_manager::ShaderManager;
use core::resource_manager::texture_manager::TextureManager;
use core::settings::Settings;

use std::fs::File;
use std::path::Path;
use std::sync::Mutex;
use std::time::Instant;

lazy_static! {
    pub static ref SETTINGS: Mutex<Settings> = Mutex::new(load_config());
    static ref STARTED: Instant = Instant::now();
}

fn load_config() -> Settings {
    use std::fs::copy;

    match File::open(CONFIG_FILE) {
        Err(why) => {
            warn!("Can't read configuration file: {}", why);
            generate_config()
        },
        Ok(file) => match serde_yaml::from_reader(file) {
            Err(why) => {
                warn!("Can't parse configuration file: {}", why);
                warn!("Regenerating, old configuration placed at {}.bak", CONFIG_FILE);

                if let Err(error) = copy(CONFIG_FILE, format!("{}.bak", CONFIG_FILE)) {
                    warn!("Failed to copy old configuration to .bak file. {}", error);
                }

                generate_config()
            },
            Ok(settings) => settings,
        },
    }
}

fn generate_config() -> Settings {
    use std::io::prelude::*;

    let config = Settings::new();
    let path = Path::new(CONFIG_FILE);

    let serialized = serde_yaml::to_string(&config).expect("Couldn't serialize configuration");

    File::create(&path)
        .expect("Couldn't create configuration file")
        .write_all(serialized.as_bytes())
        .expect("Couldn't write to configuration file");

    config
}

pub struct ResourceManager {
    pool: ThreadPool,
    window_size: LogicalSize,

    texture_manager: TextureManager,
    shader_manager: ShaderManager,
}

impl ResourceManager {
    /// Create Litecraft's resource manager
    pub fn new() -> ResourceManager {
        let settings = SETTINGS.lock().expect("Could not lock mutex");
        let pool = ThreadPool::new(settings.loader_threads());
        let window_size = LogicalSize::new(settings.width().into(), settings.height().into());

        ResourceManager {
            pool,
            window_size,
            texture_manager: TextureManager::new(),
            shader_manager: ShaderManager::new(),
        }
    }

    /// Get time since application start
    pub fn time() -> f32 {
        let dur = STARTED.elapsed();
        dur.as_secs() as f32 + dur.subsec_nanos() as f32 / 1_000_000_000.0
    }

    /// Tick all resource managers
    pub fn tick(&mut self, display: &Display) { self.texture_manager.tick(display); }

    /// Get window size
    pub fn size(&self) -> LogicalSize { self.window_size }

    /// Get window size
    pub fn set_size(&mut self, value: LogicalSize) { self.window_size = value }

    /// Get window width
    pub fn width(&self) -> u32 { self.window_size.width as u32 }

    /// Get window height
    pub fn height(&self) -> u32 { self.window_size.height as u32 }

    /// Set window width
    pub fn set_width(&mut self, value: u32) { self.window_size.width = value.into() }

    /// Set window height
    pub fn set_height(&mut self, value: u32) { self.window_size.height = value.into() }

    /// Get texture manager
    pub fn textures(&self) -> &TextureManager { &self.texture_manager }

    /// Get shader manager
    pub fn shaders(&self) -> &ShaderManager { &self.shader_manager }

    /// Load texture using a local asset
    pub fn load_texture(&mut self, name: Resource) { self.texture_manager.load(name, &self.pool); }

    /// Load shader using a local asset
    pub fn load_shader(&mut self, name: &'static str, display: &Display) {
        self.shader_manager.load(name, display);
    }

    /// Parameters to draw almost any shape
    pub fn parameters(&self) -> DrawParameters {
        use glium::draw_parameters::{Blend, DepthTest};
        use glium::Depth;

        DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            blend: Blend::alpha_blending(),
            ..Default::default()
        }
    }

    /// Parameters to draw shapes without depth
    pub fn no_depth(&self) -> DrawParameters {
        use glium::draw_parameters::Blend;

        DrawParameters {
            blend: Blend::alpha_blending(),
            ..Default::default()
        }
    }
}
