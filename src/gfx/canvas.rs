// The MIT License (MIT)
// Copyright © 2014-2018 Miguel Peláez <kernelfreeze@outlook.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation
// files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy,
// modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
// OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use core::constants::LITECRAFT_VERSION;
use core::resource_manager::ResourceManager;
use core::settings::Settings;

use gfx::scene::{Scene, SceneAction::ChangeScene};
use scenes::loading::LoadingScene;

use glium::glutin::dpi::LogicalSize;
use glium::glutin::{ContextBuilder, ControlFlow, Event, EventsLoop, WindowBuilder, WindowEvent};
use glium::Display;

use std::sync::Arc;

/// Main game struct, its role is draw and manage everything in existence
pub struct Canvas {
    resource_manager: ResourceManager,
    settings: Arc<Settings>,
}

impl Canvas {
    /// Create Canvas
    pub fn new(settings: Settings) -> Canvas {
        let settings = Arc::new(settings);

        Canvas {
            resource_manager: ResourceManager::new(Arc::clone(&settings)),
            settings,
        }
    }

    /// Create a custom Window
    fn create_window(&self, events_loop: &EventsLoop) -> WindowBuilder {
        let screen = match self.settings.fullscreen() {
            true => Some(events_loop.get_primary_monitor()),
            false => None,
        };

        let window = WindowBuilder::new()
            .with_min_dimensions(self.resource_manager.size())
            .with_title(format!("Litecraft {}", LITECRAFT_VERSION))
            .with_maximized(self.settings.maximized())
            .with_fullscreen(screen);

        window
    }

    /// Window events handler
    fn event_handler(&mut self, event: WindowEvent) -> ControlFlow {
        match event {
            // Window close
            WindowEvent::CloseRequested => ControlFlow::Break,

            // Window resize
            WindowEvent::Resized(size) => {
                self.resource_manager.set_size(size);
                ControlFlow::Continue
            },

            // Dropped file
            WindowEvent::DroppedFile(_) => ControlFlow::Continue,

            _ => ControlFlow::Continue,
        }
    }

    /// Start main game loop
    pub fn draw(&mut self) {
        let mut events_loop = EventsLoop::new();
        let mut status = ControlFlow::Continue;

        // Create game window and OpenGL context
        let window = self.create_window(&events_loop);
        let context = ContextBuilder::new()
            .with_vsync(self.settings.vsync())
            .with_depth_buffer(24);

        // Create glium display
        let display = Display::new(window, context, &events_loop);
        let display = display.expect("Failed to initialize display");

        // Create default scene
        let mut scene: Box<Scene> = box LoadingScene::new(&display);

        scene.init(&mut self.resource_manager, &display);

        // Main game loop
        while status == ControlFlow::Continue {
            let mut target = display.draw();

            // Tick resource manager
            self.resource_manager.tick(&display);

            // Draw current scene
            let draw = scene.draw(&mut self.resource_manager, &mut target, &display);

            // Change scene if requested
            if let ChangeScene(_scene) = draw {
                scene = _scene;
                scene.init(&mut self.resource_manager, &display);
            }

            // Draw to window
            target.finish().expect("Couldn't render scene");

            // Check for events
            events_loop.poll_events(|events| match events {
                Event::WindowEvent { event, .. } => status = self.event_handler(event),
                _ => (),
            });
        }

        info!("Stopping Litecraft...");
    }
}
