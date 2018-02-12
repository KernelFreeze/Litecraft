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

module scenes.main_menu;

import scenes.scene;
import dlib.math;
import resource_manager;
import litecraft;
import gui;
import gl : closeGame;

/// Show a fancy loading screen...
public final class MainMenu : Scene {
    private bool show = true;

    override void render3D() {

    }

    override void render2D() {
        FullScreenQuad.draw(shader("litecraft:noise"));

        auto w = Window("Litecraft", &show, 330, 390, Litecraft.width / 2, Litecraft.height / 2);

        w.button("Single-Player");
        w.button("Multi-Player");

        w.button("Options");
        
        if (w.button("Close") || !show) {
            closeGame();
        }
    }
}
