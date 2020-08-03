//! A wrapper for SDL2 library.

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::{Window, WindowContext};
use sdl2::EventPump;
use std::collections::HashSet;

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
        let events = self.event_pump.poll_iter().collect::<Vec<_>>();

        for event in events {
            match event {
                Event::Quit { .. } => self.should_close = true,
                Event::KeyDown {
                    keycode: Some(key_code),
                    ..
                } => {
                    self.pressed_keys.insert(key_code);
                }
                Event::KeyUp {
                    keycode: Some(key_code),
                    ..
                } => {
                    self.pressed_keys.remove(&key_code);
                }
                _ => {}
            };
        }
    }

    /// Clears the screen with the given color.
    pub fn clear(&mut self, clear_color: Color) {
        self.canvas.set_draw_color(clear_color);
        self.canvas.clear();
    }

    /// Updates the screen,
    pub fn present(&mut self) {
        self.canvas.present();
    }

    /// Checks whether the given key is pressed.
    pub fn is_key_pressed(&self, key_code: &Keycode) -> bool {
        self.pressed_keys.contains(key_code)
    }

    /// Returns true when a quit event has been received.
    pub fn should_close(&self) -> bool {
        self.should_close
    }

    /// Hints that this window should close.
    pub fn close(&mut self) {
        self.should_close = true;
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

    /// Returns a reference to current [`Canvas`] of `SDL2` [`Window`].
    ///
    /// [`Canvas`]: ../../sdl2/render/struct.Canvas.html
    /// [`Window`]: ../../sdl2/video/struct.Window.html
    pub fn canvas(&self) -> &Canvas<Window> {
        &self.canvas
    }

    /// Returns a mutable reference to current [`Canvas`] of `SDL2` [`Window`].
    ///
    /// [`Canvas`]: ../../sdl2/render/struct.Canvas.html
    /// [`Window`]: ../../sdl2/video/struct.Window.html
    pub fn canvas_mut(&mut self) -> &mut Canvas<Window> {
        &mut self.canvas
    }
}

/// A helper text renderer for specific `Font`.
pub struct TextRenderer<'a> {
    font: Font<'a, 'a>,
    texture_creator: TextureCreator<WindowContext>,
}

impl<'a> TextRenderer<'a> {
    /// Creates a new text renderer for the given [`Canvas`].
    ///
    /// [`Canvas`]: ../../sdl2/render/struct.Canvas.html
    pub fn new(ttf_context: &'a Sdl2TtfContext, canvas: &Canvas<Window>) -> Result<Self, String> {
        let mut font = ttf_context.load_font("Inconsolata-Bold.ttf", 128)?;
        font.set_style(sdl2::ttf::FontStyle::BOLD);

        let texture_creator = canvas.texture_creator();

        Ok(Self {
            font,
            texture_creator,
        })
    }

    /// Draws the given text on the [`Canvas`].
    ///
    /// # Examples
    ///
    /// [`Canvas`]: ../../sdl2/render/struct.Canvas.html
    ///
    /// ```
    /// # use dinai::window::{GameWindow, TextRenderer, WindowConfig};
    /// #
    /// # let config = WindowConfig {
    /// #     title: "Title",
    /// #     width: 1280,
    /// #     height: 720,
    /// # };
    /// #
    /// # let mut game_window = GameWindow::new(config).unwrap();
    /// #
    /// let ttf_context = sdl2::ttf::init().unwrap();
    /// let text_renderer = TextRenderer::new(&ttf_context, game_window.canvas()).unwrap();
    ///
    /// text_renderer.draw_text("Hello", 0, 0, 0.2, game_window.canvas_mut());
    /// ```
    pub fn draw_text(
        &self,
        text: &str,
        x: i32,
        y: i32,
        scale: f32,
        canvas: &mut Canvas<Window>,
    ) -> Result<(), String> {
        let surface = self
            .font
            .render(text)
            .blended(Color::RGBA(0, 0, 0, 255))
            .map_err(|e| e.to_string())?;

        let texture = self
            .texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let width = surface.width() as f32 * scale;
        let height = surface.height() as f32 * scale;

        canvas.copy(
            &texture,
            None,
            Some(Rect::new(x, y, width as u32, height as u32)),
        )?;

        Ok(())
    }
}
