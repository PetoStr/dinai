use dinai::math::{AABBf, Vector2f};
use dinai::window::{GameWindow, TextRenderer, WindowConfig};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Instant;

const GRAVITY: f32 = 800.0;

struct Context<'a> {
    game_window: &'a mut GameWindow,
    text_renderer: &'a TextRenderer<'a>,
    floor: &'a Floor,
    delta_time: f32,
}

enum MovementState {
    Running,
    Jumping,
}

struct Player {
    pos: Vector2f,
    size: Vector2f,
    state: MovementState,
    alive: bool,
    score: f32,

    // Defined as pixels per second.
    velocity: Vector2f,
}

impl Player {
    fn draw(&self, ctx: &mut Context) -> Result<(), String> {
        let canvas = ctx.game_window.canvas_mut();
        let score_text = format!("Score: {:.2}", self.score);

        ctx.text_renderer
            .draw_text(&score_text, 0, 0, 0.2, canvas)?;

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(Rect::new(
            self.pos.x as i32,
            self.pos.y as i32,
            self.size.x as u32,
            self.size.y as u32,
        ))?;

        Ok(())
    }

    fn update(&mut self, ctx: &Context) {
        if !self.alive {
            return;
        }

        match self.state {
            MovementState::Running => {
                if ctx.game_window.is_key_pressed(&Keycode::W) {
                    self.jump();
                }
            }
            MovementState::Jumping => {
                self.velocity.y += GRAVITY * ctx.delta_time;

                // Predict collision one frame in advance. This way the player
                // does not flicker after landing on the floor.
                let future_pos = self.pos + self.velocity * ctx.delta_time;

                let bb = AABBf {
                    min: future_pos,
                    max: future_pos + self.size,
                };

                // Player intersects with floor.
                if bb.intersects(&ctx.floor.bounding_box) {
                    self.velocity.y = 0.0;
                    self.pos.y = ctx.floor.bounding_box.min.y - self.size.y;
                    self.state = MovementState::Running;
                }
            }
        }

        self.score += ctx.delta_time;

        self.velocity.x = 0.0;
        self.pos += self.velocity * ctx.delta_time;
    }

    fn aabbf(&self) -> AABBf {
        AABBf {
            min: self.pos,
            max: self.pos + self.size,
        }
    }

    fn jump(&mut self) {
        self.velocity.y = -350.0;
        self.state = MovementState::Jumping;
    }
}

struct Floor {
    // The floor does not move and therefore it always has the same
    // axis-aligned bounding box used for intersection testing.
    bounding_box: AABBf,
}

impl Floor {
    fn draw(&self, ctx: &mut Context) -> Result<(), String> {
        let bb = &self.bounding_box;
        let canvas = ctx.game_window.canvas_mut();

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

struct Obstacle {
    pos: Vector2f,
    size: Vector2f,

    // Defined as pixels per second on the x-axis.
    velocity_x: f32,
}

impl Obstacle {
    fn draw(&self, ctx: &mut Context) -> Result<(), String> {
        let canvas = ctx.game_window.canvas_mut();

        canvas.set_draw_color(Color::RGB(0, 127, 0));
        canvas.fill_rect(Rect::new(
            self.pos.x as i32,
            self.pos.y as i32,
            self.size.x as u32,
            self.size.y as u32,
        ))?;

        Ok(())
    }

    fn update(&mut self, ctx: &Context) {
        self.pos.x += self.velocity_x * ctx.delta_time;

        if self.pos.x + self.size.x < 0.0 {
            self.pos.x = ctx.game_window.config().width as f32;
        }
    }

    fn aabbf(&self) -> AABBf {
        AABBf {
            min: self.pos,
            max: self.pos + self.size,
        }
    }
}

fn main() -> Result<(), String> {
    let win_conf = WindowConfig {
        title: "dinai",
        width: 1280,
        height: 720,
    };

    let floor = Floor {
        bounding_box: AABBf {
            min: Vector2f::from_coords(0.0, 600.0),
            max: Vector2f::from_coords(win_conf.width as f32, 620.0),
        },
    };

    let floor_top_y = floor.bounding_box.min.y;
    let mut player = Player {
        pos: Vector2f::from_coords(100.0, floor_top_y - 25.0),
        size: Vector2f::from_coords(25.0, 25.0),
        state: MovementState::Running,
        alive: true,
        score: 0.0,
        velocity: Vector2f::new(),
    };

    let mut obstacle = Obstacle {
        pos: Vector2f::from_coords(win_conf.width as f32, floor_top_y - 35.0),
        size: Vector2f::from_coords(25.0, 35.0),
        velocity_x: -400.0,
    };

    let mut game_window = GameWindow::new(win_conf)?;

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let text_renderer = TextRenderer::new(&ttf_context, game_window.canvas())?;

    let mut start_time = Instant::now();

    while !game_window.should_close() {
        let delta_time = start_time.elapsed().as_secs_f32();
        start_time = Instant::now();

        game_window.poll();

        if game_window.is_key_pressed(&Keycode::Q) {
            break;
        }

        let mut ctx = Context {
            game_window: &mut game_window,
            text_renderer: &text_renderer,
            floor: &floor,
            delta_time,
        };

        if player.alive {
            player.update(&ctx);
            obstacle.update(&ctx);

            if player.aabbf().intersects(&obstacle.aabbf()) {
                player.alive = false;
            }
        }

        let canvas = ctx.game_window.canvas_mut();

        // Set clear color.
        canvas.set_draw_color(Color::RGB(240, 240, 240));
        canvas.clear();

        obstacle.draw(&mut ctx)?;
        player.draw(&mut ctx)?;
        floor.draw(&mut ctx)?;

        ctx.game_window.canvas_mut().present();
    }

    Ok(())
}
