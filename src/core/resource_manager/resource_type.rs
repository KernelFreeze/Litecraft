// The MIT License (MIT)
// Copyright © 2014-2018 Miguel Peláez <kernelfreeze@outlook.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation
// files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy,
// modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
// OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::fmt;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum ResourceType {
    Language,
    Blockstate,
    Model,
    Sound,
    Texture,
    Animation,
    Colormap,
    Font,
    Property,
    Text,
    FragmentShader,
    VertexShader,
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ResourceType::Language => "lang",
                ResourceType::Blockstate => "blockstate",
                ResourceType::Model => "model",
                ResourceType::Sound => "sound",
                ResourceType::Font => "font",
                ResourceType::Text => "text",
                ResourceType::VertexShader => "shader",
                ResourceType::FragmentShader => "shader",
                ResourceType::Texture => "texture",
                ResourceType::Animation => "texture",
                ResourceType::Colormap => "colormap",
                ResourceType::Property => "misc",
            }
        )
    }
}

impl ResourceType {
    pub fn folder(&self) -> &str {
        match self {
            ResourceType::Language => "lang",
            ResourceType::Blockstate => "blockstates",
            ResourceType::Model => "models",
            ResourceType::Sound => "sounds",
            ResourceType::Font => "fonts",
            ResourceType::Text => "texts",
            ResourceType::VertexShader => "shaders",
            ResourceType::FragmentShader => "shaders",
            ResourceType::Texture => "textures",
            ResourceType::Animation => "textures",
            ResourceType::Colormap => "textures/colormap",
            ResourceType::Property => "textures/misc",
        }
    }

    pub fn extension(&self) -> &str {
        match self {
            ResourceType::Language => "lang",
            ResourceType::Blockstate => "json",
            ResourceType::Model => "json",
            ResourceType::Sound => "ogg",
            ResourceType::Font => "ttf",
            ResourceType::Text => "txt",
            ResourceType::VertexShader => "vsh",
            ResourceType::FragmentShader => "fsh",
            ResourceType::Texture => "png",
            ResourceType::Animation => "mcmeta",
            ResourceType::Colormap => "mcmeta",
            ResourceType::Property => "mcmeta",
        }
    }
}
