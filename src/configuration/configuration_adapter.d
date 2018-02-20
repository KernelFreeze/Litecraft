module configuration.configuration_adapter;

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

/// Customizable configuration adapter
public abstract class ConfigurationAdapter {
    @Read @Write protected int _configVersion;
    @Read @Write protected int _renderDistance = 12;
    @Read @Write protected int _difficulty = 2;
    @Read @Write protected int _quality = 2; // Low, Normal, Higth

    @Read @Write protected int _width = 800;
    @Read @Write protected int _height = 600;

    @Read @Write protected float _mouseSensitivity = 0.5f;
    @Read @Write protected float _fov = 0.5f;
    @Read @Write protected float _brightness = 0.5f;

    @Read @Write protected int _guiScale; // Auto, Smaller, Small, Normal, Big, Bigger
    @Read @Write protected int _particles; // None, Low, Medium, Higth

    @Read @Write protected string _lastServer = "";

    @Read @Write protected bool _antiAliasing = true;
    @Read @Write protected int _antiAliasingLevel = 4;

    @Read @Write protected bool _oldCombat = false;

    mixin(GenerateFieldAccessors);
}
