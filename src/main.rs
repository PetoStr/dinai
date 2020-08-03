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

    fn update(&mut self, ctx: &Context, environment: &Environment) {
        if self.aabbf().intersects(&environment.obstacle.aabbf()) {
            self.alive = false;
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

                let floor_bb = &environment.floor.bounding_box;

                // Player intersects with floor.
                if bb.intersects(floor_bb) {
                    self.velocity.y = 0.0;
                    self.pos.y = floor_bb.min.y - self.size.y;
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

trait Game {
    fn draw(&mut self, ctx: &mut Context) -> Result<(), String>;
    fn update(&mut self, ctx: &mut Context) -> Result<(), String>;
}

struct Environment {
    floor: Floor,
    obstacle: Obstacle,
}

struct DinaiGame {
    player: Player,
    environment: Environment,
}

impl DinaiGame {
    fn new(ctx: &mut Context) -> Self {
        let win_width = ctx.game_window.config().width;

        let floor = Floor {
            bounding_box: AABBf {
                min: Vector2f::from_coords(0.0, 600.0),
                max: Vector2f::from_coords(win_width as f32, 620.0),
            },
        };
        let floor_bot_y = floor.bounding_box.min.y;

        let player = Player {
            pos: Vector2f::from_coords(100.0, floor_bot_y - 25.0),
            size: Vector2f::from_coords(25.0, 25.0),
            state: MovementState::Running,
            alive: true,
            score: 0.0,
            velocity: Vector2f::new(),
        };

        let obstacle = Obstacle {
            pos: Vector2f::from_coords(win_width as f32, floor_bot_y - 35.0),
            size: Vector2f::from_coords(25.0, 35.0),
            velocity_x: -400.0,
        };

        Self {
            player,
            environment: Environment { floor, obstacle },
        }
    }
}

impl Game for DinaiGame {
    fn draw(&mut self, ctx: &mut Context) -> Result<(), String> {
        ctx.game_window.clear(Color::RGB(240, 240, 240));

        self.environment.obstacle.draw(ctx)?;
        self.player.draw(ctx)?;
        self.environment.floor.draw(ctx)?;

        ctx.game_window.present();

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> Result<(), String> {
        let player = &mut self.player;
        let env = &mut self.environment;

        if ctx.game_window.is_key_pressed(&Keycode::Q) {
            ctx.game_window.close();
        }

        if player.alive {
            player.update(ctx, env);
            env.obstacle.update(ctx);
        }

        Ok(())
    }
}

fn main() -> Result<(), String> {
    let win_conf = WindowConfig {
        title: "dinai",
        width: 1280,
        height: 720,
    };

    let mut game_window = GameWindow::new(win_conf)?;

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let text_renderer = TextRenderer::new(&ttf_context, game_window.canvas())?;

    let mut ctx = Context {
        game_window: &mut game_window,
        text_renderer: &text_renderer,
        delta_time: 0.0,
    };

    let mut the_game = DinaiGame::new(&mut ctx);

    let mut start_time = Instant::now();

    while !ctx.game_window.should_close() {
        ctx.delta_time = start_time.elapsed().as_secs_f32();
        start_time = Instant::now();

        ctx.game_window.poll();

        the_game.update(&mut ctx)?;
        the_game.draw(&mut ctx)?;
    }

    Ok(())
}
