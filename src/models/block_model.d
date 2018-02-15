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
import dlib.math : vec2;

private static BlockModel[string] blockmodels;

/// Internal representation of Minecraft Model ready for render
class BlockModel : AsyncLoadable {
    private float[] vertices;

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
            if (j.display == j.display.init)
                j.display = p.display;

            if (j.elements == j.elements.init)
                j.elements = p.elements;

            foreach (texture, value; j.textures) {
                p.textures[texture] = value;
            }

            j.textures = p.textures;
        }

        return j;
    }

    private float[] rotateUV(int[] uv, int rotation) @safe {
        import std.algorithm.mutation : swap;

        if (uv.length == 0) {
            uv = [0, 0, 16, 16];
        }

        uv[] /= 16;

        switch (rotation) {
        case 90:
            swap(uv[0], uv[1]);
            swap(uv[1], uv[2]);
            swap(uv[1], uv[3]);
            break;
        case 180:
            swap(uv[0], uv[3]);
            swap(uv[1], uv[2]);
            break;
        case 270:
            swap(uv[0], uv[2]);
            swap(uv[1], uv[2]);
            swap(uv[2], uv[3]);
            break;
        case 0:
            break;
        default:
            warningf("Invalid rotation '%d' found for element UV.", rotation);
            break;
        }

        return cast(float[]) uv;
    }

    override void asyncLoad() {
        auto j = loadModelTree(name, namespace);

        foreach (element; j.elements) {
            float[] f = element.from;
            float[] t = element.to;

            if (f.length < 3) throw new Exception("Invalid model data, 'from' must have 3 values");
            if (t.length < 3) throw new Exception("Invalid model data, 'to' must have 3 values");

            t[] /= 16.0f;
            f[] /= 16.0f;

            // dfmt off
            
            // South
            if (element.faces.south != ElementFace.init) {
                auto uv = rotateUV(element.faces.south.uv, element.faces.south.rotation);
                if (uv.length < 4) throw new Exception("Invalid model data, 'uv' must have 4 values");

                vertices ~= [
                    f[0], f[1],  t[2],  uv[0], uv[1],
                    t[0], f[1],  t[2],  uv[2], uv[1],
                    t[0], t[1],  t[2],  uv[2], uv[3],
                    t[0], t[1],  t[2],  uv[2], uv[3],
                    f[0], t[1],  t[2],  uv[0], uv[3],
                    f[0], f[1],  t[2],  uv[0], uv[1]
                ];
            }
            // North
            if (element.faces.north != ElementFace.init) {
                auto uv = rotateUV(element.faces.north.uv, element.faces.north.rotation);
                if (uv.length < 4) throw new Exception("Invalid model data, 'uv' must have 4 values");

                vertices ~= [
                    f[0], f[1], f[2],  uv[0], uv[1],
                    t[0], f[1], f[2],  uv[2], uv[1],
                    t[0], t[1], f[2],  uv[2], uv[3],
                    t[0], t[1], f[2],  uv[2], uv[3],
                    f[0], t[1], f[2],  uv[0], uv[3],
                    f[0], f[1], f[2],  uv[0], uv[1]
                ];
            }
            
            // Top face
            if (element.faces.up != ElementFace.init) {
                auto uv = rotateUV(element.faces.up.uv, element.faces.up.rotation);
                if (uv.length < 4) throw new Exception("Invalid model data, 'uv' must have 4 values");

                vertices ~= [
                    f[0],  t[1], f[2],  uv[0], uv[3],
                    t[0],  t[1], f[2],  uv[2], uv[3],
                    t[0],  t[1], t[2],  uv[2], uv[1],
                    t[0],  t[1], t[2],  uv[2], uv[1],
                    f[0],  t[1], t[2],  uv[0], uv[1],
                    f[0],  t[1], f[2],  uv[0], uv[3]
                ];
            }
            
            // Bottom face
            if (element.faces.down != ElementFace.init) {
                auto uv = rotateUV(element.faces.down.uv, element.faces.down.rotation);
                if (uv.length < 4) throw new Exception("Invalid model data, 'uv' must have 4 values");

                vertices ~= [
                    f[0], f[1], f[2],  uv[0], uv[3],
                    t[0], f[1], f[2],  uv[2], uv[3],
                    t[0], f[1], t[2],  uv[2], uv[1],
                    t[0], f[1], t[2],  uv[2], uv[1],
                    f[0], f[1], t[2],  uv[0], uv[1],
                    f[0], f[1], f[2],  uv[0], uv[3]
                ];
            }
            
            // East face
            if (element.faces.east != ElementFace.init) {
                auto uv = rotateUV(element.faces.east.uv, element.faces.east.rotation);
                if (uv.length < 4) throw new Exception("Invalid model data, 'uv' must have 4 values");

                vertices ~= [
                    t[0], t[1], t[2],  uv[2], uv[1],
                    t[0], t[1], f[2],  uv[2], uv[3],
                    t[0], f[1], f[2],  uv[0], uv[3],
                    t[0], f[1], f[2],  uv[0], uv[3],
                    t[0], f[1], t[2],  uv[0], uv[1],
                    t[0], t[1], t[2],  uv[2], uv[1]
                ];
            }
            
            // West face
            if (element.faces.west != ElementFace.init) {
                auto uv = rotateUV(element.faces.west.uv, element.faces.west.rotation);
                if (uv.length < 4) throw new Exception("Invalid model data, 'uv' must have 4 values");

                vertices ~= [
                    f[0], t[1], t[2],  uv[2], uv[1],
                    f[0], t[1], f[2],  uv[2], uv[3],
                    f[0], f[1], f[2],  uv[0], uv[3],
                    f[0], f[1], f[2],  uv[0], uv[3],
                    f[0], f[1], t[2],  uv[0], uv[1],
                    f[0], t[1], t[2],  uv[2], uv[1]
                ];
            }
             // dfmt on
        }
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
