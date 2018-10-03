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

use core::resource_manager::texture_manager::UiTexture;

use conrod::position::rect::Rect;
use conrod::widget::button::Image;
use conrod::widget::Button;
use conrod::{color, widget, Labelable, Sizeable};

/// Draw Litecraft button widget
pub fn button<'a>(widgets: &UiTexture, scale: f64) -> Button<'a, Image> {
    let base = 256.0;
    let (w, h) = widgets.1;

    // Texture coordinates
    let base_rect = Rect::from_corners([0.0, 170.0 * h / base], [200.0 * w / base, 190.0 * h / base]);
    let hover_rect = Rect::from_corners([0.0, 150.0 * h / base], [200.0 * w / base, 170.0 * h / base]);
    let press_rect = Rect::from_corners([0.0, 190.0 * h / base], [200.0 * w / base, 210.0 * h / base]);

    widget::Button::image(widgets.0)
        .h(45.0 * scale)
        .w(480.0 * scale)
        .label_font_size((12.0 * scale) as u32)
        .label_color(color::WHITE)
        .center_justify_label()
        .source_rectangle(base_rect)
        .hover_source_rectangle(hover_rect)
        .press_source_rectangle(press_rect)
}
