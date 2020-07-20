use dinai::math::{AABBf, Vector2f};
use dinai::window::{GameWindow, WindowConfig};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use std::time::Instant;

enum MovementState {
    Running,
    Jumping,
}

struct Player {
    pos: Vector2f,
    size: Vector2f,
    state: MovementState,

    // Defined as pixels per second.
    velocity: Vector2f,
}

impl Player {
    fn draw(&self, canvas: &mut Canvas<sdl2::video::Window>)
        -> Result<(), String>
    {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(Rect::new(
            self.pos.x as i32,
            self.pos.y as i32,
            self.size.x as u32,
            self.size.y as u32,
        ))?;

        Ok(())
    }
}

struct Floor {
    // The floor does not move and therefore it always has the same
    // axis-aligned bounding box used for intersection testing.
    bounding_box: AABBf,
}

impl Floor {
    fn draw(&self, canvas: &mut Canvas<sdl2::video::Window>)
        -> Result<(), String>
    {
        let bb = &self.bounding_box;

        canvas.set_draw_color(Color::RGB(55, 55, 55));
        canvas.fill_rect(Rect::new(
            bb.min.x as i32,
            bb.min.y as i32,
            (bb.max.x - bb.min.x) as u32,
            (bb.max.y - bb.min.y) as u32,
        ))?;

        Ok(())
    }
}

fn main() -> Result<(), String> {
    let win_conf = WindowConfig {
        title: "dinai",
        width: 1280,
        height: 720,
    };

    let mut player = Player {
        pos: Vector2f::from_coords(100.0, 100.0),
        size: Vector2f::from_coords(25.0, 25.0),
        state: MovementState::Jumping,
        velocity: Vector2f::new(),
    };

    let floor = Floor {
        bounding_box: AABBf {
            min: Vector2f::from_coords(0.0, 600.0),
            max: Vector2f::from_coords(win_conf.width as f32, 620.0),
        },
    };

    let mut game_window = GameWindow::new(win_conf)?;

    let gravity = 9.81;

    let mut start_time = Instant::now();

    while !game_window.should_close() {
        let delta_time = start_time.elapsed().as_secs_f32();
        start_time = Instant::now();

        game_window.poll();

        if game_window.is_key_pressed(&Keycode::Q) {
            break;
        }

        match player.state {
            MovementState::Running => {
                if game_window.is_key_pressed(&Keycode::W) {
                    player.velocity.y = -350.0;
                    player.state = MovementState::Jumping;
                }
            },
            MovementState::Jumping => {
                player.velocity.y += gravity;

                // Predict collision one frame in advance. This way the player
                // does not flicker after landing on the floor.
                let future_pos = player.pos + player.velocity * delta_time;

                let bb = AABBf {
                    min: future_pos,
                    max: future_pos + player.size,
                };

                // Player intersects with floor.
                if bb.intersects(&floor.bounding_box) {
                    player.velocity.y = 0.0;
                    player.pos.y = floor.bounding_box.min.y - player.size.y;
                    player.state = MovementState::Running;
                }
            },
        }

        player.velocity.x = 0.0;
        if game_window.is_key_pressed(&Keycode::A) {
            player.velocity.x -= 200.0;
        }
        if game_window.is_key_pressed(&Keycode::D) {
            player.velocity.x += 200.0;
        }

        player.pos += player.velocity * delta_time;

        let canvas = game_window.canvas_mut();

        // Set clear color.
        canvas.set_draw_color(Color::RGB(240, 240, 240));
        canvas.clear();

        player.draw(canvas)?;
        floor.draw(canvas)?;

        canvas.present();
    }

    Ok(())
}
