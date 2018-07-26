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

use core::resource_manager::resource::Resource;
use core::resource_manager::resource_type::ResourceType;
use core::resource_manager::ResourceManager;

use gfx::scene::{Scene, SceneAction};
use gfx::shapes;
use gfx::shapes::Vertex2D;

use glium::uniforms::EmptyUniforms;
use glium::{Display, Frame, IndexBuffer, Surface, VertexBuffer};

/// Show Litecraft logo and start resource loading
pub struct LoadingScene {
    vertex_buffer: VertexBuffer<Vertex2D>,
    index_buffer: IndexBuffer<u16>,

    camera: Camera,
}

impl LoadingScene {
    pub fn new(res: &mut ResourceManager, display: &Display) -> LoadingScene {
        let (vertex_buffer, index_buffer) = shapes::quad(display);

        res.load_texture(Resource::litecraft("logo", ResourceType::Texture));

        res.load_shader("noise", display);
        res.load_shader("quad", display);
        res.load_shader("logo", display);

        LoadingScene {
            camera: Camera::new(),
            vertex_buffer,
            index_buffer,
        }
    }
}

impl Scene for LoadingScene {
    fn draw(&mut self, res: &mut ResourceManager, frame: &mut Frame, display: &Display) -> SceneAction {
        // Update camera aspect ratio
        self.camera.aspect_ratio(res.size());

        // Clear to black
        frame.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let noise_program = res.shaders().get("noise").expect("Required shader not found");

        frame
            .draw(
                &self.vertex_buffer,
                &self.index_buffer,
                &noise_program,
                &EmptyUniforms,
                &res.no_depth(),
            )
            .expect("Failed to draw geometry to screen");

        // Draw litecraft logo
        let logo = res
            .textures()
            .get(&Resource::litecraft("logo", ResourceType::Texture));

        // Check if logo is now loaded
        if let Some(logo) = logo {
            use glium::uniforms::SamplerWrapFunction;

            // Get updated camera matrices
            let persp_matrix: [[f32; 4]; 4] = self.camera.perspective().into();
            let view_matrix: [[f32; 4]; 4] = self.camera.view().into();

            // Change logo sampler
            let logo = logo.sampled().wrap_function(SamplerWrapFunction::BorderClamp);

            // Generate uniforms
            let uniforms = uniform! {
                persp_matrix: persp_matrix,
                view_matrix: view_matrix,
                time: ResourceManager::time(),
                tex: logo,
            };

            let logo_program = res.shaders().get("logo").expect("Required shader not found");

            // Draw using logo program
            frame
                .draw(
                    &self.vertex_buffer,
                    &self.index_buffer,
                    &logo_program,
                    &uniforms,
                    &res.no_depth(),
                )
                .expect("Failed to draw geometry to screen");
        }

        SceneAction::None
    }
}
