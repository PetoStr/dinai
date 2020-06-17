pub mod window;

use crate::window::GameWindow;
use crate::window::WindowConfig;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;

struct Game {
    game_window: GameWindow,
    running: bool,
}

impl Game {
    fn new(game_window: GameWindow) -> Self {
        Game { game_window, running: false }
    }

    fn start_loop(&mut self) {
        self.running = true;
        while self.running {
            let events = self.game_window
                .event_pump_mut()
                .poll_iter()
                .collect::<Vec<_>>();
            for event in events {
                match event {
                    Event::Quit { .. } => self.running = false,
                    Event::KeyDown {
                        keycode: Some(x),
                        ..
                    } => self.key_down(x),
                    Event::KeyUp {
                        keycode: Some(x),
                        ..
                    } => self.key_up(x),
                    _ => {}
                }
            }

            self.draw();
        }
    }

    fn draw(&mut self) {
        let canvas = &mut self.game_window.canvas_mut();

        canvas.set_draw_color(Color::RGB(240, 240, 240));
        canvas.clear();

        canvas.present();
    }

    fn key_down(&mut self, keycode: Keycode) {
        match keycode {
            Keycode::Q => self.running = false,
            _ => {}
        }
    }

    fn key_up(&mut self, _keycode: Keycode) {
    }
}

pub fn run(win_conf: WindowConfig) -> Result<(), String> {
    let game_window = GameWindow::new(win_conf)?;
    let mut game = Game::new(game_window);
    game.start_loop();

    Ok(())
}
