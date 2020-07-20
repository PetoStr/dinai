//! A wrapper for SDL2 library.

use std::collections::HashSet;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

/// A config that specifies window constants.
pub struct WindowConfig {
    /// Title of the window.
    pub title: &'static str,

    /// Width of the window.
    pub width: u32,

    /// Height of the window.
    pub height: u32,
}

/// A custom window wrapper for the game.
///
/// # Examples
///
/// ```
/// use dinai::window::{GameWindow, WindowConfig};
///
/// let config = WindowConfig {
///     title: "Title",
///     width: 1280,
///     height: 720,
/// };
///
/// let game_window = GameWindow::new(config).unwrap();
/// ```
pub struct GameWindow {
    config: WindowConfig,
    canvas: Canvas<Window>,
    event_pump: EventPump,
    pressed_keys: HashSet<Keycode>,
    should_close: bool,
}

impl GameWindow {
    /// Creates a new window with given [`WindowConfig`]. This window will
    /// appear right after calling this method. No other steps are required in
    /// order to render on the canvas or poll events.
    ///
    /// [`WindowConfig`]: struct.WindowConfig.html
    pub fn new(config: WindowConfig) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window(config.title, config.width, config.height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())?;

        let event_pump = sdl_context.event_pump()?;

        Ok(GameWindow {
            config,
            canvas,
            event_pump,
            pressed_keys: HashSet::new(),
            should_close: false,
        })
    }

    /// Poll the `SDL2` events and handle them.
    pub fn poll(&mut self) {
        let events = self.event_pump
            .poll_iter()
            .collect::<Vec<_>>();

        for event in events {
            match event {
                Event::Quit { .. } => self.should_close = true,
                Event::KeyDown {
                    keycode: Some(key_code),
                    ..
                } => { self.pressed_keys.insert(key_code); },
                Event::KeyUp {
                    keycode: Some(key_code),
                    ..
                } => { self.pressed_keys.remove(&key_code); },
                _ => {},
            };
        }
    }

    /// Checks whether the given key is pressed.
    pub fn is_key_pressed(&self, key_code: &Keycode) -> bool {
        self.pressed_keys.contains(key_code)
    }

    /// Returns true when a quit event has been received.
    pub fn should_close(&self) -> bool {
        self.should_close
    }

    /// Returns a mutable reference to [`EventPump`] of the current `SDL2`
    /// context.
    ///
    /// [`EventPump`]: ../../sdl2/struct.EventPump.html
    pub fn event_pump_mut(&mut self) -> &mut EventPump {
        &mut self.event_pump
    }

    /// Returns a reference to [`WindowConfig`] with which this window was
    /// created.
    ///
    /// [`WindowConfig`]: struct.WindowConfig.html
    pub fn config(&self) -> &WindowConfig {
        &self.config
    }

    /// Returns a mutable reference to current [`Canvas`] of `SDL2` [`Window`].
    ///
    /// [`Canvas`]: ../../sdl2/render/struct.Canvas.html
    /// [`Window`]: ../../sdl2/video/struct.Window.html
    pub fn canvas_mut(&mut self) -> &mut Canvas<Window> {
        &mut self.canvas
    }
}
