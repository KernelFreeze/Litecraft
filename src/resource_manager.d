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
import accessors;
import dlib.math;
import std.experimental.logger;
import std.string : format, chomp, toStringz;
import std.array : split;
import std.parallelism : task, taskPool;
import util;

private static Texture[string] textures;
private static AnimatedTexture[string] animated_textures;
private static Shader[string] shaders;

private static SafeQueue!Loadable loadQueue;
private static Loadable[] loadedResources;

/// Ensure we free all resources...
shared static ~this() {
    foreach (resource; loadedResources) {
        resource.unload(true);
        resource.isLoaded = false;
        resource.destroy;
    }
}

/// Do a pending load tick
void loadResources() {
    if (!loadQueue.empty) {
        Loadable resource = cast(Loadable) loadQueue.pop;

        if (auto asyncResource = cast(AsyncLoadable) resource) {
            if (!asyncResource.isPreLoaded) {
                auto t = task!preLoadResource(asyncResource);
                taskPool.put(t);

                // Request another resource load as this task is running async...
                loadResources();
                return;
            }
        }

        auto type = typeid(resource).toString.split(".")[$ - 1];
        infof("Loading %s '%s:%s'...", type, resource.namespace, resource.name);

        if (resource.isLoaded)
            resource.unload();
        resource.load();
        resource.isLoaded = true;
        loadedResources ~= resource;
    }
}

/// Add a resource to load queue
void loadResource(Loadable resource) {
    loadQueue.push(cast(shared(Loadable)) resource);
}

/// Pre-load a resource, you should call loadResource instead...
void preLoadResource(AsyncLoadable resource) {
    auto type = typeid(resource).toString.split(".")[$ - 1];
    infof("Pre-Loading %s '%s:%s'...", type, resource.namespace, resource.name);

    resource.asyncLoad();
    resource.isPreLoaded = true;

    // Add resource to the queue again, but this time will be full loaded...
    resource.loadResource;
}

/// Represents a resource that can be loaded at initialization
public abstract class Loadable {
    @Read @Write private bool _isLoaded;
    @Read @Write private string _name;
    @Read @Write private string _namespace;

    /// Load the resource
    abstract void load();

    /// Unload the resource
    abstract void unload(bool force = false);

    mixin(GenerateFieldAccessors);
}

/// Represents a resource that can be loaded async at initialization
public abstract class AsyncLoadable : Loadable {
    @Read @Write private bool _isPreLoaded;

    /// Load the resource async
    abstract void asyncLoad();

    mixin(GenerateFieldAccessors);
}

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

/// OpenGL Shader loader
public final class Shader : Loadable {
    @Read private uint _program;

    /// Create vertex and fragment shaders
    this(string name) {
        this.name = name;
        this.namespace = "litecraft";
        shaders[namespace ~ ":" ~ name] = this;
    }

    override void unload(bool force = false) {
        if (isLoaded || force) {
            infof("Unloading shader %s...", name);

            glDeleteProgram(program);
        }
    }

