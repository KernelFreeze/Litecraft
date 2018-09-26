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

        header_left_column,
        header_right_column,

        title,
        logo_left,
        logo_right,

        body,

        body_left_column,
        body_middle_column,
        body_right_column,

        singleplayer,
        multiplayer,

        footer,

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

    fn draw_wallpaper(&mut self, canvas: &mut Canvas, frame: &mut Frame) {
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
}

impl Scene for MainMenu {
    /// Do resource load
    fn load(&mut self, canvas: &mut Canvas) {
        canvas
            .resources_mut()
            .textures_mut()
            .load_ui(Resource::minecrafty_path(
                "minecraft",
                "gui/title",
                ResourceType::Texture,
            ));
    }

    /// Draw scene
    fn draw(&mut self, canvas: &mut Canvas, frame: &mut Frame) -> SceneAction {
        use conrod::{color, widget, Borderable, Colorable, Positionable, Sizeable, Widget};

        // Clear to black
        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        let logo = canvas.resources().textures().get_ui(&Resource::minecrafty_path(
            "minecraft",
            "gui/title",
            ResourceType::Texture,
        ));

        self.draw_wallpaper(canvas, frame);

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
                        .pad(85.0)
                        .flow_right(&[
                            (
                                self.ids.header_left_column,
                                widget::Canvas::new().color(color::TRANSPARENT).border(0.0),
                            ),
                            (
                                self.ids.header_right_column,
                                widget::Canvas::new().color(color::TRANSPARENT).border(0.0),
                            ),
                        ]),
                ),
                (
                    self.ids.body,
                    widget::Canvas::new()
                        .color(color::TRANSPARENT)
                        .border(0.0)
                        .length(300.0)
                        .flow_right(&[
                            (
                                self.ids.body_left_column,
                                widget::Canvas::new().color(color::TRANSPARENT).border(0.0),
                            ),
                            (
                                self.ids.body_middle_column,
                                widget::Canvas::new().color(color::TRANSPARENT).border(0.0),
                            ),
                            (
                                self.ids.body_right_column,
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

        // Draw the beloved Minecraft logo
        if let Some(logo) = logo {
            use conrod::position::rect::Rect;

            let base = 256.0;
            let (w, h) = logo.1;

            // Draw logo first part
            let logo_1 = widget::Image::new(logo.0)
                .bottom_right_of(self.ids.header_left_column)
                .h_of(self.ids.header_left_column)
                .source_rectangle(Rect::from_corners(
                    // Use only part of our texture
                    [
                        0.0,              // x from
                        212.0 * h / base, // y from
                    ],
                    [
                        156.0 * w / base, // x to
                        256.0 * h / base, // y to
                    ],
                ));

            let logo_2 = widget::Image::new(logo.0)
                .bottom_left_of(self.ids.header_right_column)
                .h_of(self.ids.header_right_column)
                .source_rectangle(Rect::from_corners(
                    // Use only part of our texture
                    [
                        0.0,              // x from
                        168.0 * h / base, // y from
                    ],
                    [
                        156.0 * w / base, // x to
                        211.0 * h / base, // y to
                    ],
                ));

            let w = 124.0 * w / base;

            if let Some(xh) = logo_1.get_h(&ui) {
                logo_1.w(xh * h / w).set(self.ids.logo_left, &mut ui);
            }

            if let Some(xh) = logo_2.get_h(&ui) {
                logo_2.w(xh * h / w).set(self.ids.logo_right, &mut ui);
            }
        }

        // Footer

        // Litecraft and Minecraft version
        widget::Text::new(&format!(
            "Litecraft {}\nMinecraft {}",
            LITECRAFT_VERSION, MINECRAFT_VERSION
        )).color(color::WHITE)
        .font_size(16)
        .bottom_left_of(self.ids.footer)
        .set(self.ids.version, &mut ui);

        // Credits
        widget::Text::new("Made with love by the community")
            .color(color::WHITE)
            .font_size(16)
            .bottom_right_of(self.ids.footer)
            .set(self.ids.copyright, &mut ui);

        SceneAction::None
    }
}
