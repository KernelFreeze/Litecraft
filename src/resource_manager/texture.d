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
import std.experimental.logger;
import std.string : format;
import std.array : split;
import std.parallelism : task, taskPool;
import util;
import resource_manager;

private static Texture[string] textures;
private static AnimatedTexture[string] animated_textures;

/// GPU Texture
public final class Texture : AsyncLoadable {
    import dlib.image;

    @Read private uint _id;
    @Read private ubyte[] _data;
    @Read uint _width, _height;

    private uint internalFormat, format;

    /// Create a GPU texture
    this(string name, string namespace = "minecraft") {
        this.name = name;
        this.namespace = namespace;

        textures[namespace ~ ":" ~ name] = this;
    }

    override void asyncLoad() {
        auto texture = loadPNG(resourcePath(name ~ ".png", "textures", namespace));
        this._data = texture.data;

        this._width = texture.width;
        this._height = texture.height;

        switch (texture.pixelFormat) {
        case PixelFormat.RGB8:
            internalFormat = GL_RGB8;
            format = GL_RGB;

            info("Loading 8 bits RGB texture");
            break;
        case PixelFormat.RGBA8:
            internalFormat = GL_RGBA8;
            format = GL_RGBA;

            info("Loading 8 bits RGBA texture");
            break;
        case PixelFormat.RGB16:
            internalFormat = GL_RGB16;
            format = GL_RGB;

            info("Loading 16 bits RGB texture");
            break;
        case PixelFormat.RGBA_FLOAT:
        case PixelFormat.RGBA16:
            internalFormat = GL_RGBA16;
            format = GL_RGBA;

            info("Loading 16 bits RGBA texture");
            break;
        default:
            throw new Exception("Unsupported PNG image format");
        }
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

        glTexImage2D(GL_TEXTURE_2D, 0, internalFormat, width,
                height, 0, format, GL_UNSIGNED_BYTE, data.ptr);
    }

    void bind() {
        glActiveTexture(GL_TEXTURE0);
        glBindTexture(GL_TEXTURE_2D, id);
    }

    mixin(GenerateFieldAccessors);
}

/// Animated GPU Texture
public final class AnimatedTexture : Loadable {
    import dlib.image;

    @Read private uint[] _ids;

    /// Create a GPU texture
    this(string name, string namespace = "minecraft") {
        this.name = name;
        this.namespace = namespace;

        animated_textures[namespace ~ ":" ~ name] = this;
    }

    override void unload(bool force = false) {
        if (isLoaded || force) {
            infof("Unloading animated texture %s...", name);

            glDeleteTextures(cast(int) ids.length, _ids.ptr);
        }
    }

    override void load() {
        auto texture = loadAPNG(resourcePath(name ~ ".apng", "textures", namespace));

        info("Loading animated texture...");

        for (int i = 0; i < texture.frameSize; i++) {
            auto data = texture.data;
            uint frame;

            glGenTextures(1, &frame);
            glBindTexture(GL_TEXTURE_2D, frame);

            // Send frame to GPU
            glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, texture.width,
                    texture.height, 0, GL_RGBA, GL_UNSIGNED_BYTE, data.ptr);

            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);

            _ids ~= frame;
            texture.advanceFrame;
        }

        infof("Loaded %d frames for %s", ids.length, name);
    }

    // TODO: Bind

    mixin(GenerateFieldAccessors);
}

/// Get or load a texture by name
Texture texture(string name) {
    if ((name in textures) is null) {
        return new Texture(name);
    }

    return textures[name];
}