    override void load() {
        uint vertex = glCreateShader(GL_VERTEX_SHADER);
        uint fragment = glCreateShader(GL_FRAGMENT_SHADER);

        scope (exit) {
            glDeleteShader(vertex);
            glDeleteShader(fragment);
        }

        if (vertex == 0) {
            throw new Exception("Vertex shader creation failed");
        }

        if (fragment == 0) {
            throw new Exception("Fragment shader creation failed");
        }

        // TODO: Use resource loader (for resource pack support)
        auto vertex_source = loadResource(name ~ ".vsh", "shaders", "litecraft").chomp;
        auto fragment_source = loadResource(name ~ ".fsh", "shaders", "litecraft").chomp;

        {
            const(char*) p = vertex_source.toStringz;

            glShaderSource(vertex, 1, &p, null);
            glCompileShader(vertex);

            int success, logSize;
            glGetShaderiv(vertex, GL_COMPILE_STATUS, &success);
            glGetShaderiv(vertex, GL_INFO_LOG_LENGTH, &logSize);

            if (logSize > 1) {
                char[] log = new char[](logSize);
                glGetShaderInfoLog(vertex, logSize, null, log.ptr);
                warning("Vertex shader compiler warning: ", log);
            }

            if (success == GL_FALSE) {
                throw new Exception("Vertex shader compilation failed");
            }
        }

        {
            const(char*) p = fragment_source.toStringz;

            glShaderSource(fragment, 1, &p, null);
            glCompileShader(fragment);

            int success, logSize;
            glGetShaderiv(fragment, GL_COMPILE_STATUS, &success);
            glGetShaderiv(fragment, GL_INFO_LOG_LENGTH, &logSize);

            if (logSize > 1) {
                char[] log = new char[](logSize);
                glGetShaderInfoLog(fragment, logSize, null, log.ptr);
                warning("Fragment shader compiler warning: ", log);
            }

            if (success == GL_FALSE) {
                throw new Exception("Fragment shader compilation failed");
            }
        }

        {
            _program = glCreateProgram();

            glAttachShader(program, vertex);
            glAttachShader(program, fragment);

            glLinkProgram(program);

            int success, logSize;
            glGetProgramiv(program, GL_LINK_STATUS, &success);
            glGetProgramiv(program, GL_INFO_LOG_LENGTH, &logSize);

            if (logSize > 1) {
                char[] log = new char[](logSize);
                glGetProgramInfoLog(program, logSize, null, log.ptr);
                warning("Shader linker warning: ", log);
            }

            if (success == GL_FALSE) {
                throw new Exception("Program linking failed");
            }

            glDetachShader(program, vertex);
            glDetachShader(program, fragment);

            infof("Successfully compiled shader %s", name);
        }
    }

    /// Use shader program on this thread
    void use() {
        glUseProgram(program);
    }

    /// Set uniform with value
    void set(string u, int value) {
        glUniform1i(uniform(u), value);
    }

    /// Set uniform with value
    void set(string u, float value) {
        glUniform1f(uniform(u), value);
    }

    /// Set uniform with value
    void set(string u, mat4 value) {
        glUniformMatrix4fv(uniform(u), 1, GL_FALSE, value.arrayof.ptr);
    }

    /// Set uniform with value
    void set(string u, vec2 value) {
        glUniform2fv(uniform(u), 1, value.arrayof.ptr);
    }

    /// Get Shader uniform
    uint uniform(string u) {
        return glGetUniformLocation(program, u.toStringz);
    }

    /// Get attribute shader attribute location
    uint attribute(string a) {
        auto atr = glGetAttribLocation(program, a.toStringz);

        if (atr >= GL_MAX_VERTEX_ATTRIBS) {
            warningf("Attribute '%s' for program '%s' is invalid!", a, name);
        }
        return atr;
    }

    ~this() {
        unload();
    }

    mixin(GenerateFieldAccessors);
}

/// Get or load a shader program by name
Shader shader(string name) {
    if ((name in shaders) is null) {
        return new Shader(name);
    }

    return shaders[name];
}

/// Get or load a texture by name
Texture texture(string name) {
    if ((name in textures) is null) {
        return new Texture(name);
    }

    return textures[name];
}

/// Load a resource by name
string loadResource(string name, string type, string namespace = "minecraft") {
    import std.file : readText;

    return readText(resourcePath(name, type, namespace));
}

/// Load a binary resource by name
ubyte[] loadBinaryResource(string name, string type, string namespace = "minecraft") {
    import std.file : read;

    return cast(ubyte[]) read(resourcePath(name, type, namespace));
}

/// Get a resource path, look up on Resource Packs first
string resourcePath(string name, string type, string namespace) {
    // TODO: Look-up on resource packs
    return "resources/%s/%s/%s".format(namespace, type, name);
}
