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

module resource_manager.texture;

import gl;
import accessors;
import dlib.math;
import dlib.image;
import std.experimental.logger;
import std.string : format;
import std.array : split;
import std.parallelism : task, taskPool;
import util;
import resource_manager;

private static Texture[string] textures;

private ImageRGBA8 invertImage(SuperImage img) {
    auto res = new ImageRGBA8(img.width, img.height);

    foreach (x; 0 .. img.width)
        foreach (y; 0 .. img.height)
            res[x, y] = img[x, img.height - 1 - y];

    return res;
}

/// GPU Texture
public final class Texture : AsyncLoadable {
    @Read private uint _id;
    @Read private uint _width, _height;
    @Read private ubyte[] _data;

    /// Create a GPU texture
    this(string name, string namespace = "minecraft") {
        this.name = name;
        this.namespace = namespace;

        textures[namespace ~ ":" ~ name] = this;
    }

    override void asyncLoad() {
        auto texture = loadPNG(resourcePath(name ~ ".png", "textures", namespace));

        this._width = texture.width;
        this._height = texture.height;

        if (texture.pixelFormat == PixelFormat.RGB8 || texture.pixelFormat == PixelFormat.RGB16) {
            this._data = invertImage(texture).data;
            return;
        }

        this._data = texture.data;
    }

    override void unload(bool force = false) {
        if (isLoaded || force) {
            infof("Unloading texture %s...", name);

            glDeleteTextures(1, &_id);

            this.isPreLoaded = false;
        }
    }

    override void load() {
        if (!isPreLoaded) {
            throw new Exception("The resource is not pre-loaded!");
        }

        glGenTextures(1, &_id);
        bind();

        glPixelStorei(GL_UNPACK_ALIGNMENT, 1);

        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_BORDER);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_BORDER);

        // set texture filtering parameters
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);

        glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, width, height, 0, GL_RGBA,
                GL_UNSIGNED_BYTE, data.ptr);

        _data.destroy;
    }

    /// Make texture current
    void bind() {
        glActiveTexture(GL_TEXTURE0);
        glBindTexture(GL_TEXTURE_2D, _id);
    }

    mixin(GenerateFieldAccessors);
}

/// Get or load a texture by name
Texture texture(string name) {
    if (!(name in textures)) {
        return new Texture(name);
    }

    return textures[name];
}
