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
            event_pump,
        })
    }

    pub fn event_pump_mut(&mut self) -> &mut EventPump {
        &mut self.event_pump
    }

    pub fn config(&self) -> &WindowConfig {
        &self.config
    }

    pub fn canvas_mut(&mut self) -> &mut Canvas<Window> {
        &mut self.canvas
    }
}
