use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

pub struct WindowConfig {
    pub title: &'static str,
    pub width: u32,
    pub height: u32,
}

pub struct GameWindow {
    config: WindowConfig,
    canvas: Canvas<Window>,
    should_close: bool,
    event_pump: EventPump,
}

impl GameWindow {
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
            should_close: false,
            event_pump,
        })
    }

    pub fn update(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => self.should_close = true,
                _ => {}
            }
        }
    }

    pub fn config(&self) -> &WindowConfig {
        &self.config
    }

    pub fn canvas_mut(&mut self) -> &mut Canvas<Window> {
        &mut self.canvas
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }
}
