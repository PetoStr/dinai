use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

const TITLE: &str = "dinai";
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn render(canvas: &mut Canvas<Window>) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(240, 240, 240));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(0, 128, 0));
    canvas.fill_rect(Rect::new(0, (HEIGHT as i32) * 4 / 5, WIDTH, 5))?;

    canvas.present();

    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window(TITLE, WIDTH, HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    break 'main_loop
                },
                _ => {}
            }
        }

        render(&mut canvas)?;
    }

    Ok(())
}

