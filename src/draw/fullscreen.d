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

module draw.fullscreen;

import gl;
import draw.drawable;
import dlib.math;
import resource_manager;
import litecraft;
import std.experimental.logger;

/// 2D Full Screen Quad
public final class FullScreenQuad : Drawable!FullScreenQuad {
    private VAO vao; // Status
    private VBO vbo; // Vertex
    private EBO ebo; // Elements

    this() {
        this._instance = this;

        this.name = "fullscreen_primitive";
        this.namespace = "litecraft";
    }

    /// Draw primitive on screen
    static void draw(Shader s) {
        if (instance is null) {
            warning("Tried to draw unloaded primitive");
            return;
        }

        if (!instance.isLoaded)
            return;

        if (!s.isLoaded)
            return;

        s.use;
        instance.vao.bind;

        s.set("uTime", time);
        s.set("uResolution", vec2(Litecraft.width, Litecraft.height));

        glDrawElements(GL_TRIANGLES, instance.ebo.size, GL_UNSIGNED_SHORT, null);
    }

    override void load() {
        // Generate and bind VAO
        vao = new VAO;

        // Generate Vertex Buffer Object
        vbo = new VBO(
        [ // positions
            1.0f, 1.0f,
            1.0f, -1.0f,
            -1.0f, -1.0f,
            -1.0f, 1.0f,
        ]);

        // Positions
        glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE, 2 * float.sizeof, cast(void*) 0);
        glEnableVertexAttribArray(0);

        // Generate Element Buffer Object
        ebo = new EBO([
            0, 1, 3, // first triangle
            1, 2, 3 // second triangle
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

/// 2D Full Screen Quad
public final class TexturedFullScreenQuad : Drawable!TexturedFullScreenQuad {
    private VAO vao; // Status
    private VBO vbo; // Vertex
    private EBO ebo; // Elements

    this() {
        this._instance = this;

        this.name = "fullscreen_textured_primitive";
        this.namespace = "litecraft";
    }

    /// Draw primitive on screen
    static void draw(Texture texture, Shader s = shader("fullquad")) {
        if (instance is null) {
            warning("Tried to draw unloaded primitive");
            return;
        }

        if (!instance.isLoaded)
            return;

        if (!texture.isLoaded)
            return;

        if (!s.isLoaded)
            return;

        texture.bind;
        s.use;
        instance.vao.bind;

        s.set("uTime", time);
        s.set("uTexture", 0);
        s.set("uResolution", vec2(Litecraft.width, Litecraft.height));

        glDrawElements(GL_TRIANGLES, instance.ebo.size, GL_UNSIGNED_SHORT, null);
    }

    override void load() {
        // Generate and bind VAO
        vao = new VAO;

        // Generate Vertex Buffer Object
        vbo = new VBO(
        [ // positions         // texture coords
            1.0f, 1.0f,          1.0f, 1.0f, // top right
            1.0f, -1.0f,         1.0f, 0.0f, // bottom right
            -1.0f, -1.0f,        0.0f, 0.0f, // bottom left
            -1.0f, 1.0f,         0.0f, 1.0f  // top left
        ]);

        // Positions
        glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE, 4 * float.sizeof, cast(void*) 0);
        glEnableVertexAttribArray(0);

        // Texture coords
        glVertexAttribPointer(1, 2, GL_FLOAT, GL_FALSE, 4 * float.sizeof,
                cast(void*)(2 * float.sizeof));
        glEnableVertexAttribArray(1);

        // Generate Element Buffer Object
        ebo = new EBO([
            0, 1, 3, // first triangle
            1, 2, 3 // second triangle
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
