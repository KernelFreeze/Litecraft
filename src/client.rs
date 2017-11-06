extern crate allegro;
extern crate allegro_font;

use self::allegro::*;
use self::allegro_font::*;

// Versions and stuff...
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const MINECRAFT: &'static str = "1.13";

pub fn run(session: &str) {
    let core = Core::init().unwrap();
    let font_addon = FontAddon::init(&core).unwrap();

    info!("Game Engine Core started!");

    let display = Display::new(&core, 800, 600).unwrap();
    let timer = Timer::new(&core, 1.0 / 60.0).unwrap();
    let font = Font::new_builtin(&font_addon).unwrap();

    let queue = EventQueue::new(&core).unwrap();
    queue.register_event_source(display.get_event_source());
    queue.register_event_source(timer.get_event_source());

    let mut redraw = true;
    timer.start();
    'exit: loop {
        if redraw && queue.is_empty() {
            core.clear_to_color(Color::from_rgb_f(0.0, 0.0, 0.0));
            core.draw_text(
                &font,
                Color::from_rgb_f(1.0, 1.0, 1.0),
                (display.get_width() / 2) as f32,
                (display.get_height() / 2) as f32,
                FontAlign::Centre,
                "Potato!",
            );
            core.flip_display();
            redraw = false;
        }

        match queue.wait_for_event() {
            DisplayClose { .. } => break 'exit,
            TimerTick { .. } => redraw = true,
            _ => (),
        }
    }
}