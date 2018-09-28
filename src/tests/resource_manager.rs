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

use core::resource_manager::resource::Resource;
use core::resource_manager::resource_type::ResourceType;

#[test]
fn resource_path_litecraft() {
    let resource = Resource::litecraft("logo", ResourceType::Texture);

    assert_eq!(
        resource.folder("resources"),
        "resources/litecraft/textures/logo.png"
    );
}

#[test]
fn resource_load_logo() {
    let resource = Resource::litecraft("logo", ResourceType::Texture);
    resource.load_binary().unwrap();

    assert_eq!(
        resource.folder("resources"),
        "resources/litecraft/textures/logo.png"
    );
}

#[test]
#[should_panic]
fn resource_load_as_text_logo() {
    let resource = Resource::litecraft("logo", ResourceType::Texture);
    resource.load().unwrap();

    assert_eq!(
        resource.folder("resources"),
        "resources/litecraft/textures/logo.png"
    );
}

#[test]
fn resource_path_minecraft() {
    let resource = Resource::minecraft_path("creeper", "entity", ResourceType::Texture);

    assert_eq!(
        resource.folder("assets"),
        "assets/minecraft/textures/entity/creeper.png"
    );
}

#[test]
#[should_panic]
fn resource_not_found() {
    let resource = Resource::litecraft("panic", ResourceType::Text);
    resource.load().unwrap();
}

#[test]
#[should_panic]
fn resource_binary_not_found() {
    let resource = Resource::litecraft("panic", ResourceType::Text);
    resource.load_binary().unwrap();
}
