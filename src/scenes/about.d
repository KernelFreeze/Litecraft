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

module scenes.about;

import scenes;
import gui;
import litecraft;
import draw;
import resource_manager;
import std.file : readText;

/// Multi-Player Main Menu
public class AboutMenu : MainMenu {
    private bool show = true;

    private bool credits;
    private bool license;
    private bool libraries;

    override void render2D() {
        drawBackground();

        {
            auto w = Window("About", &show, 300, 400, Litecraft.width / 2, Litecraft.height / 2);

            if (!show) {
                Litecraft.instance.scene = new MainMenu;
            }

            w.centeredImage(texture("logo"), 80, 80);

            if (w.bigButton("Litecraft Team"))
                credits = true;

            if (w.bigButton("License"))
                license = true;

            if (w.bigButton("Libraries"))
                libraries = true;
        }

        if (credits) {
            auto w = Window("Credits", &credits, 500, 400,
                    Litecraft.width / 2 + 20, Litecraft.height / 2 + 20);

            w.text(import("CONTRIBUTORS.md"));
        }

        if (license) {
            auto w = Window("License", &license, 500, 400,
                    Litecraft.width / 2 + 20, Litecraft.height / 2 + 20);

            w.text(import("LICENSE"));
        }
    }
}
