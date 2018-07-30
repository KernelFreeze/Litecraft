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

use core::resource_manager::ResourceManager;

use gfx::scene::{Scene, SceneAction::ChangeScene};
use scenes::loading::LoadingScene;

use glium::glutin::dpi::LogicalSize;
use glium::glutin::{ContextBuilder, ControlFlow, Event, EventsLoop, WindowBuilder, WindowEvent};
use glium::Display;

/// Main game struct, its role is draw and manage everything in existence
pub struct Canvas {
    window_size: LogicalSize,
    resource_manager: ResourceManager,
    display: Display,
}

impl Canvas {
    /// Create and start drawing Canvas
    pub fn start() -> Canvas {
        let mut events_loop = EventsLoop::new();
        let mut status = ControlFlow::Continue;

        // Get window size from user preferences
        let window_size = {
            let settings = settings!();
            LogicalSize::new(settings.width().into(), settings.height().into())
        };

        // Create game window
        let window = Canvas::create_window(&events_loop, window_size);

        // Create OpenGL context
        let context = ContextBuilder::new()
            .with_vsync(settings!().vsync())
            .with_depth_buffer(24);

        // Create glium display
        let display = Display::new(window, context, &events_loop);
        let display = display.expect("Failed to initialize display");

        let mut resource_manager = ResourceManager::new(&display);

        // Create default scene
        let mut scene: Box<Scene> = box LoadingScene::new(&mut resource_manager, &display);

        let mut canvas = Canvas {
            window_size,
            resource_manager,
            display,
        };

        // Main game loop
        while status != ControlFlow::Break {
            let mut target = canvas.display.draw();

            // Tick resource manager
            canvas.resource_manager.tick(&canvas.display);

            // Draw current scene
            let draw = scene.draw(&mut canvas, &mut target);

            // Change scene if requested
            if let ChangeScene(_scene) = draw {
                scene = _scene;
            }

            // Draw to window
            target.finish().expect("Couldn't render scene");

            // Check for events
            events_loop.poll_events(|events| {
                if let Event::WindowEvent { event, .. } = events {
                    status = canvas.event_handler(event);
                }
            });
        }

        // Main loop end, now dispose resources...
        info!("Stopping Litecraft...");
        canvas
    }

    /// Create a custom Window
    fn create_window(events_loop: &EventsLoop, window_size: LogicalSize) -> WindowBuilder {
        use core::constants::{LITECRAFT_VERSION, MINECRAFT_VERSION};

        let settings = settings!();

        let screen = if settings.fullscreen() {
            Some(events_loop.get_primary_monitor())
        } else {
            None
        };

        WindowBuilder::new()
            .with_min_dimensions(window_size)
            .with_title(format!("Litecraft {} {}", MINECRAFT_VERSION, LITECRAFT_VERSION))
            .with_maximized(settings.maximized())
            .with_fullscreen(screen)
    }

    /// Window events handler
    fn event_handler(&mut self, event: WindowEvent) -> ControlFlow {
        match event {
            // Window close
            WindowEvent::CloseRequested => ControlFlow::Break,

            // Window resize
            WindowEvent::Resized(size) => {
                self.window_size = size;
                ControlFlow::Continue
            },

            // Dropped file
            WindowEvent::DroppedFile(_) => ControlFlow::Continue,

            _ => ControlFlow::Continue,
        }
    }

    /// Get current window size
    pub fn size(&self) -> LogicalSize { self.window_size }

    /// Get resource manager
    pub fn resources(&self) -> &ResourceManager { &self.resource_manager }
}
