pub mod math;
pub mod physics;
pub mod window;

use crate::math::Vector2f;
use crate::physics::CollFilter;
use crate::physics::Entity;
use crate::physics::Physics;
use crate::physics::Transform;
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
}

impl Game {
    fn new(game_window: GameWindow) -> Self {
        let mut world = World::new(Vector2f::from_coords(0.0, 0.05));

        let floor_id = 1;

        let player = Rc::new(RefCell::new(Entity {
            transform: Transform {
                pos: Vector2f::from_coords(300.0, 400.0),
                size: Vector2f::from_coords(20.0, 20.0),
            },
            physics: Physics {
                speed: Vector2f::from_coords(2.5, -5.5),
                disable_gravity: false,
                coll_filter: CollFilter {
                    group_id: 0,
                    check_mask: floor_id,
                },
            },
            collision: |this, other| {
                this.physics.disable_gravity = true;
                this.physics.speed = Vector2f::new();
                this.transform.pos.y = other.transform.pos.y - other.transform.size.y;
            },
        }));

        let floor = Rc::new(RefCell::new(Entity {
            transform: Transform {
                pos: Vector2f::from_coords(0.0, 500.0),
                size: Vector2f::from_coords(game_window.config().width as f32, 20.0),
            },
            physics: Physics {
                speed: Vector2f::new(),
                disable_gravity: true,
                coll_filter: CollFilter {
                    group_id: floor_id,
                    check_mask: 0,
                },
            },
            collision: |_this, _other| {},
        }));

        world.add_entity(Rc::clone(&player));
        world.add_entity(Rc::clone(&floor));

        Game {
            game_window,
            running: false,
            world,
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

        canvas.set_draw_color(Color::RGB(0, 0, 0));

        for entity in self.world.entities() {
            let borrowed_entity = entity.borrow();
            let transform = &borrowed_entity.transform;

            canvas.fill_rect(Rect::new(
                transform.pos.x as i32,
                transform.pos.y as i32,
                transform.size.x as u32,
                transform.size.y as u32,
            ))?;
        }

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
