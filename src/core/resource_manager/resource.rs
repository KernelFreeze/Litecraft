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
use core::settings::Settings;

use std::fmt;
use std::fs::{create_dir_all, File};
use std::io::Read;
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};
use zip::read::ZipArchive;

#[derive(PartialEq, Eq, Hash)]
pub struct Resource {
    namespace: &'static str,
    resource_type: ResourceType,
    resource_path: Option<&'static str>,
    name: &'static str,
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}:{}]", self.namespace, self.name)
    }
}

impl Resource {
    pub fn new(namespace: &'static str, name: &'static str, resource_type: ResourceType) -> Resource {
        Resource {
            namespace,
            resource_type,
            resource_path: None,
            name,
        }
    }

    pub fn litecraft(name: &'static str, resource_type: ResourceType) -> Resource {
        Resource::new("litecraft", name, resource_type)
    }

    pub fn minecraft(name: &'static str, resource_type: ResourceType) -> Resource {
        Resource::new("minecraft", name, resource_type)
    }

    pub fn new_path(
        namespace: &'static str, name: &'static str, path: &'static str, r_type: ResourceType,
    ) -> Resource {
        Resource {
            namespace,
            resource_type: r_type,
            resource_path: Some(path),
            name,
        }
    }

    pub fn litecraft_path(
        name: &'static str, path: &'static str, resource_type: ResourceType,
    ) -> Resource {
        Resource::new_path("litecraft", name, path, resource_type)
    }

    pub fn minecraft_path(
        name: &'static str, path: &'static str, resource_type: ResourceType,
    ) -> Resource {
        Resource::new_path("minecraft", name, path, resource_type)
    }

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

    /// Get most priority file to load
    fn find(&self, settings: &Settings) -> Result<Vec<u8>> {
        if !Path::new("resourcepacks").exists() {
            create_dir_all("resourcepacks")?;
        }

        // Read from resource packs first
        let resourcepacks = settings
            .resourcepacks()
            .into_iter()
            .map(|entry| {
                let mut path = PathBuf::from("resourcepacks");
                path.push(entry);
                path
            })
            .filter(|entry| entry.exists())
            .filter(|entry| entry.is_file());

        for entry in resourcepacks {
            // Read resourcepack ZIP file
            let zipfile = File::open(entry)?;
            let mut zipfile = ZipArchive::new(zipfile)?;

            // Read resource from ZIP
            let mut file = zipfile.by_name(&self.folder("assets"));
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

        Err(Error::new(
            ErrorKind::Other,
            format!("Resource {} does not exist on any resource pack", self),
        ))
    }

    /// Get a resource as plain test
    pub fn load(&self, settings: &Settings) -> String {
        String::from_utf8(self.load_binary(settings))
            .unwrap_or_else(|_| panic!("Failed to decode required resource as UTF-8 text {}", &self))
    }

    /// Get a resource as binary
    pub fn load_binary(&self, settings: &Settings) -> Vec<u8> {
        self.find(settings)
            .unwrap_or_else(|_| panic!("Failed to load required binary resource {}", &self))
    }
}
