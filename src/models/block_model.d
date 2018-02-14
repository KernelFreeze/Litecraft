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

module models.block_model;

import resource_manager : AsyncLoadable, loadResource;
import models.base;
import std.experimental.logger;
import std.string : chomp;

private static BlockModel[string] blockmodels;

/// Internal representation of Minecraft Model ready for render
class BlockModel : AsyncLoadable {
    /// Create a Model loader, name should include "block/"
    this(string name, string namespace = "minecraft") {
        this.name = name;
        this.namespace = namespace;

        blockmodels[namespace ~ ":" ~ name] = this;
    }

    private JSONModel loadModelTree(string n, string ns) {
        infof("Loading JSON model '%s'", n);
        auto source = loadResource(n ~ ".json", "models", ns).chomp;

        // Create FSM and parse JSON
        auto j = JSONModel.load(source);

        if (j.parent && j.parent != string.init && j.parent != "builtin/generated") {
            auto p = loadModelTree(j.parent, ns);

            // Check if we should override data
            if (j.display == j.display.init) j.display = p.display;
            if (j.elements == j.elements.init) j.elements = p.elements;

            foreach (texture, value; j.textures) {
                p.textures[texture] = value;
            }

            j.textures = p.textures;
        }

        return j;
    }

    override void asyncLoad() {
        auto j = loadModelTree(name, namespace);
        
        /*foreach (element; j.elements) {

        }*/
    }

    override void unload(bool force = false) {
        if (isLoaded || force) {

        }
    }

    override void load() {
        if (!isPreLoaded) {
            throw new Exception("The resource is not pre-loaded!");
        }
    }
}
