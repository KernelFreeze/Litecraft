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

import resource_manager;
import models.base;
import dlib.math : vec2, vec3, translationMatrix;
import gl;
import litecraft;
import std.experimental.logger;
import std.string : chomp, format;
import std.algorithm.searching : countUntil, canFind, startsWith;
import std.array : replaceFirst;

private static BlockModel[string] blockmodels;

enum Face {
    North = 0x01,
    South = 0x02,
    East = 0x04,
    West = 0x08,
    Up = 0x10,
    Down = 0x20
}

/// Internal representation of Minecraft Model ready for render
final class BlockModel : AsyncLoadable {
    private VAO vao; // Status
    private VBO vbo; // Vertex

    private float[] vertices;
    private Texture[] textures;

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

        if (j.parent && j.parent != "builtin/generated") {
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

    private float findTexture(Texture[string] textureMap, string texture) @trusted {
        if (auto tx = texture in textureMap) {
            // Add only if not added yet
            if (!textures.canFind(*tx))
                textures ~= *tx;

            if (textures.length > 10)
                throw new Exception("Too many textures for a single model!");

            return countUntil(textures, *tx);
        }
        else {
            throw new Exception("I can't find a texture in a model! " ~ texture);
        }
    }

    /// Get cullface for fragment shader
    private float parseCullface(string cullface) @safe {
        switch (cullface) {
        case "down":
            return Face.Down;
        case "up":
            return Face.Up;
        case "north":
            return Face.North;
        case "south":
            return Face.South;
        case "west":
            return Face.West;
        case "east":
            return Face.East;
        default:
            return 0;
        }
    }

    override void asyncLoad() {
        Texture[string] textureMap;
        JSONModel j = loadModelTree(name, namespace);

        foreach (key, value; j.textures) {
            if (value.startsWith("#")) {
                value = j.textures[value.replaceFirst("#", "")];
            }

            textureMap["#" ~ key] = texture(value, namespace);
        }

        foreach (element; j.elements) {
            float[] f = element.from;
            float[] t = element.to;

            if (f.length < 3)
                throw new Exception("Invalid model data, 'from' must have 3 values");

            if (t.length < 3)
                throw new Exception("Invalid model data, 'to' must have 3 values");

            t[] /= 16.0f;
            f[] /= 16.0f;

            // dfmt off
            
            // South
            if (element.faces.south != ElementFace.init) {
                auto face = element.faces.south;
                auto uv = rotateUV(face.uv, face.rotation);
                auto txn = findTexture(textureMap, face.texture);
                auto cullface = parseCullface(face.cullface ? face.cullface : "south");
                auto tint = face.tintindex != int.min ? 1 : 0;

                if (uv.length < 4) throw new Exception("Invalid model data, 'uv' must have 4 values");

                vertices ~= [
                    f[0], f[1],  t[2],  uv[0], uv[1], txn, cullface, tint,
                    t[0], f[1],  t[2],  uv[2], uv[1], txn, cullface, tint,
                    t[0], t[1],  t[2],  uv[2], uv[3], txn, cullface, tint,
                    t[0], t[1],  t[2],  uv[2], uv[3], txn, cullface, tint,
                    f[0], t[1],  t[2],  uv[0], uv[3], txn, cullface, tint,
                    f[0], f[1],  t[2],  uv[0], uv[1], txn, cullface, tint
                ];
            }
            // North
            if (element.faces.north != ElementFace.init) {
                auto face = element.faces.north;
                auto uv = rotateUV(face.uv, face.rotation);
                auto txn = findTexture(textureMap, face.texture);
                auto cullface = parseCullface(face.cullface ? face.cullface : "north");
                auto tint = face.tintindex != int.min ? 1 : 0;

                if (uv.length < 4) throw new Exception("Invalid model data, 'uv' must have 4 values");

                vertices ~= [
                    f[0], f[1], f[2],  uv[0], uv[1], txn, cullface, tint,
                    t[0], f[1], f[2],  uv[2], uv[1], txn, cullface, tint,
                    t[0], t[1], f[2],  uv[2], uv[3], txn, cullface, tint,
                    t[0], t[1], f[2],  uv[2], uv[3], txn, cullface, tint,
                    f[0], t[1], f[2],  uv[0], uv[3], txn, cullface, tint,
                    f[0], f[1], f[2],  uv[0], uv[1], txn, cullface, tint
                ];
            }
            
            // Top face
            if (element.faces.up != ElementFace.init) {
                auto face = element.faces.up;
                auto uv = rotateUV(face.uv, face.rotation);
                auto txn = findTexture(textureMap, face.texture);
                auto cullface = parseCullface(face.cullface ? face.cullface : "up");
                auto tint = face.tintindex != int.min ? 1 : 0;

                if (uv.length < 4) throw new Exception("Invalid model data, 'uv' must have 4 values");

                vertices ~= [
                    f[0],  t[1], f[2],  uv[0], uv[3], txn, cullface, tint,
                    t[0],  t[1], f[2],  uv[2], uv[3], txn, cullface, tint,
                    t[0],  t[1], t[2],  uv[2], uv[1], txn, cullface, tint,
                    t[0],  t[1], t[2],  uv[2], uv[1], txn, cullface, tint,
                    f[0],  t[1], t[2],  uv[0], uv[1], txn, cullface, tint,
                    f[0],  t[1], f[2],  uv[0], uv[3], txn, cullface, tint
                ];
            }
            
            // Bottom face
            if (element.faces.down != ElementFace.init) {
                auto face = element.faces.down;
                auto uv = rotateUV(face.uv, face.rotation);
                auto txn = findTexture(textureMap, face.texture);
                auto cullface = parseCullface(face.cullface ? face.cullface : "down");
                auto tint = face.tintindex != int.min ? 1 : 0;

                if (uv.length < 4) throw new Exception("Invalid model data, 'uv' must have 4 values");

                vertices ~= [
                    f[0], f[1], f[2],  uv[0], uv[3], txn, cullface, tint,
                    t[0], f[1], f[2],  uv[2], uv[3], txn, cullface, tint,
                    t[0], f[1], t[2],  uv[2], uv[1], txn, cullface, tint,
                    t[0], f[1], t[2],  uv[2], uv[1], txn, cullface, tint,
                    f[0], f[1], t[2],  uv[0], uv[1], txn, cullface, tint,
                    f[0], f[1], f[2],  uv[0], uv[3], txn, cullface, tint
                ];
            }
            
            // East face
            if (element.faces.east != ElementFace.init) {
                auto face = element.faces.east;
                auto uv = rotateUV(face.uv, face.rotation);
                auto txn = findTexture(textureMap, face.texture);
                auto cullface = parseCullface(face.cullface ? face.cullface : "east");
                auto tint = face.tintindex != int.min ? 1 : 0;

                if (uv.length < 4) throw new Exception("Invalid model data, 'uv' must have 4 values");

                vertices ~= [
                    t[0], t[1], t[2],  uv[2], uv[1], txn, cullface, tint,
                    t[0], t[1], f[2],  uv[2], uv[3], txn, cullface, tint,
                    t[0], f[1], f[2],  uv[0], uv[3], txn, cullface, tint,
                    t[0], f[1], f[2],  uv[0], uv[3], txn, cullface, tint,
                    t[0], f[1], t[2],  uv[0], uv[1], txn, cullface, tint,
                    t[0], t[1], t[2],  uv[2], uv[1], txn, cullface, tint
                ];
            }
            
            // West face
            if (element.faces.west != ElementFace.init) {
                auto face = element.faces.west;
                auto uv = rotateUV(face.uv, face.rotation);
                auto txn = findTexture(textureMap, face.texture);
                auto cullface = parseCullface(face.cullface ? face.cullface : "west");
                auto tint = face.tintindex != int.min ? 1 : 0;

                if (uv.length < 4) throw new Exception("Invalid model data, 'uv' must have 4 values");

                vertices ~= [
                    f[0], t[1], t[2],  uv[2], uv[1], txn, cullface, tint,
                    f[0], t[1], f[2],  uv[2], uv[3], txn, cullface, tint,
                    f[0], f[1], f[2],  uv[0], uv[3], txn, cullface, tint,
                    f[0], f[1], f[2],  uv[0], uv[3], txn, cullface, tint,
                    f[0], f[1], t[2],  uv[0], uv[1], txn, cullface, tint,
                    f[0], t[1], t[2],  uv[2], uv[1], txn, cullface, tint
                ];
            }
            // dfmt on
            infof("Using %d textures for model %s", textures.length, name);
        }
    }

    override void unload(bool force = false) {
        if (isLoaded || force) {
            infof("Unloading model '%s'", name);

            vao.destroy;
            vbo.destroy;
        }
    }

    override void load() {
        if (!isPreLoaded) {
            throw new Exception("The resource is not pre-loaded!");
        }

        // Generate and bind VAO
        vao = new VAO;

        // Generate Vertex Buffer Object
        vbo = new VBO(vertices);

        // Now, tell OpenGL how to handle our data

        // Vertex positions
        glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 8 * float.sizeof, cast(void*) 0);
        glEnableVertexAttribArray(0);

        // UVs
        glVertexAttribPointer(1, 2, GL_FLOAT, GL_FALSE, 8 * float.sizeof,
                cast(void*)(3 * float.sizeof));
        glEnableVertexAttribArray(1);

        // Texture to use
        glVertexAttribPointer(2, 1, GL_FLOAT, GL_FALSE, 8 * float.sizeof,
                cast(void*)(5 * float.sizeof));
        glEnableVertexAttribArray(2);

        // Cullface
        glVertexAttribPointer(3, 1, GL_FLOAT, GL_FALSE, 8 * float.sizeof,
                cast(void*)(6 * float.sizeof));
        glEnableVertexAttribArray(3);

        // Tint
        glVertexAttribPointer(4, 1, GL_FLOAT, GL_FALSE, 8 * float.sizeof,
                cast(void*)(7 * float.sizeof));
        glEnableVertexAttribArray(4);
    }

    /// Draw model at some location
    void draw(vec3 position, ubyte cullfaces = 0, Shader shader = shader("block")) {
        if (!isLoaded)
            return;

        if (!shader.isLoaded)
            return;

        foreach (i, texture; textures) {
            texture.bind(cast(ushort) i);
        }

        shader.use;
        vao.bind;

        if (auto camera = Litecraft.instance.scene.camera) {
            shader.set("uProjection", camera.viewMatrix);
        } else {
            shader.set("uProjection", orthoProjection);
        }

        shader.set("uTransform", translationMatrix(position));
        shader.set("uTime", time);
        shader.set("uCullface", cast(int) cullfaces);
        shader.set("uResolution", vec2(Litecraft.width, Litecraft.height));

        foreach (tx; 0 .. textures.length) {
            shader.set("uTextures[%d]".format(tx), tx);
        }
    }
}
