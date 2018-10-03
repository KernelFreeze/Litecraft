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

use core::resource_manager::resource_type::ResourceType;
use core::resource_manager::ResourceManager;

use std::error;
use std::fmt;
use std::fmt::{Display, Formatter};

use std::borrow::Cow;
use std::fs::{create_dir_all, File};

use std::io::Read;
use std::io::{Error, ErrorKind};

use std::path::{Path, PathBuf};
use zip::read::ZipArchive;

type Result<T> = std::result::Result<T, Box<error::Error>>;

#[derive(PartialEq, Eq, Hash)]
/// Represents a resource URI and allows loading resource data
pub struct Resource {
    namespace: Cow<'static, str>,
    resource_type: ResourceType,
    resource_path: Option<Cow<'static, str>>,
    name: Cow<'static, str>,
}

impl Display for Resource {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[{}:{}:{}]", self.namespace, self.resource_type, self.name)
    }
}

impl Resource {
    /// Create a resource with full URI
    pub fn new<S, T>(namespace: S, name: T, resource_type: ResourceType) -> Resource
    where
        S: Into<Cow<'static, str>>,
        T: Into<Cow<'static, str>>,
    {
        Resource {
            namespace: namespace.into(),
            resource_type,
            resource_path: None,
            name: name.into(),
        }
    }

    /// Create new resource URI using Litecraft's namespace
    pub fn litecraft<S>(name: S, resource_type: ResourceType) -> Resource
    where
        S: Into<Cow<'static, str>>,
    {
        Resource::new("litecraft", name, resource_type)
    }

    /// Create new resource URI using Minecraft's namespace
    pub fn minecraft<S>(name: S, resource_type: ResourceType) -> Resource
    where
        S: Into<Cow<'static, str>>,
    {
        Resource::new("minecraft", name, resource_type)
    }

    /// Create new resource URI and set a custom path
    pub fn path<S, T, U>(namespace: S, name: T, path: U, kind: ResourceType) -> Resource
    where
        S: Into<Cow<'static, str>>,
        T: Into<Cow<'static, str>>,
        U: Into<Cow<'static, str>>,
    {
        Resource {
            namespace: namespace.into(),
            resource_type: kind,
            resource_path: Some(path.into()),
            name: name.into(),
        }
    }

    /// Create new resource URI using Litecraft's namespace and set a custom path
    pub fn litecraft_path<S, T>(name: S, path: T, resource_type: ResourceType) -> Resource
    where
        S: Into<Cow<'static, str>>,
        T: Into<Cow<'static, str>>,
    {
        Resource::path("litecraft", name, path, resource_type)
    }

    /// Create new resource URI using Minecraft's namespace and set a custom path
    pub fn minecraft_path<S, T>(name: S, path: T, resource_type: ResourceType) -> Resource
    where
        S: Into<Cow<'static, str>>,
        T: Into<Cow<'static, str>>,
    {
        Resource::path("minecraft", name, path, resource_type)
    }

    /// Get asset folder
    pub fn folder(&self, parent: &str) -> String {
        if let Some(ref resource_path) = self.resource_path {
            format!(
                "{}/{}/{}/{}/{}.{}",
                parent,                         // Ex. resources
                self.namespace,                 // Ex. minecraft
                self.resource_type.folder(),    // Ex. textures
                resource_path,                  // Ex. entity
                self.name,                      // Ex. creeper
                self.resource_type.extension(), // Ex. png
            )
        } else {
            format!(
                "{}/{}/{}/{}.{}",
                parent,                         // Ex. assets
                self.namespace,                 // Ex. litecraft
                self.resource_type.folder(),    // Ex. textures
                self.name,                      // Ex. logo
                self.resource_type.extension(), // Ex. png
            )
        }
    }

    /// Get a resource as binary
    pub fn load_binary(&self) -> Result<Vec<u8>> {
        // Create resourcepacks folder if not exists
        if !Path::new("resourcepacks").exists() {
            create_dir_all("resourcepacks")?;
        }

        let resourcepacks = ResourceManager::resourcepacks();

        // Get all enabled resource packs
        let resourcepacks = resourcepacks
            .into_iter()
            .map(|entry| PathBuf::from(format!("resourcepacks/{}.zip", entry)))
            .filter(|entry| entry.exists())
            .filter(|entry| entry.is_file());

        // Check every resource-pack
        for entry in resourcepacks {
            // Read ZIP file
            let zipfile = File::open(entry)?;
            let mut zipfile = ZipArchive::new(zipfile)?;

            // Read resource from ZIP
            let mut file = zipfile.by_name(&self.folder("assets"));

            // If file exist in zip
            if let Ok(mut file) = file {
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)?;

                return Ok(buffer);
            }
        }

        // Nothing found on any resource pack, using resources folder now...
        let path = self.folder("resources");
        let path = Path::new(&path);

        if path.exists() {
            let mut file = File::open(path)?;
            let mut buffer = Vec::new();

            file.read_to_end(&mut buffer)?;
            return Ok(buffer);
        }

        Err(box Error::new(
            ErrorKind::Other,
            format!("Resource {} does not exist on any resource pack", self),
        ))
    }

    /// Get a resource as plain test
    pub fn load(&self) -> Result<String> { String::from_utf8(self.load_binary()?).map_err(|e| e.into()) }
}
