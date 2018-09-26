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

use cgmath::{Matrix4, One};

use core::camera::Camera;
use core::resource_manager::ResourceManager;

use gfx::canvas::Canvas;
use gfx::shapes::VertexData;

use glium::draw_parameters::Blend;
use glium::texture::CompressedSrgbTexture2d;
use glium::{DrawParameters, Surface};

/// Utility for drawing on screen
pub struct Pencil<'a, S> {
    program: String,
    linear: bool,

    frame: &'a mut S,

    persp_matrix: [[f32; 4]; 4],
    view_matrix: [[f32; 4]; 4],

    vertices: &'a VertexData,
    transform: [[f32; 4]; 4],

    texture: Option<&'a CompressedSrgbTexture2d>,

    canvas: &'a Canvas,
}

impl<'a, S> Pencil<'a, S> {
    /// Create a new Pencil
    pub fn new(frame: &'a mut S, program: &str, canvas: &'a Canvas) -> Pencil<'a, S>
    where
        S: Surface,
    {
        Pencil {
            program: program.into(),
            linear: false,

            view_matrix: Matrix4::one().into(),
            persp_matrix: Matrix4::one().into(),

            vertices: canvas.resources().shapes().quad(),
            transform: Matrix4::one().into(),

            texture: None,

            canvas,
            frame,
        }
    }

    /// Add vertices to draw
    pub fn vertices(&'a mut self, vertices: &'a VertexData) -> &'a mut Pencil<S>
    where
        S: Surface,
    {
        self.vertices = vertices;
        self
    }

    /// Set if rendering should be linear
    pub fn linear(&'a mut self, linear: bool) -> &'a mut Pencil<S>
    where
        S: Surface,
    {
        self.linear = linear;
        self
    }

    /// Add texture to draw
    pub fn texture(&'a mut self, texture: &'a CompressedSrgbTexture2d) -> &'a mut Pencil<S>
    where
        S: Surface,
    {
        self.texture = Some(texture);
        self
    }

    /// Add camera to draw
    pub fn camera(&'a mut self, camera: &'a Camera) -> &'a mut Pencil<S>
    where
        S: Surface,
    {
        self.persp_matrix = camera.perspective().into();
        self.view_matrix = camera.view().into();
        self
    }

    /// Add transform to draw
    pub fn transform(&'a mut self, transform: Matrix4<f32>) -> &'a mut Pencil<S>
    where
        S: Surface,
    {
        self.transform = transform.into();
        self
    }

    /// Draw shape to 3D space
    pub fn draw(&mut self)
    where
        S: Surface,
    {
        use glium::draw_parameters::DepthTest;
        use glium::uniforms::{MagnifySamplerFilter, SamplerWrapFunction};
        use glium::Depth;

        let uniforms = uniform! {
            time: ResourceManager::time(),
            persp_matrix: self.persp_matrix,
            view_matrix: self.view_matrix,
            transform: self.transform,
        };

        // Get previously loaded shader program
        let program = self
            .canvas
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
            multisampling: true,
            ..Default::default()
        };

        let (vertex_buffer, index_buffer) = self.vertices;

        let magnify_filter = if self.linear {
            MagnifySamplerFilter::Linear
        } else {
            MagnifySamplerFilter::Nearest
        };

        // Check if we need to attach a texture
        if let Some(texture) = self.texture {
            let uniforms = uniforms
                .add(
                    "tex",
                    texture
                        .sampled()
                        .wrap_function(SamplerWrapFunction::BorderClamp)
                        .magnify_filter(magnify_filter),
                ).add("resolution", [texture.width() as f32, texture.height() as f32]);

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
