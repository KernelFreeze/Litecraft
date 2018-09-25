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
use gfx::scene::{Scene, SceneAction};

use core::resource_manager::resource::Resource;
use core::resource_manager::resource_type::ResourceType;

use glium::{Frame, Surface};

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
        copyright,
    }
}

/// Show Litecraft logo and start resource loading
pub struct MainMenu {
    ids: Ids,
}

impl MainMenu {
    pub fn new(canvas: &mut Canvas) -> MainMenu {
        MainMenu {
            ids: Ids::new(canvas.ui_mut().widget_id_generator()),
        }
    }
}

impl Scene for MainMenu {
    /// Do resource load
    fn load(&mut self, canvas: &mut Canvas) {
        canvas
            .resources_mut()
            .textures_mut()
            .load_ui(Resource::litecraft("logo", ResourceType::Texture));
    }

    /// Draw scene
    fn draw(&mut self, canvas: &mut Canvas, frame: &mut Frame) -> SceneAction {
        use conrod::{color, widget, Colorable, Positionable, Widget};

        // Clear to black
        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        let logo = canvas
            .resources()
            .textures()
            .get_ui(&Resource::litecraft("logo", ResourceType::Texture));

        let mut ui = canvas.ui_mut().set_widgets();

        // Construct our main `Canvas` tree.
        widget::Canvas::new()
            .flow_down(&[
                (self.ids.header, widget::Canvas::new().pad_bottom(20.0)),
                (
                    self.ids.body,
                    widget::Canvas::new().length(300.0).flow_right(&[
                        (self.ids.left_column, widget::Canvas::new()),
                        (self.ids.middle_column, widget::Canvas::new()),
                        (self.ids.right_column, widget::Canvas::new()),
                    ]),
                ),
                (
                    self.ids.footer,
                    widget::Canvas::new().pad(20.0).scroll_kids_vertically(),
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

        widget::Text::new("Open source client")
            .color(color::WHITE)
            .font_size(12)
            .bottom_right_of(self.ids.footer)
            .set(self.ids.copyright, &mut ui);

        SceneAction::None
    }
}
