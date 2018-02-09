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

import gl;
import resource_manager;
import dlib.geometry;

/* How to add a new primitive:
    load():
        1. Create & bind a VAO. Any VBO and IBO that you bind in the sequence will be
            associated with the current VAO (this one).

        2. Create & bind index buffer.
            Fill index buffer with glBufferData/glMapBuffer.
        
        3. Create & bind vertex buffer.
            Fill vertex buffer glBufferData/glMapBuffer.
        
        4. Set up vertex attributes with glEnableVertexAttribArray/glVertexAttribPointer, etc.

        5. Optionally unbind everything to avoid accidental modification of the buffers and VAO.
            Remember to unbind the VAO first. E.g.: glBindVertexArray(0);

    draw():
        1. If only drawing the buffers:
            Bind the VAO;
            Perform the draw call(s).

        2. If updating and drawing:
            Bind the VAO, VBO, IBO;
            Update the buffers (updating vertex attributes is only necessary if the vertex formats changed);
            Perform the draw call(s).

        3. Optionally unbind to avoid accidental modification of the buffers and VAO.
*/

/// A drawable primitive element
public abstract class Drawable : Loadable {
    private static Drawable _instance;

    /// Get a instance of this geometric
    public static auto instance() {
        return _instance;
    }

    /// Create a new primitive
    this() {
        _instance = this;
    }
}

/// 2D Quad
public final class Quad : Drawable {
    private VAO vao; // Status
    private VBO vbo; // Vertex
    private VBO ebo; // Elements

    /// Draw primitive on screen
    static void draw(Plane plane) {
        (cast(Quad) instance).vao.bind;
        glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_INT, null);
        (cast(Quad) instance).vao.unbind;
    }

    override void load() {
        // Generate and bind VAO
        vao = new VAO;

        // Generate Index Buffer Object
        ebo = new VBO([0, 1, 2, 2, 3, 0]);

        // Generate Vertex Buffer Object
        vbo = new VBO([
            -0.5f, 0.5f, 1.0f, // Top-left
            0.5f, 0.5f, 0.0f, // Top-right
            0.5f, -0.5f, 0.0f, // Bottom-right
            -0.5f, -0.5f, 1.0f // Bottom-left
        ]);

        glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 0, null);
    }

    override void unload(bool force = false) {
        vao.destroy;
        vbo.destroy;
        ebo.destroy;
    }
}
