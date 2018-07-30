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

use gfx::canvas::Canvas;
use gfx::pencil::Geometry;
use gfx::scene::{Scene, SceneAction};

use glium::{Display, Frame, Surface};

/// Show Litecraft logo and start resource loading
pub struct LoadingScene {
    camera: Camera,
}

impl LoadingScene {
    pub fn new(res: &mut ResourceManager, display: &Display) -> LoadingScene {
        res.load_texture(Resource::litecraft("logo", ResourceType::Texture));

        res.load_shader("noise", display);
        res.load_shader("quad", display);
        res.load_shader("logo", display);

        LoadingScene {
            camera: Camera::new(),
        }
    }
}

impl Scene for LoadingScene {
    fn draw(&mut self, canvas: &mut Canvas, frame: &mut Frame) -> SceneAction {
        use cgmath::Matrix4;

        // Update camera aspect ratio
        self.camera.aspect_ratio(canvas.size());

        // Clear to black
        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        // Draw background
        canvas.dummy_draw(frame, "noise");

        // Draw litecraft logo
        let logo = canvas
            .resources()
            .textures()
            .get(&Resource::litecraft("logo", ResourceType::Texture));

        // Check if logo is now loaded
        if let Some(logo) = logo {
            let position = Matrix4::from_scale(1.0);

            canvas.draw(frame, "logo", logo, &self.camera, Geometry::Quad, position);
        }

        SceneAction::None
    }
}
