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
import std.experimental.logger;

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
    private EBO ebo; // Elements

    this() {
        name = "quad_primitive";
    }

    /// Draw primitive on screen
    static void draw(Plane plane, Texture texture) {
        auto i = cast(Quad) instance;

        if (!i.isLoaded) return;

        auto s = shader("quad");

        texture.bind();
        s.use;
        i.vao.bind;

        glDrawElements(GL_TRIANGLES, i.ebo.size, GL_UNSIGNED_SHORT, null);
    }

    override void load() {
        // Generate and bind VAO
        vao = new VAO;

        auto s = shader("quad");
        s.use;
        s.set("uTexture", 0);

        // Generate Vertex Buffer Object
        vbo = new VBO([
            // positions         // texture coords
            0.5f,  0.5f, 0.0f,   1.0f, 1.0f,   // top right
            0.5f, -0.5f, 0.0f,   1.0f, 0.0f,   // bottom right
            -0.5f, -0.5f, 0.0f,  0.0f, 0.0f,   // bottom left
            -0.5f,  0.5f, 0.0f,  0.0f, 1.0f    // top left
        ]);

        // Positions
        glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 5 * float.sizeof, cast(void*) 0);
        glEnableVertexAttribArray(0);

        // Texture coords
        glVertexAttribPointer(1, 2, GL_FLOAT, GL_FALSE, 5 * float.sizeof, cast(void*) (3 * float.sizeof));
        glEnableVertexAttribArray(1);

        // Generate Element Buffer Object
        ebo = new EBO([
            0, 1, 3,   // first triangle
            1, 2, 3    // second triangle
        ]);
    }

    override void unload(bool force = false) {
        if (isLoaded || force) {
            infof("Unloading geometry '%s'", name);

            vao.destroy;
            vbo.destroy;
            ebo.destroy;
        }
    }
}
