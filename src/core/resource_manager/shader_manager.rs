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
use core::resource_manager::resource_type::ResourceType;
use core::settings::Settings;

use glium::{Display, Program};

use std::collections::HashMap;

pub struct ShaderManager {
    shaders: HashMap<&'static str, Program>,
}

impl ShaderManager {
    /// Start shader manager
    pub fn new() -> ShaderManager {
        info!("Starting shader manager...");

        ShaderManager {
            shaders: HashMap::new(),
        }
    }

    /// Load and build a shader
    pub fn load(&mut self, name: &'static str, settings: &Settings, display: &Display) {
        if self.get(name).is_some() {
            warn!("Shader '{}' is already loaded!", name);
            return;
        }

        let v_140 = Resource::litecraft_path(name, "140", ResourceType::VertexShader);
        let f_140 = Resource::litecraft_path(name, "140", ResourceType::FragmentShader);

        let v_100 = Resource::litecraft_path(name, "100", ResourceType::VertexShader);
        let f_100 = Resource::litecraft_path(name, "100", ResourceType::FragmentShader);

        let program =
            program!(display,
        140 => {
            vertex: &v_140.load(&settings),
            fragment: &f_140.load(&settings)
        },

        100 => {
            vertex: &v_100.load(&settings),
            fragment: &f_100.load(&settings)
        }).expect("Failed to build a required shader program. Do you have updated GPU drivers?");

        self.shaders.insert(name, program);
    }

    /// Get a compiled shader program
    pub fn get(&self, name: &str) -> Option<&Program> { self.shaders.get(name) }
}
