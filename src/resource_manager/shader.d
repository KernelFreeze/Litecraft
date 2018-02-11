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

module resource_manager.shader;

import gl;
import accessors;
import dlib.math;
import std.experimental.logger;
import util;
import resource_manager;
import std.string : chomp, toStringz;

private static Shader[string] shaders;

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