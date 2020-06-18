pub mod math;
pub mod physics;
pub mod window;

use crate::math::Vector2f;
use crate::physics::Entity;
use crate::physics::World;
use crate::window::GameWindow;
use crate::window::WindowConfig;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::cell::RefCell;
use std::rc::Rc;

struct Game {
    game_window: GameWindow,
    running: bool,
    world: World,
    player: Rc<RefCell<Entity>>,
}

impl Game {
    fn new(game_window: GameWindow) -> Self {
        let mut world = World::new(Vector2f::from_coords(0.0, 0.05));

        let player = world.create_entity();
        player.borrow_mut().position = Vector2f::from_coords(300.0, 500.0);
        player.borrow_mut().speed = Vector2f::from_coords(2.5, -5.5);

        Game {
            game_window,
            running: false,
            world,
            player,
        }
    }

    fn start_loop(&mut self) {
        self.running = true;
        while self.running {
            let events = self
                .game_window
                .event_pump_mut()
                .poll_iter()
                .collect::<Vec<_>>();
            for event in events {
                match event {
                    Event::Quit { .. } => self.running = false,
                    Event::KeyDown {
                        keycode: Some(x), ..
                    } => self.key_down(x),
                    Event::KeyUp {
                        keycode: Some(x), ..
                    } => self.key_up(x),
                    _ => {}
                }
            }

            self.world.update();
            let draw_res = self.draw();

            if let Some(err) = draw_res.err() {
                eprintln!("{}", err);
            }
        }
    }

    fn draw(&mut self) -> Result<(), String> {
        let canvas = &mut self.game_window.canvas_mut();

        canvas.set_draw_color(Color::RGB(240, 240, 240));
        canvas.clear();

        let player = &*self.player.borrow();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(Rect::new(
            player.position.x as i32,
            player.position.y as i32,
            20,
            20,
        ))?;

        canvas.present();

        Ok(())
    }

    fn key_down(&mut self, keycode: Keycode) {
        match keycode {
            Keycode::Q => self.running = false,
            _ => {}
        }
    }

    fn key_up(&mut self, _keycode: Keycode) {}
}

pub fn run(win_conf: WindowConfig) -> Result<(), String> {
    let game_window = GameWindow::new(win_conf)?;
    let mut game = Game::new(game_window);
    game.start_loop();

    Ok(())
}
