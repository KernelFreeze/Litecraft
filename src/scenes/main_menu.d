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

import scenes;
import dlib.math;
import resource_manager;
import litecraft;
import gui;
import draw;
import gl : closeGame, time;
import camera;
import models.block_model : blockmodel;

/// Show a fancy loading screen...
public class MainMenu : Scene {
    protected void drawBackground() {
        switch (cast(uint) (time() / 10 % 7)) {
            case 0:
                TexturedFullScreenQuad.draw(texture("menu_1"), shader("blur"));
                break;
            case 1:
                TexturedFullScreenQuad.draw(texture("menu_2"), shader("blur"));
                break;
            case 2:
                TexturedFullScreenQuad.draw(texture("menu_3"), shader("blur"));
                break;
            case 3:
                TexturedFullScreenQuad.draw(texture("menu_4"), shader("blur"));
                break;
            case 4:
                TexturedFullScreenQuad.draw(texture("menu_5"), shader("blur"));
                break;
            case 5:
                TexturedFullScreenQuad.draw(texture("menu_6"), shader("blur"));
                break;
            default:
                TexturedFullScreenQuad.draw(texture("menu_7"), shader("blur"));
                break;
        }
    }

    override void render2D() {
        drawBackground();

        auto w = Window("Litecraft", 0, 0, Litecraft.width / 2, Litecraft.height / 2);

        w.centeredImage(texture("logo"), 80, 80);

        // TODO: Translations

        w.bigButton("Singleplayer");

        if (w.bigButton("Multiplayer")) {
            Litecraft.instance.scene = new MultiPlayerMenu;
        }

        w.bigButton("Options...");

        if (w.bigButton("Quit Game")) {
            closeGame();
        }
    }
}
