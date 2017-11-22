use client::Client;

pub trait Scene {
    fn draw(&self, client: &mut Client) -> Option<Box<Scene>>;
}
