use client::Client;
use scenes::scene::Scene;
use scenes::gui::Component;
use client::resourcemanager::ResourceManager;
use scenes::mainmenu::MainMenu;

use allegro::Color;

pub struct SplashScreen;

impl Component for SplashScreen {}

impl Scene for SplashScreen {
    fn draw(&self, client: &mut Client) -> Option<Box<Scene>> {
        client.get_core().clear_to_color(Color::from_rgb_f(1.0, 1.0, 1.0));

        let w = client.get_display().get_width() as f32;
        let h = client.get_display().get_height() as f32;

        let x = w / 2.0;
        let y = h / 2.0;

        let color = Color::from_rgb_f(1f32, 1f32, 1f32);

        self.draw_2d(client, 0.0, 0.0, w, h, "background");
        self.draw_2d(client, x - 100.0, y - 130.0, 200.0, 200.0, "logo");
        self.draw_litecraft_text(client, color, "Starting Litecraft!", x, y + 90.0);

        if !ResourceManager::load_assets(client) {
            return Some(Box::new(MainMenu::new()));
        }

        None
    }
}

impl SplashScreen {
    pub fn new() -> Self {
        SplashScreen {}
    }
}