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

use gfx::canvas::Canvas;

use glium::draw_parameters::Blend;
use glium::texture::CompressedSrgbTexture2d;
use glium::{DrawParameters, Frame, Surface};

use cgmath::Matrix4;

/// Geometry type
pub enum Geometry {
    /// 2D Quad
    Quad,
}

impl Canvas {
    /// Draw using program to fullscreen
    pub fn dummy_draw(&self, frame: &mut Frame, program: &str) {
        let uniforms = uniform! {
            time: ResourceManager::time(),
        };

        let (vertex_buffer, index_buffer) = self.resources().shapes().quad();
        let program = self
            .resources()
            .shaders()
            .get(program)
            .expect("Required shader not found");

        let parameters = DrawParameters {
            blend: Blend::alpha_blending(),
            ..Default::default()
        };

        frame
            .draw(vertex_buffer, index_buffer, program, &uniforms, &parameters)
            .expect("Failed to draw geometry to screen");
    }

    /// Draw shape to 3D space
    pub fn draw(
        &self, frame: &mut Frame, program: &str, texture: &CompressedSrgbTexture2d, camera: &Camera,
        geometry: Geometry, transform: Matrix4<f32>,
    ) {
        use glium::draw_parameters::DepthTest;
        use glium::uniforms::{MagnifySamplerFilter, SamplerWrapFunction};
        use glium::Depth;

        // Get updated camera matrices
        let persp_matrix: [[f32; 4]; 4] = camera.perspective().into();
        let view_matrix: [[f32; 4]; 4] = camera.view().into();
        let transform: [[f32; 4]; 4] = transform.into();
        let texture = texture
            .sampled()
            .wrap_function(SamplerWrapFunction::BorderClamp)
            .magnify_filter(MagnifySamplerFilter::Nearest);

        // Generate uniforms
        let uniforms = uniform! {
            persp_matrix: persp_matrix,
            view_matrix: view_matrix,
            time: ResourceManager::time(),
            transform: transform,
            tex: texture,
        };

        let (vertex_buffer, index_buffer) = match geometry {
            Geometry::Quad => self.resources().shapes().quad(),
        };
        let program = self
            .resources()
            .shaders()
            .get(program)
            .expect("Required shader not found");

        let parameters = DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            blend: Blend::alpha_blending(),
            ..Default::default()
        };

        frame
            .draw(vertex_buffer, index_buffer, program, &uniforms, &parameters)
            .expect("Failed to draw geometry to screen");
    }
}
