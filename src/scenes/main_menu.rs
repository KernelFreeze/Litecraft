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
use gfx::ui_helper;

use core::camera::Camera;
use core::constants::*;

use core::resource_manager::resource::Resource;
use core::resource_manager::resource_type::ResourceType;
use core::resource_manager::ResourceManager;

use glium::Frame;

use conrod::position::rect::Rect;

/// How many time we should wait before changing our wallpaper
const WALLPAPER_DELAY: u32 = 40;

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

        body_footer,
        body_footer_left,
        body_footer_right,

        singleplayer,
        multiplayer,
        realms,
        options,
        quit,

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
        let ids = Ids::new(canvas.ui_mut().widget_id_generator());
        let mut camera = Camera::new();

        camera.set_fov(55.0);

        MainMenu { ids, camera }
    }

    /// Main menu's background
    fn draw_wallpaper(&mut self, canvas: &mut Canvas, frame: &mut Frame) {
        let i = ResourceManager::time() as u32 / WALLPAPER_DELAY % 5;

        let wallpaper = canvas.resources().textures().get(&Resource::minecraft_path(
            format!("panorama_{}", i),
            "gui/title/background",
            ResourceType::Texture,
        ));

        if let Some(wallpaper) = wallpaper {
            Pencil::new(frame, "wallpaper", &canvas)
                .texture(wallpaper)
                .camera(&self.camera)
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
            .load_ui(Resource::minecraft_path(
                "minecraft",
                "gui/title",
                ResourceType::Texture,
            ));

        canvas
            .resources_mut()
            .textures_mut()
            .load_ui(Resource::minecraft_path("widgets", "gui", ResourceType::Texture));
    }

    /// Draw scene
    fn draw(&mut self, canvas: &mut Canvas, frame: &mut Frame) -> SceneAction {
        use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};

        let logo = canvas.resources().textures().get_ui(&Resource::minecraft_path(
            "minecraft",
            "gui/title",
            ResourceType::Texture,
        ));

        let widgets = canvas.resources().textures().get_ui(&Resource::minecraft_path(
            "widgets",
            "gui",
            ResourceType::Texture,
        ));

        self.draw_wallpaper(canvas, frame);

        let scale = canvas.settings().scale();
        let mut ui = canvas.ui_mut().set_widgets();

        // Construct our main `Canvas` tree.
        widget::Canvas::new()
            .flow_down(&[
                (
                    self.ids.header,
                    widget::Canvas::new().pad(85.0).flow_right(&[
                        (self.ids.header_left_column, widget::Canvas::new()),
                        (self.ids.header_right_column, widget::Canvas::new()),
                    ]),
                ),
                (self.ids.body, widget::Canvas::new().length(300.0)),
                (
                    self.ids.footer,
                    widget::Canvas::new().pad(20.0).scroll_kids_vertically(),
                ),
            ])
            .set(self.ids.master, &mut ui);

        // Draw the beloved Minecraft logo
        if let Some(logo) = logo {
            // Texture coordinates
            let base = 256.0;
            let size = [280.0 * scale, 85.0 * scale];
            let (w, h) = logo.1;

            // Draw logo first part
            widget::Image::new(logo.0)
                .bottom_right_of(self.ids.header_left_column)
                .wh(size)
                .source_rectangle(Rect::from_corners(
                    [0.0, 212.0 * h / base],
                    [156.0 * w / base, 256.0 * h / base],
                ))
                .set(self.ids.logo_left, &mut ui);

            // Draw logo second part
            widget::Image::new(logo.0)
                .bottom_left_of(self.ids.header_right_column)
                .wh(size)
                .source_rectangle(Rect::from_corners(
                    [0.0, 168.0 * h / base],
                    [156.0 * w / base, 211.0 * h / base],
                ))
                .set(self.ids.logo_right, &mut ui);
        }

        if let Some(widgets) = widgets {
            ui_helper::button(&widgets, scale)
                .label("Singleplayer")
                .up_from(self.ids.multiplayer, 15.0 * scale)
                .set(self.ids.singleplayer, &mut ui);

            ui_helper::button(&widgets, scale)
                .label("Multiplayer")
                .middle_of(self.ids.body)
                .set(self.ids.multiplayer, &mut ui);

            ui_helper::button(&widgets, scale)
                .label("Minecraft Realms")
                .down_from(self.ids.multiplayer, 15.0 * scale)
                .set(self.ids.realms, &mut ui);

            widget::Canvas::new()
                .flow_right(&[
                    (self.ids.body_footer_left, widget::Canvas::new()),
                    (self.ids.body_footer_right, widget::Canvas::new()),
                ])
                .w(480.0 * scale)
                .down_from(self.ids.realms, 50.0 * scale)
                .set(self.ids.body_footer, &mut ui);

            ui_helper::button(&widgets, scale)
                .label("Options")
                .top_left_of(self.ids.body_footer_left)
                .padded_w_of(self.ids.body_footer_left, 5.0)
                .set(self.ids.options, &mut ui);

            if ui_helper::button(&widgets, scale)
                .label("Quit Game")
                .top_right_of(self.ids.body_footer_right)
                .padded_w_of(self.ids.body_footer_right, 5.0)
                .set(self.ids.quit, &mut ui)
                .was_clicked()
            {
                return SceneAction::Quit;
            }
        }

        // Litecraft and Minecraft version
        widget::Text::new(VERSION_TEXT)
            .color(color::WHITE)
            .font_size(16)
            .bottom_left_of(self.ids.footer)
            .set(self.ids.version, &mut ui);

        // Credits
        widget::Text::new("© Litecraft Team")
            .color(color::WHITE)
            .font_size(16)
            .bottom_right_of(self.ids.footer)
            .set(self.ids.copyright, &mut ui);

        SceneAction::None
    }
}
