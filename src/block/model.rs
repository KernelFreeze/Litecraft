use std::vec::Vec;
use std::result::Result;

use kiss3d::scene::SceneNode;
use na::{Point3, Point2, Rotation2, Vector2};
use na;
use resource_manager as res;
use serde_json;
use serde_json::Map;
use std::f32;

#[derive(Deserialize)]
pub struct Model {
    parent: Option<String>,
    ambientocclusion: Option<bool>,
    display: Option<Displays>,
    textures: Option<Map<String, String>>,
    elements: Option<Vec<Element>>,
}
#[derive(Deserialize)]
struct Displays {
    thirdperson_righthand: Option<Display>,
    thirdperson_lefthand: Option<Display>,
    firstperson_righthand: Option<Display>,
    firstperson_lefthand: Option<Display>,
    gui: Option<Display>,
    head: Option<Display>,
    ground: Option<Display>,
    fixed: Option<Display>,
}
#[derive(Deserialize)]
struct Display {
    rotation: Option<Vec<i32>>,
    translation: Option<Vec<i32>>,
    scale: Option<Vec<i32>>,
}
#[derive(Deserialize)]
struct Element {
    from: Option<Vec<f32>>,
    to: Option<Vec<f32>>,
    rotation: Option<ElementRotation>,
    shade: Option<bool>,
    faces: Option<Faces>,
}
#[derive(Deserialize)]
struct ElementRotation {
    origin: Option<Vec<i32>>,
    axis: Option<String>,
    angle: Option<i32>,
    rescale: Option<bool>,
}
#[derive(Deserialize, Clone)]
struct Faces {
    down: Option<Face>,
    up: Option<Face>,
    north: Option<Face>,
    south: Option<Face>,
    west: Option<Face>,
    east: Option<Face>,
}
#[derive(Deserialize, Clone)]
struct Face {
    uv: Option<Vec<i32>>,
    texture: Option<String>,
    cullface: Option<Option<String>>,
    rotation: Option<i32>,
    tintindex: Option<i32>,
}

