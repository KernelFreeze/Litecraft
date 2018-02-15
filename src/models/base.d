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

module models.base;

import util.jsonx;

/// Holds the different places where item models are displayed.
struct Display {
    /// Specifies the rotation of the model according to the scheme [x, y, z].
    float[] rotation;

    /**
        Specifies the position of the model according to the scheme [x, y, z].
        If the value is greater than 80, it is displayed as 80. If the value is less then
        -80, it is displayed as -80.
    */
    float[] translation;

    /**
        Specifies the scale of the model according to the scheme [x, y, z].
        If the value is greater than 4, it is displayed as 4.
    */
    float[] scale;
}

/// Holds the different places where item models are displayed.
struct DisplayCompound {
    /**
        Place where an item model is displayed. Holds its rotation, translation and
        scale for the specified situation. fixed refers to item frames, while the rest
        are as their name states. Note that translations are applied to the model before rotations.
     */
    Display thirdperson_righthand, thirdperson_lefthand, firstperson_righthand,
        firstperson_lefthand, gui, head, ground, fixed;
}

/// Defines the rotation of an element.
struct ElementRotation {
    /// Sets the center of the rotation according to the scheme [x, y, z], defaults to [8, 8, 8].
    float[] origin = [8, 8, 8];

    /// Specifies the direction of rotation, can be "x", "y" or "z".
    string axis = "z";

    /// Specifies the angle of rotation. Can be 45 through -45 degrees in 22.5 degree increments. Defaults to 0.
    float angle;

    /// Specifies whether or not to scale the faces across the whole block. Can be true or false. Defaults to false.
    bool rescale;
}

/// Contains the properties of the specified face.
struct ElementFace {
    /**
        Defines the area of the texture to use according to the scheme [x1, y1, x2, y2].
        If unset, it defaults to values equal to xyz position of the element.
        The texture behavior will be inconsistent if UV extends below 0 or above 16.
        If the numbers of x1 and x2 are swapped (e.g. from 0, 0, 16, 16 to 16, 0, 0, 16),
        the texture will be flipped. UV is optional, and if not supplied it will automatically
        generate based on the element's position.
    */
    int[] uv;

    /// Specifies the texture in form of the texture variable prepended with a #.
    string texture;

    /**
        Specifies whether a face does not need to be rendered when there is a block touching it
        in the specified position. The position can be: down, up, north, south, west, or east.
        It will also determine which side of the block to use the light level from for lighting
        the face, and if unset, defaults to the side.
    */
    string cullface;

    /**
        Rotates the texture by the specified number of degrees.
        Can be 0, 90, 180, or 270. Defaults to 0. Rotation does not affect which part of the
        texture is used. Instead, it amounts to permutation of the selected texture
        vertexes (selected implicitly, or explicitly though uv).
    */
    int rotation;

    /**
        Determines whether to tint the texture using a hardcoded tint index.
        The default is not using the tint, and any number causes it to use tint. Note that only
        certain blocks have a tint index, all others will be unaffected.
    */
    int tintindex = int.min;
}

/// Contains the properties of the specified face.
struct ElementFaces {
    /// Contains the properties of the specified face.
    ElementFace down, up, north, south, west, east;
}

/// An element
struct Element {
    /// Start point of a cube according to the scheme [x, y, z]. Values must be between -16 and 32.
    float[] from;

    /// Stop point of a cube according to the scheme [x, y, z]. Values must be between -16 and 32.
    float[] to;

    /// Defines the rotation of an element.
    ElementRotation rotation;

    /// Defines if shadows are rendered (true - default), not (false).
    bool shade;

    /// Holds all the faces of the cube. If a face is left out, it will not be rendered.
    ElementFaces faces;
}

/**
    The folder resources/minecraft/models/block holds the model files for all the specified variants.
    The names of the files can be changed, but must always correspond with the names used
    in the variant files.
*/
struct JSONModel {
    /**
        Loads a different model from the given path, starting in resources/minecraft/models.
        If both "parent" and "elements" are set, the "elements" tag overrides the "elements"
        tag from the previous model.
        Can be set to "builtin/generated" to use a model that is created out of the specified icon.
        Note that only the first layer is supported, and rotation can only be achieved using block
        states files.
    */
    string parent;

    /// Whether to use ambient occlusion (true - default), or not (false).
    bool ambientocclusion = true;

    /// Holds the different places where item models are displayed.
    DisplayCompound display;

    /**
        Contains all the elements of the model. they can only have cubic forms.
        If both "parent" and "elements" are set, the "elements" tag overrides the "elements"
        tag from the previous model.
    */
    Element[] elements;

    /**
        Holds the textures of the model. Each texture starts in resources/minecraft/textures
        or can be another texture variable.
    */
    string[string] textures;

    /// Load and parse a JSON Model
    static JSONModel load(string source) {
        return jsonDecode!JSONModel(source);
    }
}
