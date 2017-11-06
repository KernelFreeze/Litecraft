use client::Client;
use scenes::scene::Scene;

use allegro::Color;
use client::allegro_font::FontDrawing;
use client::allegro_font::FontAlign;

pub struct SplashScreen;

impl Scene for SplashScreen {
    fn draw(&self, client: &Client) {
        client.core.clear_to_color(Color::from_rgb_f(1.0, 1.0, 1.0));
        client.core.draw_text(
            &client.font,
            Color::from_rgb_f(0.5, 0.5, 0.5),
            (client.display.get_width() / 2) as f32,
            (client.display.get_height() / 2) as f32,
            FontAlign::Centre,
            "Potato!",
        );
    }
}

impl SplashScreen {
    pub fn new() -> Self {
        info!("Starting splash screen...");
        SplashScreen {}
    }
}