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

use cgmath::prelude::*;
use cgmath::{ortho, perspective, Deg, Euler, Matrix4, Point3, Quaternion, Vector3};
use std::f32;

const YAW: f32 = -90.0;
const PITCH: f32 = 0.0;

/// Camera movement direction
enum Movement {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
}

/// Basic camera
pub struct Camera {
    // Camera Attributes
    position: Point3<f32>,
    yaw: f32,
    pitch: f32,

    // Window size
    width: u32,
    height: u32,
}

impl Camera {
    /// Create new camera
    pub fn new() -> Camera {
        Camera {
            position: Point3::new(0.0, 0.0, 1.0),

            yaw: YAW,
            pitch: PITCH,

            width: 800,
            height: 600,
        }
    }

    /// Set current window size to match aspect ratio
    pub fn aspect_ratio(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    /// Get current camera position
    pub fn position(&self) -> Point3<f32> { self.position }

    /// Get current camera yaw
    pub fn yaw(&self) -> f32 { self.yaw }

    /// Get current camera pitch
    pub fn pitch(&self) -> f32 { self.pitch }

    /// Set camera position
    pub fn set_position(&mut self, pos: Point3<f32>) { self.position = pos; }

    /// Set camera yaw
    pub fn set_yaw(&mut self, value: f32) { self.yaw = value; }

    /// Set camera pitch
    pub fn set_pitch(&mut self, value: f32) { self.pitch = value; }

    /// Get perspective matrix
    pub fn perspective(&self) -> Matrix4<f32> {
        let zfar = 1024.0;
        let znear = 1.0;

        let width: f32 = self.width as f32;
        let height: f32 = if self.height == 0 {
            600.0
        } else {
            self.height as f32
        };

        let aspect_ratio = width / height;

        perspective(Deg(90.0), aspect_ratio, znear, zfar)
    }

    /// Get orthographic matrix
    pub fn ortho(&self) -> Matrix4<f32> {
        ortho(0.0, self.width as f32, self.height as f32, 0.0, 0.0, 1024.0)
    }

    /// Get view matrix
    pub fn view(&self) -> Matrix4<f32> {
        let look = Quaternion::from(Euler {
            x: Deg(0.0),
            y: Deg(self.pitch),
            z: Deg(self.yaw),
        });

        let look = look.v.normalize();
        let up = Vector3::new(0.0, 1.0, 0.0);

        // Calculate 4x4 view matrix
        Matrix4::look_at_dir(self.position, look, up)
    }
}
