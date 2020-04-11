use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

struct Config {
    title: &'static str,
    width: u32,
    height: u32,
}

fn render(canvas: &mut Canvas<Window>) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(240, 240, 240));
    canvas.clear();

    canvas.present();

    Ok(())
}

fn main() -> Result<(), String> {
    let conf = Config {
        title: "default title",
        width: 450,
        height: 450,
    };

    run(conf)
}

fn run(config: Config) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(config.title, config.width, config.height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => break 'main_loop,
                _ => {}
            }
        }

        render(&mut canvas)?;
    }

    Ok(())
}
