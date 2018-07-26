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

use glium::index::PrimitiveType;
use glium::{Display, IndexBuffer, VertexBuffer};

#[derive(Copy, Clone)]
pub struct Vertex3D {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

#[derive(Copy, Clone)]
pub struct Vertex2D {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex3D, position, tex_coords);
implement_vertex!(Vertex2D, position, tex_coords);

pub fn quad(display: &Display) -> (VertexBuffer<Vertex2D>, IndexBuffer<u16>) {
    let vertex = VertexBuffer::new(
        display,
        &[
            Vertex2D {
                position: [-1.0, -1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex2D {
                position: [-1.0, 1.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex2D {
                position: [1.0, 1.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex2D {
                position: [1.0, -1.0],
                tex_coords: [1.0, 0.0],
            },
        ],
    ).expect("Failed to generate VertexBuffer for quad");

    let index = IndexBuffer::new(display, PrimitiveType::TriangleStrip, &[1 as u16, 2, 0, 3])
        .expect("Failed to generate IndexBuffer for quad");

    (vertex, index)
}
