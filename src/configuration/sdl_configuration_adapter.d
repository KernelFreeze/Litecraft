module configuration.sdl_configuration_adapter;

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

import configuration.configuration_adapter;
import sdlang;
import std.file : FileException;
import std.experimental.logger;

/// SDL implentation of ConfigurationAdapter
public final class SDLConfigurationAdapter : ConfigurationAdapter {
    /// Load configuration from file
    this() {
        try {
            auto data = parseFile("configs/litecraft.sdl");

            _renderDistance = data.getTagValue!int("render-distance", renderDistance);
            _configVersion = data.getTagValue!int("version", configVersion);

            _difficulty = data.getTagValue!int("difficulty", difficulty);
            _quality = data.getTagValue!int("quality", quality);

            _width = data.getTagValue!int("width", width);
            _height = data.getTagValue!int("height", height);

            _mouseSensitivity = data.getTagValue!float("mouse-sensitivity", mouseSensitivity);
            _fov = data.getTagValue!float("fov", fov);
            _brightness = data.getTagValue!float("brightness", brightness);

            _guiScale = data.getTagValue!int("gui-scale", guiScale);
            _particles = data.getTagValue!int("particles", particles);
            _lastServer = data.getTagValue!string("last-server", lastServer);

            _antiAliasing = data.getTagValue!bool("anti-aliasing", antiAliasing);
            _antiAliasingLevel = data.getTagAttribute!int("anti-aliasing", "level", antiAliasingLevel);

            _oldCombat = data.getTagValue!bool("old-combat", oldCombat);
        }
        catch (FileException e) {
            warning("Generating new configuration...");
            save();
        }
    }

    /// Save configuration to file
    public void save() {
        info("Saving configuration...");

        auto data = new Tag();

        new Tag(data, null, "render-distance", [Value(renderDistance)]);
        new Tag(data, null, "version", [Value(configVersion)]);
        new Tag(data, null, "difficulty", [Value(difficulty)]);
        new Tag(data, null, "quality", [Value(quality)]);
        new Tag(data, null, "width", [Value(width)]);
        new Tag(data, null, "height", [Value(height)]);
        new Tag(data, null, "mouse-sensitivity", [Value(mouseSensitivity)]);
        new Tag(data, null, "fov", [Value(fov)]);
        new Tag(data, null, "gui-scale", [Value(guiScale)]);
        new Tag(data, null, "particles", [Value(particles)]);
        new Tag(data, null, "last-server", [Value(lastServer)]);
        new Tag(data, null, "old-combat", [Value(oldCombat)]);
        new Tag(data, null, "brightness", [Value(brightness)]);

        auto AALevelAttribute = new Attribute(null, "level", Value(antiAliasingLevel));
        new Tag(data, null, "anti-aliasing", [Value(antiAliasing)], [AALevelAttribute]);

        import std.file : exists, mkdir, write;

        if (!exists("configs")) {
            mkdir("configs");
        }

        write("configs/litecraft.sdl", data.toSDLDocument());
    }
}
