use std::f32;
use glfw;
use glfw::{Key, MouseButton, Action, WindowEvent};
use num::Zero;
use na::{Point3, Vector2, Vector3, Matrix4, Isometry3, Perspective3, Translation3};
use na;
use kiss3d::camera::Camera;

/// First-person camera mode.
#[derive(Debug, Clone)]
pub struct FirstPerson {
    eye: Point3<f32>,
    yaw: f32,
    pitch: f32,

    yaw_step: f32,
    pitch_step: f32,
    move_step: f32,

    up_key: Option<Key>,
    down_key: Option<Key>,

    forward_key: Option<Key>,
    backward_key: Option<Key>,
    left_key: Option<Key>,
    right_key: Option<Key>,

    projection: Perspective3<f32>,
    proj_view: Matrix4<f32>,
    inverse_proj_view: Matrix4<f32>,
    last_cursor_pos: Vector2<f32>,
}

impl FirstPerson {
    /// Creates a first person camera with default sensitivity values.
    pub fn new(eye: Point3<f32>, at: Point3<f32>) -> FirstPerson {
        FirstPerson::new_with_frustrum(f32::consts::PI / 4.0, 0.1, 1024.0, eye, at)
    }

    /// Creates a new first person camera with default sensitivity values.
    pub fn new_with_frustrum(fov: f32,
                             znear: f32,
                             zfar: f32,
                             eye: Point3<f32>,
                             at: Point3<f32>)
                             -> FirstPerson {
        let mut res = FirstPerson {
            eye: Point3::new(0.0, 0.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
            yaw_step: 0.005,
            pitch_step: 0.005,
            move_step: 0.1,
            up_key: Some(Key::Space),
            down_key: Some(Key::LeftShift),
            forward_key: Some(Key::W),
            backward_key: Some(Key::S),
            left_key: Some(Key::A),
            right_key: Some(Key::D),
            projection: Perspective3::new(800.0 / 600.0, fov, znear, zfar),
            proj_view: na::zero(),
            inverse_proj_view: na::zero(),
            last_cursor_pos: na::zero(),
        };

        res.look_at(eye, at);

        res
    }

    /// Sets the translational increment per arrow press.
    ///
    /// The default value is 0.5.
    #[inline]
    pub fn set_move_step(&mut self, step: f32) {
        self.move_step = step;
    }

    /// Sets the pitch increment per mouse movement.
    ///
    /// The default value is 0.005.
    #[inline]
    pub fn set_pitch_step(&mut self, step: f32) {
        self.pitch_step = step;
    }


    /// Sets the yaw increment per mouse movement.
    ///
    /// The default value is 0.005.
    #[inline]
    pub fn set_yaw_step(&mut self, step: f32) {
        self.yaw_step = step;
    }

    /// Gets the translational increment per arrow press.
    #[inline]
    pub fn move_step(&self) -> f32 {
        self.move_step
    }

    /// Gets the pitch increment per mouse movement.
    #[inline]
    pub fn pitch_step(&self) -> f32 {
        self.pitch_step
    }

    /// Gets the yaw  increment per mouse movement.
    #[inline]
    pub fn yaw_step(&self) -> f32 {
        self.yaw_step
    }

    /// Changes the orientation and position of the camera to look at the specified point.
    pub fn look_at(&mut self, eye: Point3<f32>, at: Point3<f32>) {
        let dist = na::norm(&(eye - at));

        let pitch = ((at.y - eye.y) / dist).acos();
        let yaw = (at.z - eye.z).atan2(at.x - eye.x);

        self.eye = eye;
        self.yaw = yaw;
        self.pitch = pitch;
        self.update_projviews();
    }

    /// The point the camera is looking at.
    pub fn at(&self) -> Point3<f32> {
        let ax = self.eye.x + self.yaw.cos() * self.pitch.sin();
        let ay = self.eye.y + self.pitch.cos();
        let az = self.eye.z + self.yaw.sin() * self.pitch.sin();

        Point3::new(ax, ay, az)
    }

    fn update_restrictions(&mut self) {
        if self.pitch <= 0.01 {
            self.pitch = 0.01
        }

        let _pi: f32 = f32::consts::PI;
        if self.pitch > _pi - 0.01 {
            self.pitch = _pi - 0.01
        }
    }

    /// The movement button for up.
    pub fn up_key(&self) -> Option<Key> {
        self.up_key
    }

    /// The movement button for down.
    pub fn down_key(&self) -> Option<Key> {
        self.down_key
    }

    /// The movement button for left.
    pub fn left_key(&self) -> Option<Key> {
        self.left_key
    }

    /// The movement button for right.
    pub fn right_key(&self) -> Option<Key> {
        self.right_key
    }

    /// Set the movement button for up.
    /// Use None to disable movement in this direction.
    pub fn rebind_up_key(&mut self, new_key: Option<Key>) {
        self.up_key = new_key;
    }

    /// Set the movement button for down.
    /// Use None to disable movement in this direction.
    pub fn rebind_down_key(&mut self, new_key: Option<Key>) {
        self.down_key = new_key;
    }

    /// Set the movement button for left.
    /// Use None to disable movement in this direction.
    pub fn rebind_left_key(&mut self, new_key: Option<Key>) {
        self.left_key = new_key;
    }

    /// Set the movement button for right.
    /// Use None to disable movement in this direction.
    pub fn rebind_right_key(&mut self, new_key: Option<Key>) {
        self.right_key = new_key;
    }

    /// Disable the movement buttons for up, down, left and right.
    pub fn unbind_movement_keys(&mut self) {
        self.up_key = None;
        self.down_key = None;
        self.left_key = None;
        self.right_key = None;
    }

    #[doc(hidden)]
    pub fn handle_mouse(&mut self, dpos: &Vector2<f32>) {
        self.yaw = self.yaw + dpos.x * self.yaw_step;
        self.pitch = self.pitch + dpos.y * self.pitch_step;

        self.update_restrictions();
        self.update_projviews();
    }

    fn update_projviews(&mut self) {
        let _ = self.proj_view = *self.projection.as_matrix() *
                                 self.view_transform().to_homogeneous();
        let _ =
            self.proj_view.try_inverse().map(|inverse_proj| self.inverse_proj_view = inverse_proj);
    }

    /// The direction this camera is looking at.
    pub fn eye_dir(&self) -> Vector3<f32> {
        na::normalize(&(self.at() - self.eye))
    }

    /// The direction this camera is being moved by the keyboard keys for a given set of key states.
    pub fn move_dir(&self,
                    up: bool,
                    down: bool,
                    forward: bool,
                    backward: bool,
                    right: bool,
                    left: bool)
                    -> Vector3<f32> {

        let t = self.eye_dir();
        let upv = Vector3::new(0.0, 1.0, 0.0);
        let rightv = t.cross(&upv);
        let forwardv = upv.cross(&rightv);

        let mut movement = na::zero::<Vector3<f32>>();

        if up {
            movement += upv;
        }
        if down {
            movement -= upv;
        }
        if forward {
            movement += forwardv;
        }

        if backward {
            movement -= forwardv;
        }

        if right {
            movement += rightv
        }

        if left {
            movement -= rightv
        }

        if movement.is_zero() {
            movement
        } else {
            na::normalize(&movement)
        }
    }

    /// Get camera yaw
    #[inline]
    pub fn yaw(&mut self) -> f32 {
        self.yaw
    }

    /// Get camera pitch
    #[inline]
    pub fn pitch(&mut self) -> f32 {
        self.pitch
    }

    /// Translates in-place this camera by `t`.
    #[inline]
    pub fn translate_mut(&mut self, t: &Translation3<f32>) {
        let new_eye = t * self.eye;

        self.set_eye(new_eye);
    }

    /// Translates this camera by `t`.
    #[inline]
    pub fn translate(&self, t: &Translation3<f32>) -> FirstPerson {
        let mut res = self.clone();
        res.translate_mut(t);
        res
    }

    /// Sets the eye of this camera to `eye`.
    #[inline]
    fn set_eye(&mut self, eye: Point3<f32>) {
        self.eye = eye;
        self.update_restrictions();
        self.update_projviews();
    }
}

impl Camera for FirstPerson {
    fn clip_planes(&self) -> (f32, f32) {
        (self.projection.znear(), self.projection.zfar())
    }

