extern crate std;

use na::{Vector3, Point3, Translation3};
use na;

use glfw::CursorMode;
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::text::Font;

use camera::FirstPerson;
use resource_manager as res;
use block;
use version;

pub struct Client {
    camera: FirstPerson,
    window: Window,
    font: std::rc::Rc<Font>,
}

impl Client {
    #[inline]
    pub fn new() -> Client {
        Client {
            camera: FirstPerson::new(Point3::new(1.0, 1.0, 1.0), na::origin()),
            window: Window::new(&format!("Litecraft {}", version::MINECRAFT)),
            font: Font::new(&res::get_resource("litecraft:font/default.ttf"), 30),
        }
    }
    pub fn run(&mut self) {
        // Reset mouse
        self.window.glfw_window_mut().set_cursor_mode(CursorMode::Disabled);
        self.window.glfw_window_mut().set_cursor_pos((self.camera.yaw() as f64 * 1_000.0f64),
                                                     (self.camera.pitch() as f64 * 1_000.0f64));

        let mut model = block::model::Model::new("block/crafting_table").unwrap();
        let mut c = model.get_node();

        c.append_translation(&Translation3::from_vector(Vector3::new(2.0, 0.0, 0.0)));
        self.window.scene_mut().add_child(c);

        self.window.set_light(Light::StickToCamera);
        self.window.set_background_color(0.529, 0.808, 0.980);

        while !self.window.should_close() {
            /*
            self.window
                .draw_text(&format!("Litecraft {} for {}",
                                    version::VERSION,
                                    version::MINECRAFT)[..],
                           &na::origin(),
                           &self.font,
                           &Point3::new(1.0, 1.0, 1.0));
            */
            self.window.render_with_camera(&mut self.camera);
        }
    }
}
