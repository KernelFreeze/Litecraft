use client::Client;

pub trait Scene {
    fn draw(&self, client: &Client);
}
