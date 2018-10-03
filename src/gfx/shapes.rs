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

use std::error::Error;

pub type VertexData2D = (VertexBuffer<Vertex2D>, IndexBuffer<u16>);
pub type VertexData3D = (VertexBuffer<Vertex3D>, IndexBuffer<u16>);

#[derive(Copy, Clone)]
pub struct Vertex3D {
    position: [f32; 3],
    tex_coords: [f32; 2],
    texture: u8,
}

#[derive(Copy, Clone)]
pub struct Vertex2D {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

pub struct Shapes {
    quad: VertexData2D,
    rectangle: VertexData2D,

    cube: VertexData3D,
}

impl Shapes {
    pub fn new(display: &Display) -> Result<Shapes, Box<Error>> {
        Ok(Shapes {
            quad: {
                (
                    VertexBuffer::new(
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
                    )?,
                    IndexBuffer::new(display, PrimitiveType::TriangleStrip, &[1u16, 2, 0, 3])?,
                )
            },
            rectangle: {
                (
                    VertexBuffer::new(
                        display,
                        &[
                            Vertex2D {
                                position: [-2.0, -1.0],
                                tex_coords: [0.0, 0.0],
                            },
                            Vertex2D {
                                position: [-2.0, 1.0],
                                tex_coords: [0.0, 1.0],
                            },
                            Vertex2D {
                                position: [2.0, 1.0],
                                tex_coords: [1.0, 1.0],
                            },
                            Vertex2D {
                                position: [2.0, -1.0],
                                tex_coords: [1.0, 0.0],
                            },
                        ],
                    )?,
                    IndexBuffer::new(display, PrimitiveType::TriangleStrip, &[1u16, 2, 0, 3])?,
                )
            },
            cube: {
                (
                    VertexBuffer::new(
                        display,
                        &[
                            Vertex3D {
                                position: [-1.0, 1.0, -1.0],
                                tex_coords: [0.0, 0.0],
                                texture: 0,
                            },
                            Vertex3D {
                                position: [1.0, 1.0, -1.0],
                                tex_coords: [1.0, 0.0],
                                texture: 0,
                            },
                            Vertex3D {
                                position: [1.0, 1.0, 1.0],
                                tex_coords: [1.0, 1.0],
                                texture: 0,
                            },
                            Vertex3D {
                                position: [-1.0, 1.0, 1.0],
                                tex_coords: [0.0, 1.0],
                                texture: 0,
                            },
                            Vertex3D {
                                position: [-1.0, -1.0, 1.0],
                                tex_coords: [0.0, 0.0],
                                texture: 0,
                            },
                            Vertex3D {
                                position: [1.0, -1.0, 1.0],
                                tex_coords: [0.0, 0.0],
                                texture: 0,
                            },
                            Vertex3D {
                                position: [1.0, -1.0, -1.0],
                                tex_coords: [0.0, 0.0],
                                texture: 0,
                            },
                            Vertex3D {
                                position: [-1.0, -1.0, -1.0],
                                tex_coords: [0.0, 0.0],
                                texture: 0,
                            },
                        ],
                    )?,
                    IndexBuffer::new(
                        display,
                        PrimitiveType::TrianglesList,
                        &[
                            0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 3, 2, 5, 3, 5, 4, 2, 1, 6, 2, 6, 5, 1,
                            7, 6, 1, 0, 7, 0, 3, 4, 0, 4, 7u16,
                        ][..],
                    )?,
                )
            },
        })
    }

    /// Get quad VAO and VBO
    pub fn quad(&self) -> &VertexData2D { &self.quad }

    /// Get rectangle VAO and VBO
    pub fn rectangle(&self) -> &VertexData2D { &self.rectangle }
}

implement_vertex!(Vertex3D, position, tex_coords, texture);
implement_vertex!(Vertex2D, position, tex_coords);
