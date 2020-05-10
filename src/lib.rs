pub mod window;

use crate::window::GameWindow;
use crate::window::WindowConfig;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

fn render(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(240, 240, 240));
    canvas.clear();

    canvas.present();
}

pub fn run(win_conf: WindowConfig) -> Result<(), String> {
    let mut game_window = GameWindow::new(win_conf)?;

    while !game_window.should_close() {
        game_window.update();

        render(game_window.canvas_mut());
    }

    Ok(())
}
