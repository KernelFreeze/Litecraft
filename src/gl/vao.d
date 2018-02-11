/*
 * Copyright 2014-2018 Miguel Peláez <kernelfreeze@outlook.com>
 * 
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software
 * and associated documentation files (the "Software"), to deal in the Software without restriction,
 * including without limitation the rights to use, copy, modify, merge, publish, distribute,
 * sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 * 
 * The above copyright notice and this permission notice shall be included in all copies or
 * substantial portions of the Software.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
 * BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
 * DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

module gl.vao;

import accessors;
import gl.render;

/**
    GPU Vertex Array Object:

    Encapsulate vertex array state on the GPU side.
*/
final class VAO {
    @Read private uint _id;

    /// Ask the GPU to generate a new VAO
    this() {
        glGenVertexArrays(1, &_id);
        bind();
    }

    ~this() {
        glDeleteVertexArrays(1, &_id);
    }

    /// Bind VAO to current stack
    void bind() {
        glBindVertexArray(_id);
    }

    /// Unbind VBO from current stack
    void unbind() {
        glBindVertexArray(0);
    }

    mixin(GenerateFieldAccessors);
}