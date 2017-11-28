/*
 * Copyright 2017 Miguel Peláez <kernelfreeze@outlook.com>
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 */
pub extern crate allegro;
pub extern crate allegro_font;
pub extern crate allegro_image;
pub extern crate allegro_ttf;
pub extern crate allegro_primitives;
extern crate allegro_sys;
extern crate allegro_audio;
extern crate allegro_acodec;

pub mod resourcemanager;

use self::allegro::*;
use self::allegro_image::*;
use self::allegro_acodec::*;
use self::allegro_audio::*;
use self::allegro::display::{RESIZABLE, PROGRAMMABLE_PIPELINE, MAXIMIZED};
use self::allegro_primitives::PrimitivesAddon;

use scenes::scene::Scene;
use scenes::splash::SplashScreen;

use self::resourcemanager::ResourceManager;

// Versions and stuff...
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const MINECRAFT: &str = "1.14";

pub struct Client<'a> {
    core: Core,
    queue: EventQueue,
    display: &'a Display,
    resource_manager: ResourceManager<'a>,
    gui_scale: u8,
    timer: Timer,
    primitives: PrimitivesAddon,
}

impl<'a> Client<'a> {
    pub fn get_display(&self) -> &'a Display {
        self.display
    }

    pub fn get_core(&self) -> &Core {
        &self.core
    }

    pub fn get_resource_manager(&self) -> &ResourceManager {
        &self.resource_manager
    }

    pub fn get_resource_manager_mut(&mut self) -> &mut ResourceManager<'a> {
        &mut self.resource_manager
    }

    pub fn scale(&self) -> u8 {
        self.gui_scale
    }

    pub fn get_timer(&self) -> &Timer {
        &self.timer
    }

    pub fn get_primitives(&self) -> &PrimitivesAddon {
        &self.primitives
    }
}

pub fn run(session: &str) {
    let core = Core::init().unwrap();
    ImageAddon::init(&core).unwrap();

    let audio_addon = AudioAddon::init(&core).unwrap();
    AcodecAddon::init(&audio_addon).unwrap();

    let primitives = PrimitivesAddon::init(&core).unwrap();

    core.install_mouse().unwrap();
    core.install_keyboard().unwrap();

    core.set_new_display_flags(RESIZABLE | PROGRAMMABLE_PIPELINE | MAXIMIZED);

    let display = Display::new(&core, 1024, 576).unwrap();
    let timer = Timer::new(&core, 1.0 / 60.0).unwrap();

    display.set_window_title("Litecraft");

    let queue = EventQueue::new(&core).unwrap();
    queue.register_event_source(display.get_event_source());
    queue.register_event_source(timer.get_event_source());
    queue.register_event_source(core.get_mouse_event_source().unwrap());
    queue.register_event_source(core.get_keyboard_event_source().unwrap());

    let mut client = Client {
        core,
        queue,
        display: &display,
        resource_manager: ResourceManager::new(),
        gui_scale: 3u8, // 1 = very small, 2 = small, 3 = normal, 4 = big, 5 = (◉ ͜ʖ ◉)
        timer: timer,
        primitives,
    };

    ResourceManager::load(&mut client);

    let mut redraw = true;
    client.timer.start();

    let mut scene: Box<Scene> = Box::new(SplashScreen::new());

    'exit: loop {
        if redraw && client.queue.is_empty() {
            if let Some(s) = scene.draw(&mut client) {
                scene = s;
            }
            client.core.flip_display();

            redraw = false;
        }

        match client.queue.wait_for_event() {
            DisplayClose { .. } => {
                info!("Stoping Litecraft Client...");
                break 'exit;
            }
            TimerTick { .. } => redraw = true,
            DisplayResize { .. } => {
                if let Err(err) = client.display.acknowledge_resize() {
                    warn!("Error while trying to resize window: {:?}", err);
                }
            }
            MouseButtonDown { x, y, .. } => {
                info!("x{} y{}", x, y);
            }
            KeyDown { keycode, .. } => {
                match keycode {
                    KeyCode::Up => client.gui_scale += 1,
                    KeyCode::Down => {
                        if client.gui_scale > 1 {
                            client.gui_scale -= 1;
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
}