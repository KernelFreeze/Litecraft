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

use core::camera::Camera;
use core::resource_manager::ResourceManager;

use glium::Program;

/// Geometry type
pub enum Geometry {
    /// 2D Quad, using: X, Y, Size
    Quad(f32, f32, f32),

    /// Fullscreen quad
    Fullscreen,
}

pub fn draw(shader: &Program, camera: &Camera, geometry: &Geometry) {
    // Get updated camera matrices
    let persp_matrix: [[f32; 4]; 4] = camera.perspective().into();
    let view_matrix: [[f32; 4]; 4] = camera.view().into();

    // Generate uniforms
    let uniforms = uniform! {
        persp_matrix: persp_matrix,
        view_matrix: view_matrix,
        time: ResourceManager::time(),
        //tex: logo,
    };

    match geometry {
        Geometry::Quad(x, y, z) => {},
        Geometry::Fullscreen => {},
    }
}