impl Model {
    pub fn get_node(&mut self) -> SceneNode {
        let mut block = SceneNode::new_empty();

        self.get_parent();

        // Render elements
        if let Some(ref elements) = self.elements {
            for ele in elements {
                let f = ele.from.clone().unwrap_or(vec![0.0, 0.0, 0.0]);
                let t = ele.to.clone().unwrap_or(vec![16.0, 16.0, 16.0]);

                let no_faces = Faces {
                    down: None,
                    up: None,
                    north: None,
                    south: None,
                    east: None,
                    west: None,
                };
                let face = ele.faces.clone().unwrap_or(no_faces);

                // Up
                {
                    let vert = [Point3::new(f[0] / 16.0, t[1] / 16.0, t[2] / 16.0),
                                Point3::new(t[0] / 16.0, t[1] / 16.0, t[2] / 16.0),
                                Point3::new(f[0] / 16.0, t[1] / 16.0, f[2] / 16.0),
                                Point3::new(t[0] / 16.0, t[1] / 16.0, f[2] / 16.0)];
                    self.create_face(vert, &face.up, &mut block);
                }
                // Down
                {
                    let vert = [Point3::new(f[0] / 16.0, f[1] / 16.0, f[2] / 16.0),
                                Point3::new(t[0] / 16.0, f[1] / 16.0, f[2] / 16.0),
                                Point3::new(f[0] / 16.0, f[1] / 16.0, t[2] / 16.0),
                                Point3::new(t[0] / 16.0, f[1] / 16.0, t[2] / 16.0)];
                    self.create_face(vert, &face.down, &mut block);
                }
                // South
                {
                    let vert = [Point3::new(t[0] / 16.0, f[1] / 16.0, t[2] / 16.0),
                                Point3::new(f[0] / 16.0, f[1] / 16.0, t[2] / 16.0),
                                Point3::new(t[0] / 16.0, t[1] / 16.0, t[2] / 16.0),
                                Point3::new(f[0] / 16.0, t[1] / 16.0, t[2] / 16.0)];
                    self.create_face(vert, &face.south, &mut block);
                }
                // North
                {
                    let vert = [Point3::new(f[0] / 16.0, f[1] / 16.0, f[2] / 16.0),
                                Point3::new(t[0] / 16.0, f[1] / 16.0, f[2] / 16.0),
                                Point3::new(f[0] / 16.0, t[1] / 16.0, f[2] / 16.0),
                                Point3::new(t[0] / 16.0, t[1] / 16.0, f[2] / 16.0)];
                    self.create_face(vert, &face.north, &mut block);
                }
                // West
                {
                    let vert = [Point3::new(f[0] / 16.0, f[1] / 16.0, t[2] / 16.0),
                                Point3::new(f[0] / 16.0, f[1] / 16.0, f[2] / 16.0),
                                Point3::new(f[0] / 16.0, t[1] / 16.0, t[2] / 16.0),
                                Point3::new(f[0] / 16.0, t[1] / 16.0, f[2] / 16.0)];
                    self.create_face(vert, &face.west, &mut block);
                }
                // East
                {
                    let vert = [Point3::new(t[0] / 16.0, f[1] / 16.0, f[2] / 16.0),
                                Point3::new(t[0] / 16.0, f[1] / 16.0, t[2] / 16.0),
                                Point3::new(t[0] / 16.0, t[1] / 16.0, f[2] / 16.0),
                                Point3::new(t[0] / 16.0, t[1] / 16.0, t[2] / 16.0)];
                    self.create_face(vert, &face.east, &mut block);
                }
            }
        }
        block
    }
    pub fn new(path: &str) -> Result<Model, serde_json::Error> {
        serde_json::from_str(&res::get_resource_file(&res::get_resource(&format!("minecraft:\
                                                                                  models/{}.\
                                                                                  json",
                                                                                 path))))
    }
    fn get_parent(&mut self) {
        let mut parent = self.parent.clone();
        while let Some(px) = parent.clone() {
            match Model::new(&px) {
                Ok(p) => {
                    // Ambient Occlusion
                    if self.ambientocclusion.is_none() {
                        if p.ambientocclusion.is_some() {
                            self.ambientocclusion = p.ambientocclusion;
                        }
                    }
                    // Display
                    if let Some(ref mut display) = self.display {
                        if let Some(ds) = p.display {
                            // FirstPerson left hand
                            if display.firstperson_lefthand.is_none() {
                                if ds.firstperson_lefthand.is_some() {
                                    display.firstperson_lefthand = ds.firstperson_lefthand;
                                }
                            }
                            // FirstPerson right hand
                            if display.firstperson_righthand.is_none() {
                                if ds.firstperson_righthand.is_some() {
                                    display.firstperson_righthand = ds.firstperson_righthand;
                                }
                            }
                            // Fixed
                            if display.fixed.is_none() {
                                if ds.fixed.is_some() {
                                    display.fixed = ds.fixed;
                                }
                            }
                            // Ground
                            if display.ground.is_none() {
                                if ds.ground.is_some() {
                                    display.ground = ds.ground;
                                }
                            }
                            // GUI
                            if display.gui.is_none() {
                                if ds.gui.is_some() {
                                    display.gui = ds.gui;
                                }
                            }
                            // Head
                            if display.head.is_none() {
                                if ds.head.is_some() {
                                    display.head = ds.head;
                                }
                            }
                            // ThirdPerson left hand
                            if display.thirdperson_lefthand.is_none() {
                                if ds.thirdperson_lefthand.is_some() {
                                    display.thirdperson_lefthand = ds.thirdperson_lefthand;
                                }
                            }
                            // ThirdPerson right hand
                            if display.thirdperson_righthand.is_none() {
                                if ds.thirdperson_righthand.is_some() {
                                    display.thirdperson_righthand = ds.thirdperson_righthand;
                                }
                            }
                        }
                    } else if let Some(display) = p.display {
                        self.display = Some(display);
                    }
                    // Textures
                    if let Some(ref mut txt) = self.textures {
                        if let Some(textures) = p.textures {
                            for (k, t) in textures {
                                if !txt.contains_key(&k) {
                                    txt.insert(k, t);
                                }
                            }
                        }
                    } else {
                        self.textures = p.textures;
                    }

                    // Elements
                    if self.elements.is_none() {
                        if p.elements.is_some() {
                            self.elements = p.elements;
                        }
                    }
                    parent = p.parent;
                }
                Err(e) => {
                    println!("Error loading model: {}", e);
                    break;
                }
            }
        }
    }
    fn create_face_uv(&self,
                      vert: [Point3<f32>; 4],
                      block: &mut SceneNode,
                      uv: &Vec<Point2<f32>>,
                      texture: &str) {
        let mut face = block.add_quad_with_vertices(&vert, 2, 2);
        face.set_texture_from_file(&res::get_resource(&format!("minecraft:textures/{}.png",
                                                               texture)),
                                   get_texture_name(texture));

        face.modify_uvs(&mut |v| {
            *v = uv.to_owned();
        });
    }
    fn create_face(&self, vert: [Point3<f32>; 4], fc: &Option<Face>, block: &mut SceneNode) {
        let mut texture = "blocks/debug";
        if let &Some(ref fcc) = fc {
            let mut rotation = 0;

            // Get face texture
            if let Some(ref txt) = fcc.texture {
                if let Some(ref textures) = self.textures {
                    let text = &txt.replace("#", "");
                    if textures.contains_key(text) {
                        texture = textures.get(text).unwrap();
                    }
                }
            }
            // Get rotation
            if let Some(ref r) = fcc.rotation {
                rotation = *r;
            }
            // Get face UV
            if let Some(ref j) = fcc.uv {
                let mut uv = vec![Point2::new(j[2] as f32 / 16.0, j[3] as f32 / 16.0),
                                  Point2::new(j[0] as f32 / 16.0, j[3] as f32 / 16.0),
                                  Point2::new(j[2] as f32 / 16.0, j[1] as f32 / 16.0),
                                  Point2::new(j[0] as f32 / 16.0, j[1] as f32 / 16.0)];
                match rotation {
                    90 => {
                        uv.swap(0, 1);
                        uv.swap(1, 2);
                        uv.swap(1, 3);
                    }
                    180 => {
                        uv.swap(0, 3);
                        uv.swap(1, 2);
                    }
                    270 => {
                        uv.swap(0, 2);
                        uv.swap(1, 2);
                        uv.swap(2, 3);
                    }
                    _ => {}
                }
                self.create_face_uv(vert, block, &uv, texture);
                return;
            }
        }
        let mut face = block.add_quad_with_vertices(&vert, 2, 2);
        face.set_texture_from_file(&res::get_resource(&format!("minecraft:textures/{}.png",
                                                               texture)),
                                   get_texture_name(texture));
    }
}
fn get_texture_name(path: &str) -> &str {
    let splited = path.split(":").collect::<Vec<&str>>();
    splited.last().unwrap().to_owned()
}