    /// The camera view transformation (i-e transformation without projection).
    fn view_transform(&self) -> Isometry3<f32> {
        Isometry3::look_at_rh(&self.eye, &self.at(), &Vector3::y())
    }

    fn handle_event(&mut self, window: &glfw::Window, event: &WindowEvent) {
        match *event {
            WindowEvent::CursorPos(x, y) => {
                let curr_pos = Vector2::new(x as f32, y as f32);

                let dpos = curr_pos - self.last_cursor_pos;
                self.handle_mouse(&dpos);

                self.last_cursor_pos = curr_pos;
            }
            // WindowEvent::Scroll(_, off) => self.handle_scroll(off as f32),
            WindowEvent::FramebufferSize(w, h) => {
                self.projection.set_aspect(w as f32 / h as f32);
                self.update_projviews();
            }
            _ => {}
        }
    }

    fn eye(&self) -> Point3<f32> {
        self.eye
    }

    fn transformation(&self) -> Matrix4<f32> {
        self.proj_view
    }

    fn inverse_transformation(&self) -> Matrix4<f32> {
        self.inverse_proj_view
    }

    fn update(&mut self, window: &glfw::Window) {
        let up = check_optional_key_state(window, self.up_key, Action::Press);
        let down = check_optional_key_state(window, self.down_key, Action::Press);
        let forward = check_optional_key_state(window, self.forward_key, Action::Press);
        let backward = check_optional_key_state(window, self.backward_key, Action::Press);
        let right = check_optional_key_state(window, self.right_key, Action::Press);
        let left = check_optional_key_state(window, self.left_key, Action::Press);

        let dir = self.move_dir(up, down, forward, backward, right, left);

        let move_amount = dir * self.move_step;
        self.translate_mut(&Translation3::from_vector(move_amount));
    }
}

fn check_optional_key_state(window: &glfw::Window, key: Option<Key>, key_state: Action) -> bool {
    if let Some(actual_key) = key {
        window.get_key(actual_key) == key_state
    } else {
        false
    }
}
