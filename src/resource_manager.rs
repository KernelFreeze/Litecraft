use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use serde_json::Value;
use serde_json;

pub fn get_resources_path() -> &'static Path {
    Path::new("resources")
}
/// Returns an asset path
pub fn get_resource(name: &str) -> PathBuf {
    let splited = name.split(":").collect::<Vec<&str>>();
    let packet = splited[0];
    let path = splited[1];
    let resource = get_resources_path().join("assets").join(packet).join(path);
    if !resource.exists() {
        warn!("Resource {} doesn't exist", name);
    }
    resource
}
/// Read and return the content of an asset
pub fn get_resource_file(path: &Path) -> String {
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open the required asset {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut asset = String::new();
    match file.read_to_string(&mut asset) {
        Err(why) => panic!("Couldn't read the required asset {}: {}", display,
                                                   why.description()),
        Ok(_) => return asset,
    }
    asset
}

/// Get file name from Minecraft asset hash
pub fn get_file_hash(name: &str) -> String {
    let index = get_resource_file(get_resource("indexes/1.11.json").as_path());
    let data: Value = serde_json::from_str(&index).unwrap();

    let path = format!("/objects/{}/hash", name.replace("/", "~1"));
    match data.pointer(&path) {
        Some(hash) => return hash.as_str().unwrap().to_owned(),
        None        => panic!("Couldn't read the required asset {}", name),
    }
}