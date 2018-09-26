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

use gfx::canvas::Canvas;
use gfx::pencil::Pencil;
use gfx::scene::{Scene, SceneAction};

use core::camera::Camera;
use core::constants::*;

use core::resource_manager::resource::Resource;
use core::resource_manager::resource_type::ResourceType;
use core::resource_manager::ResourceManager;

use glium::{Frame, Surface};

/// How many time we should wait before changing our wallpaper
const WALLPAPER_DELAY: u32 = 15;

widget_ids! {
    struct Ids {
        master,

        header,
        body,
        footer,

        left_column,
        middle_column,
        right_column,

        title,
        logo,
        singleplayer,
        multiplayer,

        version,
        copyright,
    }
}

/// Show Litecraft logo and start resource loading
pub struct MainMenu {
    ids: Ids,
    camera: Camera,
}

impl MainMenu {
    pub fn new(canvas: &mut Canvas) -> MainMenu {
        MainMenu {
            ids: Ids::new(canvas.ui_mut().widget_id_generator()),
            camera: Camera::new(),
        }
    }
}

impl Scene for MainMenu {
    /// Do resource load
    fn load(&mut self, canvas: &mut Canvas) {
        canvas
            .resources_mut()
            .textures_mut()
            .load_ui(Resource::litecrafty("logo", ResourceType::Texture));

        // Load wallpapers from 1 to 12
        for i in 1..12 {
            canvas
                .resources_mut()
                .textures_mut()
                .load(Resource::litecraft_path(
                    format!("menu_{}", i),
                    "wallpapers".to_string(),
                    ResourceType::Texture,
                ));
        }
    }

    /// Draw scene
    fn draw(&mut self, canvas: &mut Canvas, frame: &mut Frame) -> SceneAction {
        use conrod::{color, widget, Borderable, Colorable, Positionable, Widget};

        // Clear to black
        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        // Draw wallpaper
        {
            let i = ResourceManager::time() as u32 / WALLPAPER_DELAY % 12 + 1;
            let wallpaper = canvas.resources().textures().get(&Resource::litecraft_path(
                format!("menu_{}", i),
                "wallpapers".to_string(),
                ResourceType::Texture,
            ));

            if let Some(wallpaper) = wallpaper {
                Pencil::new(frame, "blur", &canvas)
                    .texture(wallpaper)
                    .camera(&self.camera)
                    .vertices(canvas.resources().shapes().rectangle())
                    .linear(true)
                    .draw();
            }
        }

        let logo = canvas
            .resources()
            .textures()
            .get_ui(&Resource::litecrafty("logo", ResourceType::Texture));

        let mut ui = canvas.ui_mut().set_widgets();

        // Construct our main `Canvas` tree.
        widget::Canvas::new()
            .color(color::TRANSPARENT)
            .flow_down(&[
                (
                    self.ids.header,
                    widget::Canvas::new()
                        .color(color::TRANSPARENT)
                        .border(0.0)
                        .pad_bottom(20.0),
                ),
                (
                    self.ids.body,
                    widget::Canvas::new()
                        .color(color::TRANSPARENT)
                        .border(0.0)
                        .length(300.0)
                        .flow_right(&[
                            (
                                self.ids.left_column,
                                widget::Canvas::new().color(color::TRANSPARENT).border(0.0),
                            ),
                            (
                                self.ids.middle_column,
                                widget::Canvas::new().color(color::TRANSPARENT).border(0.0),
                            ),
                            (
                                self.ids.right_column,
                                widget::Canvas::new().color(color::TRANSPARENT).border(0.0),
                            ),
                        ]),
                ),
                (
                    self.ids.footer,
                    widget::Canvas::new()
                        .pad(20.0)
                        .scroll_kids_vertically()
                        .border(0.0)
                        .color(color::TRANSPARENT),
                ),
            ]).set(self.ids.master, &mut ui);

        // if let Some(logo) = logo {
        // widget::Image::new(logo)
        // .middle_of(self.ids.header)
        // .set(self.ids.logo, &mut ui);
        // }

        widget::Text::new("Litecraft")
            .color(color::WHITE)
            .font_size(64)
            .middle_of(self.ids.header)
            .set(self.ids.title, &mut ui);

        widget::Text::new(&format!(
            "Litecraft {}\nMinecraft {}",
            LITECRAFT_VERSION, MINECRAFT_VERSION
        )).color(color::WHITE)
        .font_size(16)
        .bottom_left_of(self.ids.footer)
        .set(self.ids.version, &mut ui);

        widget::Text::new("Open source client")
            .color(color::WHITE)
            .font_size(16)
            .bottom_right_of(self.ids.footer)
            .set(self.ids.copyright, &mut ui);

        SceneAction::None
    }
}
