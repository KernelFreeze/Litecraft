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

use cgmath::Matrix4;

use core::camera::Camera;
use core::resource_manager::ResourceManager;

use gfx::canvas::Canvas;
use gfx::shapes::VertexData;

use glium::draw_parameters::Blend;
use glium::texture::CompressedSrgbTexture2d;
use glium::{DrawParameters, Frame, Surface};

/// Utility for drawing on screen
pub struct Pencil<'a> {
    // Required
    program: String,
    frame: &'a mut Frame,

    // Optional
    vertices: Option<&'a VertexData>,
    texture: Option<&'a CompressedSrgbTexture2d>,
    camera: Option<&'a Camera>,
    transform: Option<Matrix4<f32>>,
}

impl<'a> Pencil<'a> {
    /// Create a new Pencil
    pub fn new(frame: &'a mut Frame, program: &str) -> Pencil<'a> {
        Pencil {
            program: program.into(),
            frame: frame,

            vertices: None,
            texture: None,
            camera: None,
            transform: None,
        }
    }

    /// Add vertices to draw
    pub fn vertices(&'a mut self, vertices: &'a VertexData) -> &'a mut Pencil {
        self.vertices = Some(vertices);
        self
    }

    /// Add texture to draw
    pub fn texture(&'a mut self, texture: &'a CompressedSrgbTexture2d) -> &'a mut Pencil {
        self.texture = Some(texture);
        self
    }

    /// Add camera to draw
    pub fn camera(&'a mut self, camera: &'a Camera) -> &'a mut Pencil {
        self.camera = Some(camera);
        self
    }

    /// Add transform to draw
    pub fn transform(&'a mut self, transform: Matrix4<f32>) -> &'a mut Pencil {
        self.transform = Some(transform);
        self
    }

    /// Draw shape to 3D space
    pub fn draw(&mut self, canvas: &Canvas) {
        use glium::draw_parameters::DepthTest;
        use glium::uniforms::{MagnifySamplerFilter, SamplerWrapFunction};
        use glium::Depth;

        use cgmath::Matrix4;
        use cgmath::One;

        // Check if camera is available or use default
        let (persp_matrix, view_matrix) = if let Some(camera) = self.camera {
            let persp_matrix: [[f32; 4]; 4] = camera.perspective().into();
            let view_matrix: [[f32; 4]; 4] = camera.view().into();

            (persp_matrix, view_matrix)
        } else {
            let matrix = Matrix4::one();

            let persp_matrix: [[f32; 4]; 4] = matrix.into();
            let view_matrix: [[f32; 4]; 4] = matrix.into();

            (persp_matrix, view_matrix)
        };

        // Check if we need to transform or use default
        let transform = if let Some(transform) = self.transform {
            let transform: [[f32; 4]; 4] = transform.into();

            transform
        } else {
            let transform: [[f32; 4]; 4] = Matrix4::one().into();

            transform
        };

        let uniforms = uniform! {
            time: ResourceManager::time(),
            persp_matrix: persp_matrix,
            view_matrix: view_matrix,
            transform: transform,
        };

        // Get vertices or use default quad
        let (vertex_buffer, index_buffer) = if let Some(vertices) = self.vertices {
            vertices
        } else {
            canvas.resources().shapes().quad()
        };

        // Get previously loaded shader program
        let program = canvas
            .resources()
            .shaders()
            .get(&self.program)
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

        // Check if we need to attach a texture
        if let Some(texture) = self.texture {
            let uniforms = uniforms.add(
                "tex",
                texture
                    .sampled()
                    .wrap_function(SamplerWrapFunction::BorderClamp)
                    .magnify_filter(MagnifySamplerFilter::Nearest),
            );

            self.frame
                .draw(vertex_buffer, index_buffer, program, &uniforms, &parameters)
                .expect("Failed to draw textured geometry to screen");
        } else {
            self.frame
                .draw(vertex_buffer, index_buffer, program, &uniforms, &parameters)
                .expect("Failed to draw geometry to screen");
        };
    }
}
