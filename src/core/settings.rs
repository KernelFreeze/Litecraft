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

#[derive(Serialize, Deserialize, Debug)]
pub struct WindowSettings {
    width: u32,
    height: u32,
    fullscreen: bool,
    maximized: bool,
    multisampling: u16,
    gui_scale: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameplaySettings {
    fov: u8,
    vsync: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    window: WindowSettings,
    gameplay: GameplaySettings,
    resourcepacks: Vec<String>,
}

impl Settings {
    /// Create settings with default values
    pub fn new() -> Settings { Settings::new_with_size(800, 600) }

    /// Create settings with width and height
    pub fn new_with_size(width: u32, height: u32) -> Settings {
        Settings {
            window: WindowSettings {
                width,
                height,
                fullscreen: false,
                maximized: true,
                multisampling: 0,
                gui_scale: 1.0,
            },
            gameplay: GameplaySettings { fov: 90, vsync: true },
            resourcepacks: Vec::new(),
        }
    }

    /// Get window width
    pub fn width(&self) -> u32 { self.window.width }

    /// Set window width
    pub fn set_width(&mut self, value: u32) { self.window.width = value }

    /// Get window height
    pub fn height(&self) -> u32 { self.window.height }

    /// Set window height
    pub fn set_height(&mut self, value: u32) { self.window.height = value }

    /// Get if user wants fullscreen
    pub fn fullscreen(&self) -> bool { self.window.fullscreen }

    /// Get if user wants maximized
    pub fn maximized(&self) -> bool { self.window.maximized }

    /// Get if user wants MSAA anti-aliasing
    pub fn multisampling(&self) -> u16 { self.window.multisampling }

    /// Get if user wants vsync
    pub fn vsync(&self) -> bool { self.gameplay.vsync }

    /// Get user FOV
    pub fn fov(&self) -> u8 { self.gameplay.fov }

    /// Get user GUI scale
    pub fn scale(&self) -> f64 { self.window.gui_scale }

    /// Get enabled resourcepacks by filename
    pub fn resourcepacks(&self) -> &Vec<String> { &self.resourcepacks }
}
