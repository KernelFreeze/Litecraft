pub extern crate allegro;
pub extern crate allegro_font;

use self::allegro::*;
use self::allegro_font::*;
use scenes::scene::Scene;
use scenes::splash::SplashScreen;

// Versions and stuff...
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const MINECRAFT: &'static str = "1.13";

// Our data struct
pub struct Client<'a> {
    pub core: Core,
    font_addon: FontAddon,
    pub queue: EventQueue,
    pub font: Font,
    pub scene: &'a (Scene + 'a),
    pub display: Box<Display>
}

pub fn run(session: &str) {
    let core = Core::init().unwrap();
    let font_addon = FontAddon::init(&core).unwrap();

    info!("Game Engine Core started!");

    let display = Box::new(Display::new(&core, 800, 600).unwrap());
    let timer = Timer::new(&core, 1.0 / 60.0).unwrap();
    let font = Font::new_builtin(&font_addon).unwrap();

    let queue = EventQueue::new(&core).unwrap();
    queue.register_event_source(display.get_event_source());
    queue.register_event_source(timer.get_event_source());

    let scene = &SplashScreen::new() as &Scene;

    let client = Client{core, font_addon, queue, font, scene, display};

    let mut redraw = true;
    timer.start();

    info!("Starting main loop!");
    'exit: loop {
        if redraw && client.queue.is_empty() {
            main_loop(&client);
            redraw = false;
        }

        match client.queue.wait_for_event() {
            DisplayClose { .. } => break 'exit,
            TimerTick { .. } => redraw = true,
            _ => (),
        }
    }
}

fn main_loop(client: &Client) {
    client.scene.draw(client);
    client.core.flip_display();
}