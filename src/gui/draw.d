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

module gui.draw;

import derelict.imgui.imgui;
import std.string : toStringz;
import dlib.core.oop;

import gl : isKeyPressed;
import derelict.glfw3.glfw3 : GLFW_KEY_ESCAPE;
import resource_manager : Texture;

/// Draw a GUI Window
struct Window {
    /// Create a window
    this(string name) {
        igBegin(name.toStringz);
    }

    /// Create a window, and set size
    this(string name, uint w, uint h) {
        igSetNextWindowSize(ImVec2(w, h), ImGuiSetCond_FirstUseEver);
        igBegin(name.toStringz);
    }

    /// Create a window, set size and position
    this(string name, uint w, uint h, uint x, uint y) {
        igSetNextWindowSize(ImVec2(w, h), ImGuiSetCond_FirstUseEver);
        igSetNextWindowPos(ImVec2(x - (w / 2), y - (h / 2)), ImGuiSetCond_FirstUseEver);

        igBegin(name.toStringz);
    }

    /// Create a closeable window
    this(string name, bool* show) {
        igBegin(name.toStringz, show);

        if (isKeyPressed(GLFW_KEY_ESCAPE)) {
            *show = true;
        }
    }

    /// Create a closeable window
    this(string name, bool* show, uint w, uint h) {
        igSetNextWindowSize(ImVec2(w, h), ImGuiSetCond_FirstUseEver);

        igBegin(name.toStringz, show);

        if (isKeyPressed(GLFW_KEY_ESCAPE)) {
            *show = true;
        }
    }

    /// Create a closeable window, set size and position
    this(string name, bool* show, uint w, uint h, uint x, uint y) {
        igSetNextWindowSize(ImVec2(w, h), ImGuiSetCond_FirstUseEver);
        igSetNextWindowPos(ImVec2(x - (w / 2), y - (h / 2)), ImGuiSetCond_FirstUseEver);

        igBegin(name.toStringz, show);

        if (isKeyPressed(GLFW_KEY_ESCAPE)) {
            *show = true;
        }
    }

    ~this() {
        igEnd();
    }

    /// Draw FPS
    void fps() {
        igText("Application average %.3f ms/frame (%.1f FPS)",
                1000.0f / igGetIO().Framerate, igGetIO().Framerate);
    }

    /// Add text to window
    void text(string text) {
        igText(text.toStringz);
    }

    /// Show and get a button
    bool button(string text) {
        return igButton(text.toStringz);
    }

    /// Show and get full size button
    bool bigButton(string text) {
        return igButton(text.toStringz, ImVec2(igGetWindowWidth() - 40.0f, 40.0f));
    }

    /// Show a textured image
    void image(Texture texture, uint w, uint h) {
        igImage(cast(void*) texture.id, ImVec2(w, h));
    }
}

/// Invisible Window for draw elements
struct HeadlessWindow {
    /// Create window
    this(string name) {
        auto flags = ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoMove
            | ImGuiWindowFlags_NoScrollbar | ImGuiWindowFlags_NoSavedSettings
            | ImGuiWindowFlags_NoInputs;

        igBegin(name.toStringz, null, flags);
    }

    mixin Inherit!(Window);
}
