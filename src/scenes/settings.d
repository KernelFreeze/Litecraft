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

module scenes.settings;

import scenes;
import gui;
import litecraft;
import draw;

/// Multi-Player Main Menu
public class SettingsMenu : MainMenu {
    private bool show = true;

    private bool gameplay;
    private bool video;
    private bool resourcepacks;
    private bool controls;
    private bool multiplayer;

    override void render2D() {
        drawBackground();

        {
            auto w = Window("Settings", &show, 300, 400, Litecraft.width / 2, Litecraft.height / 2);

            if (!show) {
                Litecraft.instance.scene = new MainMenu;
            }

            if (w.bigButton("Gameplay Settings..."))
                gameplay = true;
            if (w.bigButton("Video Settings..."))
                video = true;
            if (w.bigButton("Resource Packs..."))
                resourcepacks = true;
            if (w.bigButton("Controls..."))
                controls = true;
            if (w.bigButton("MultiPlayer Settings..."))
                multiplayer = true;
        }

        if (gameplay) {
            auto w = Window("Gameplay Settings...", &gameplay, 300, 400,
                    Litecraft.width / 2 + 20, Litecraft.height / 2 + 20);

            bool old_combat = config.oldCombat;
            if (w.checkbox("Old combat mechanics", &old_combat)) {
                config.oldCombat = old_combat;
            }
        }

        if (video) {
            auto w = Window("Video Settings...", &video, 300, 400,
                    Litecraft.width / 2 + 20, Litecraft.height / 2 + 20);

            int quality = config.quality;
            if (w.combo("Quality", &quality, ["Low", "Normal", "High"])) {
                config.quality = quality;
            }

            int gui = config.guiScale;
            if (w.combo("GUI Scale", &gui, ["Automatic", "XS", "Small", "Normal", "Big", "XL"])) {
                config.guiScale = config.guiScale;
            }

            float brightness = config.brightness * 100.0f;
            if (w.slider("Brightness", &brightness)) {
                config.brightness = brightness / 100.0f;
            }
            // Particles
            // Fullscreen
            // Render distance
            // Clouds
            // Vsync
        }
    }
}
