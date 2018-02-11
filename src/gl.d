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

import accessors;
import derelict.opengl;
import derelict.glfw3.glfw3;
import draw;
import litecraft;
import configuration;
import std.experimental.logger;
import std.string : toStringz, format;
import std.conv : to;
import dlib.math : mat4, orthoMatrix;

mixin glFreeFuncs!(GLVersion.gl33);

private GLFWwindow* window;

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

/**
    Element Buffer Object
*/
final class EBO {
    @Read private uint _id;
    @Read private uint _size;

    /// Ask the GPU to generate a new VBO
    this(ushort[] element_buffer_data) {
        _size = cast(uint) element_buffer_data.length;

        glGenBuffers(1, &_id);
        bind();

        // Send buffer data to GPU
        glBufferData(GL_ELEMENT_ARRAY_BUFFER, ushort.sizeof * element_buffer_data.length,
                element_buffer_data.ptr, GL_STATIC_DRAW);

        element_buffer_data.destroy;
    }

    ~this() {
        glDeleteBuffers(1, &_id);
    }

    /// Bind VBO to current stack
    void bind() {
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, _id);
    }

    /// Unbind VBO from current stack
    void unbind() {
        glBindVertexArray(0);
    }

    mixin(GenerateFieldAccessors);
}

/**
    GPU Vertex Buffer Object:

    OpenGL feature that provides methods for uploading vertex data
    (position, normal vector, color, etc.) to the video device
    for non-immediate-mode rendering.
*/
final class VBO {
    @Read private uint _id;

    /// Ask the GPU to generate a new VBO
    this(float[] vertex_buffer_data) {
        glGenBuffers(1, &_id);
        bind();

        // Send buffer data to GPU
        glBufferData(GL_ARRAY_BUFFER, float.sizeof * vertex_buffer_data.length,
                vertex_buffer_data.ptr, GL_STATIC_DRAW);
        vertex_buffer_data.destroy;
    }

    ~this() {
        glDeleteBuffers(1, &_id);
    }

    /// Bind VBO to current stack
    void bind() {
        glBindBuffer(GL_ARRAY_BUFFER, _id);
    }

    /// Unbind VBO from current stack
    void unbind() {
        glBindVertexArray(0);
    }

    mixin(GenerateFieldAccessors);
}

private void init() {
    glViewport(0, 0, Litecraft.width, Litecraft.height);

    // Ensure we can capture the escape key being pressed below
    glfwSetInputMode(window, GLFW_STICKY_KEYS, GL_TRUE);

    showPointer();

    // White background
    glClearColor(1.0f, 1.0f, 1.0f, 1.0f);

    glEnable(GL_DEPTH_TEST);
    glDepthFunc(GL_LESS);
    glEnable(GL_CULL_FACE);

    // Enable transparency
    glEnable(GL_BLEND);
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
}

/// Get time for use on rendering stuff
public auto time() {
    return cast(float) glfwGetTime();
}

/// Center and hide mouse pointer
public void hidePointer() {
    // Hide the mouse and enable unlimited movement
    glfwSetInputMode(window, GLFW_CURSOR, GLFW_CURSOR_DISABLED);

    // Set the mouse at the center of the screen
    glfwSetCursorPos(window, Litecraft.instance.width / 2, Litecraft.instance.height / 2);
}

/// Display mouse pointer again
public void showPointer() {
    glfwSetInputMode(window, GLFW_CURSOR, GLFW_CURSOR_NORMAL);
}

private void display() {
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

    // 3D
    glEnable(GL_DEPTH_TEST);
    Litecraft.instance.scene.render3D();

    // 2D
    glDisable(GL_DEPTH_TEST);

    Litecraft.instance.scene.render2D();
}

/// Get orthographic projection for 2D rendering
public mat4 orthoProjection() {
    //return orthoMatrix(0.0f, Litecraft.width, 0.0f, Litecraft.height, 0.0f, 1.0f);
    return orthoMatrix(-1.0f, 1.0f, -1.0f, 1.0f, 0.0f, 1.0f);
}

/// Free all resources used by GLFW; Don't call this from a callback!
private void close() nothrow {
    try {
        glfwTerminate();

        info("Released GLFW resources!");
    }
    catch (Exception e) {
    }
}

/// Initialize and load Litecraft engine
void load() {
    DerelictGL3.load();
    DerelictGLFW3.load();

    if (!glfwInit()) {
        error("I can't start GLFW, please upgrade your GPU drivers");
        return;
    }

    // Use OpenGL 3.3
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);

    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

    version (OSX) {
        glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GL_TRUE);
    }

    static auto displayname = "Litecraft %s-%s".format(Litecraft.minecraft, Litecraft.litecraft);

    window = glfwCreateWindow(Litecraft.instance.width,
            Litecraft.instance.height, displayname.toStringz, null, null);

    scope (failure) {
        info("There was an unexpected error, we are sorry for the inconvenience :(");
    }

    scope (exit) {
        close();
    }

    if (window is null) {
        error("I can't open Game Window. Please upgrade your Graphic card drivers");
        return;
    }

    glfwMakeContextCurrent(window);

    if (DerelictGL3.reload() < GLVersion.gl33) {
        error("Your OpenGL version is too low! Please upgrade your Graphic card drivers");
        return;
    }

    Litecraft.opengl = to!(string)(glGetString(GL_VERSION));
    Litecraft.glVendor = to!(string)(glGetString(GL_RENDERER));

    {
        import std.compiler : name;

        infof("Running on modern OpenGL %s using %s", Litecraft.opengl, Litecraft.glVendor);
        infof("%s, compiled at %s, using %s", displayname, __TIMESTAMP__, name);
    }

    init();
    register();

    while (!glfwWindowShouldClose(window)) {
        import resource_manager : loadResources;

        // Do a load tick
        loadResources();
        display();

        // Swap buffers
        glfwSwapBuffers(window);
        glfwPollEvents();

        // Check any GPU error
        auto err = glGetError();
        if (err != GL_NO_ERROR) {
            throw new Exception("OpenGL Error: 0x%04x".format(err));
        }
    }

    info("Shutting down Litecraft...");
}

private void register() {
    glfwSetWindowSizeCallback(window, &resizeWindow);
    glfwSetCursorPosCallback(window, &mouseMove);
    glfwSetMouseButtonCallback(window, &mouseClick);
    glfwSetKeyCallback(window, &keyTrigger);
}

extern (C) {
    private void resizeWindow(GLFWwindow* window, int w, int h) nothrow {
        glViewport(0, 0, w, h);

        Litecraft.instance.configuration.width = w;
        Litecraft.instance.configuration.height = h;
    }

    private void mouseMove(GLFWwindow* window, double x, double y) nothrow {
    }

    private void mouseClick(GLFWwindow* window, int button, int action, int mods) nothrow {
    }

    private void keyTrigger(GLFWwindow* window, int key, int scancode, int action, int mods) nothrow {
        if (action != GLFW_PRESS) {
            return;
        }

        if (key == GLFW_KEY_ESCAPE) {

        }
    }
}
