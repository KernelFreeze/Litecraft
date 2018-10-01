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

use core::resource_manager::resource::Resource;
use core::resource_manager::resource_type::ResourceType;
use core::resource_manager::ResourceManager;
use core::settings::Settings;

use scenes::loading::LoadingScene;

use gfx::fxaa::{self, FxaaSystem};
use gfx::scene::{Scene, SceneAction::ChangeScene};

use glium::glutin::{ContextBuilder, ControlFlow, Event, EventsLoop, WindowBuilder, WindowEvent};
use glium::{Display, Surface};

use conrod::backend::glium::Renderer;
use conrod::{Ui, UiBuilder};

use rhai::Engine;

/// Main game struct, its role is draw and manage everything in existence
pub struct Canvas {
    resource_manager: ResourceManager,
    display: Display,
    settings: Settings,
    engine: Engine,
    ui: Ui,
}

impl Canvas {
    /// Create and start drawing Canvas
    pub fn start() {
        use core::settings_manager::load_config;

        let mut events_loop = EventsLoop::new();

        // Default action: Keep running
        let mut status = ControlFlow::Continue;

        // Load settings file
        let settings = load_config();

        // Create game window
        let window = Canvas::create_window(&settings, &events_loop);

        // Create OpenGL context
        let context = ContextBuilder::new()
            .with_vsync(settings.vsync())
            .with_multisampling(settings.multisampling())
            .with_depth_buffer(24);

        // Create glium display
        let display = Display::new(window, context, &events_loop);
        let display = display.expect("Failed to initialize display");

        // Create UI Manager
        let mut ui = UiBuilder::new([settings.width().into(), settings.height().into()]).build();

        // Load default font
        ui.fonts.insert(
            ResourceManager::font(&Resource::litecraft("default", ResourceType::Font))
                .expect("Failed to load default font file"),
        );

        // Conrod surface renderer
        let mut renderer = Renderer::new(&display).expect("Failed to initialize user interface manager");

        // Assets and resources manager
        let resource_manager = ResourceManager::new(&display, &settings);

        // Create default scene
        let mut scene: Box<Scene> = Box::new(LoadingScene::new());

        info!("Starting script engine!");

        // Rhai engine
        let engine = Engine::new();

        // FXAA
        let fxaa = FxaaSystem::new(&display);

        // Create canvas manager
        let mut canvas = Canvas {
            resource_manager,
            settings,
            display,
            engine,
            ui,
        };

        // Load initial scene resources
        scene.load(&mut canvas);

        // Main game loop
        while status != ControlFlow::Break {
            // Check for events
            events_loop.poll_events(|events| {
                use conrod::backend::winit::convert_event;

                if let Some(event) = convert_event(events.clone(), &canvas.display) {
                    canvas.ui.handle_event(event);
                }

                if let Event::WindowEvent { event, .. } = events {
                    status = canvas.event_handler(&event);
                }
            });

            let mut target = canvas.display.draw();

            // Tick resource manager
            canvas.resource_manager.tick(&canvas.display);

            // Draw user interface
            if let Some(primitives) = canvas.ui.draw_if_changed() {
                renderer.fill(
                    &canvas.display,
                    primitives,
                    &canvas.resources().textures().image_map(),
                );
            }

            // Anti-aliasing
            fxaa::draw(&fxaa, &mut target, canvas.settings.anti_aliasing(), |target| {
                // Clear buffers
                target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);

                // Draw current scene
                let draw = scene.draw(&mut canvas, target);

                // Change scene if requested
                if let ChangeScene(new_scene) = draw {
                    scene = new_scene;
                    scene.load(&mut canvas);
                }
            });

            // Render user interface surface
            // We don't use FXAA here because it looks ugly
            renderer
                .draw(
                    &canvas.display,
                    &mut target,
                    &canvas.resources().textures().image_map(),
                )
                .expect("Couldn't draw UI");

            // Draw to window
            target.finish().expect("Couldn't render scene");
        }

        // Main loop end, now dispose resources...
        info!("Stopping Litecraft...");
    }

    /// Create a custom Window
    fn create_window(settings: &Settings, events_loop: &EventsLoop) -> WindowBuilder {
        use core::constants::{LITECRAFT_VERSION, MINECRAFT_VERSION};
        use glium::glutin::Icon;

        // If user wants fullscreen get primary monitor and attach Litecraft to it
        let screen = if settings.fullscreen() {
            Some(events_loop.get_primary_monitor())
        } else {
            None
        };

        let icon = Resource::litecraft("logo", ResourceType::Texture)
            .load_binary()
            .ok()
            .and_then(|logo| Icon::from_bytes(&logo).ok());

        // Create or window
        WindowBuilder::new()
            .with_min_dimensions((settings.width(), settings.height()).into())
            .with_title(format!("Litecraft {} {}", MINECRAFT_VERSION, LITECRAFT_VERSION))
            .with_window_icon(icon)
            .with_maximized(settings.maximized())
            .with_fullscreen(screen)
    }

    /// Window events handler
    fn event_handler(&mut self, event: &WindowEvent) -> ControlFlow {
        match event {
            // Window close
            WindowEvent::CloseRequested => ControlFlow::Break,

            // Window resize
            WindowEvent::Resized(size) => {
                self.settings.set_width(size.width as u32);
                self.settings.set_height(size.height as u32);

                ControlFlow::Continue
            },

            // Dropped file
            // TODO: Allow drop resourcepacks
            WindowEvent::DroppedFile(_) => ControlFlow::Continue,

            _ => ControlFlow::Continue,
        }
    }

    /// Get resource manager
    pub fn resources(&self) -> &ResourceManager { &self.resource_manager }

    /// Get resource manager
    pub fn resources_mut(&mut self) -> &mut ResourceManager { &mut self.resource_manager }

    /// Get display manager
    pub fn display(&self) -> &Display { &self.display }

    /// Get settings
    pub fn settings(&self) -> &Settings { &self.settings }

    /// Get user interface manager
    pub fn ui(&self) -> &Ui { &self.ui }

    /// Get user interface manager
    pub fn ui_mut(&mut self) -> &mut Ui { &mut self.ui }
}
