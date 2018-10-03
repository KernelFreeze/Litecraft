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

use std::borrow::Cow;

use core::camera::Camera;
use core::resource_manager::ResourceManager;

use gfx::canvas::Canvas;
use gfx::shapes::VertexData2D;

use glium::draw_parameters::Blend;
use glium::texture::CompressedSrgbTexture2d;
use glium::{BackfaceCullingMode, DrawParameters, Surface};

/// Utility for drawing on screen
pub struct Pencil<'a, S> {
    // Shader program
    program: Cow<'a, str>,
    linear: bool,

    frame: &'a mut S,

    // Uniforms
    persp_matrix: Option<[[f32; 4]; 4]>,
    view_matrix: Option<[[f32; 4]; 4]>,
    transform: Option<[[f32; 4]; 4]>,

    // Shape vertices
    vertices: &'a VertexData2D,

    texture: Option<&'a CompressedSrgbTexture2d>,

    canvas: &'a Canvas,
}

impl<'a, S> Pencil<'a, S>
where
    S: Surface,
{
    /// Create a new Pencil
    pub fn new<T>(frame: &'a mut S, program: T, canvas: &'a Canvas) -> Pencil<'a, S>
    where
        T: Into<Cow<'a, str>>,
    {
        Pencil {
            program: program.into(),
            vertices: canvas.resources().shapes().quad(),

            linear: false,

            view_matrix: None,
            persp_matrix: None,
            transform: None,

            texture: None,

            canvas,
            frame,
        }
    }

    /// Add vertices to draw
    pub fn vertices(&'a mut self, vertices: &'a VertexData2D) -> &'a mut Pencil<S> {
        self.vertices = vertices;
        self
    }

    /// Set if rendering should be linear
    pub fn linear(&'a mut self, linear: bool) -> &'a mut Pencil<S> {
        self.linear = linear;
        self
    }

    /// Add texture to draw
    pub fn texture(&'a mut self, texture: &'a CompressedSrgbTexture2d) -> &'a mut Pencil<S> {
        self.texture = Some(texture);
        self
    }

    /// Add camera to draw
    pub fn camera(&'a mut self, camera: &'a Camera) -> &'a mut Pencil<S> {
        self.persp_matrix = Some(camera.perspective().into());
        self.view_matrix = Some(camera.view().into());
        self
    }

    /// Add transform to draw
    pub fn transform(&'a mut self, transform: Matrix4<f32>) -> &'a mut Pencil<S> {
        self.transform = Some(transform.into());
        self
    }

    /// Draw shape to 3D space
    pub fn draw(&mut self) {
        use glium::draw_parameters::DepthTest;
        use glium::uniforms::{MagnifySamplerFilter, SamplerWrapFunction};
        use glium::Depth;

        let uniforms = uniform! {
            time: ResourceManager::time(),
            persp_matrix: self.persp_matrix.unwrap_or_else(|| Matrix4::one().into()),
            view_matrix: self.view_matrix.unwrap_or_else(|| Matrix4::one().into()),
            transform: self.transform.unwrap_or_else(|| Matrix4::one().into()),
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
            backface_culling: BackfaceCullingMode::CullCounterClockwise,
            ..Default::default()
        };

        let (vertex_buffer, index_buffer) = self.vertices;

        // Check if we need to attach a texture
        if let Some(texture) = self.texture {
            let magnify_filter = if self.linear {
                MagnifySamplerFilter::Linear
            } else {
                MagnifySamplerFilter::Nearest
            };

            let uniforms = uniforms
                .add(
                    "tex",
                    texture
                        .sampled()
                        .wrap_function(SamplerWrapFunction::BorderClamp)
                        .magnify_filter(magnify_filter),
                )
                .add("resolution", [texture.width() as f32, texture.height() as f32]);

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
