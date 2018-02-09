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
import dlib.container.queue : Queue;
import std.experimental.logger;
import std.string : format, chomp, toStringz;

private static string[string] textures;
private static Shader[string] shaders;

private static Queue!Loadable loadQueue;
private static Loadable[] loadedResources;

/// Ensure we free all resources...
shared static ~this() {
    foreach (resource; loadedResources) {
        resource.unload(true);
    }
}

/// Do a pending load tick
void loadResources() {
    if (!loadQueue.empty) {
        auto resource = loadQueue.dequeue;

        infof("Loading resource '%s'...", resource.name);

        resource.unload();
        resource.load();
        resource.isLoaded = true;
        loadedResources ~= resource;
    }
}

/// Add a resource to load queue
void loadResource(Loadable toLoad) {
    loadQueue.enqueue(toLoad);
}

/// Represents a resource that can be loaded at initialization
public abstract class Loadable {
    @Read @Write private bool _isLoaded;
    @Read @Write private string _name;

    /// Load the resource
    abstract void load();

    /// Unload the resource
    abstract void unload(bool force = false);

    mixin(GenerateFieldAccessors);
}

/// OpenGL Shader loader
public final class Shader : Loadable {
    @Read private uint _program;

    /// Create and load vertex and fragment shaders
    this(string name) {
        this.name = name;
        shaders[name] = this;
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

    ~this() {
        unload();
    }

    mixin(GenerateFieldAccessors);
}

/// Get or load a shader program by name
Shader shader(string name) {
    if (shaders[name] is null) {
        return new Shader(name);
    }

    return shaders[name];
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
