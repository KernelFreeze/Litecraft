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

use gfx::canvas::Canvas;
use gfx::pencil::Pencil;
use gfx::scene::{Scene, SceneAction};

use scenes::main_menu::MainMenu;

use glium::{Frame, Surface};

/// Show Litecraft logo and start resource loading
pub struct LoadingScene {
    camera: Camera,
}

impl LoadingScene {
    pub fn new() -> LoadingScene {
        LoadingScene {
            camera: Camera::new(),
        }
    }

    pub fn draw_logo(&mut self, canvas: &mut Canvas, frame: &mut Frame) {
        let logo = canvas
            .resources()
            .textures()
            .get(&Resource::litecraft("logo", ResourceType::Texture));

        // Check if logo is now loaded
        if let Some(logo) = logo {
            Pencil::new(frame, "logo", &canvas)
                .camera(&self.camera)
                .texture(logo)
                .linear(true)
                .draw();
        }
    }
}

impl Scene for LoadingScene {
    /// Do resource load
    fn load(&mut self, canvas: &mut Canvas) {
        canvas
            .resources_mut()
            .textures_mut()
            .load(Resource::litecraft("logo", ResourceType::Texture));

        let display = canvas.display().clone();

        // Load shaders
        canvas
            .resources_mut()
            .shaders_mut()
            .load("noise", &display)
            .expect("Failed to load required shader program");

        canvas
            .resources_mut()
            .shaders_mut()
            .load("quad", &display)
            .expect("Failed to load required shader program");

        canvas
            .resources_mut()
            .shaders_mut()
            .load("wallpaper", &display)
            .expect("Failed to load required shader program");

        canvas
            .resources_mut()
            .shaders_mut()
            .load("logo", &display)
            .expect("Failed to load required shader program");

        // Load wallpapers from 1 to 12
        for i in 0..6 {
            canvas
                .resources_mut()
                .textures_mut()
                .load(Resource::minecraft_path(
                    format!("panorama_{}", i),
                    "gui/title/background",
                    ResourceType::Texture,
                ));
        }
    }

    /// Draw scene
    fn draw(&mut self, canvas: &mut Canvas, frame: &mut Frame) -> SceneAction {
        // Update camera aspect ratio
        self.camera
            .aspect_ratio(canvas.settings().width(), canvas.settings().height());

        // Clear to black
        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        // Draw background
        Pencil::new(frame, "noise", &canvas).draw();

        // Draw litecraft logo
        self.draw_logo(canvas, frame);

        if canvas.resources().loaded() {
            info!("All resources are now loaded, opening main menu");

            SceneAction::ChangeScene(box MainMenu::new(canvas))
        } else {
            SceneAction::None
        }
    }
}
