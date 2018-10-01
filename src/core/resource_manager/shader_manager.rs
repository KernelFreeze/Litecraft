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

use glium::Display;
use glium::Program;

use std::collections::HashMap;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<Error>>;

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

    /// Get a compiled shader program
    pub fn get(&self, name: &str) -> Option<&Program> { self.shaders.get(name) }

    /// Load and build a shader
    pub fn load(&mut self, name: &'static str, display: &Display) -> Result<()> {
        if self.get(name).is_some() {
            warn!("Shader '{}' is already loaded!", name);
            return Ok(());
        }

        let program = program!(display,
        140 => {
            vertex: &(Resource::litecraft(name, ResourceType::VertexShader).load()?),
            fragment: &(Resource::litecraft(name, ResourceType::FragmentShader).load()?)
        })?;

        info!("Loaded shader '{}'", name);
        self.shaders.insert(name, program);

        Ok(())
    }
}
