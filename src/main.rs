use dinai::math::{AABBf, Matrix, Vector2f};
use dinai::neuralnet::NeuralNetwork;
use dinai::window::{GameWindow, TextRenderer, WindowConfig};
use rayon::prelude::*;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Instant;

const GRAVITY: f32 = 800.0;

struct Context<'a> {
    game_window: &'a mut GameWindow,
    text_renderer: &'a TextRenderer<'a>,
    step_s: f32,
    speed: f32,
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

    nnet: NeuralNetwork<3, 4, 1>,
}

impl Player {
    fn draw(&self, ctx: &mut Context, interpolation: f32) -> Result<(), String> {
        let canvas = ctx.game_window.canvas_mut();

        let pos = self.pos + self.velocity * interpolation;

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(Rect::new(
            pos.x as i32,
            pos.y as i32,
            self.size.x as u32,
            self.size.y as u32,
        ))?;

        Ok(())
    }

    fn think(&mut self, environment: &Environment) {
        let pos_y = self.pos.y;
        let obstacle_dx = environment.obstacle.pos.x - self.pos.x;
        let score = self.score;

        let input = Matrix::from([[pos_y, obstacle_dx, score]]);
        let output = self.nnet.feed(&input);
        if output.as_ref()[0][0] > 0.75 {
            self.jump();
        }
    }

    fn update(&mut self, step_s: f32, environment: &Environment) {
        if self.aabbf().intersects(&environment.obstacle.aabbf()) {
            self.alive = false;
            return;
        }

        self.think(environment);

        if let MovementState::Jumping = self.state {
            self.velocity.y += GRAVITY * step_s;

            // Predict collision one frame in advance. This way the player
            // does not flicker after landing on the floor.
            let future_pos = self.pos + self.velocity * step_s;

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

        self.score += step_s;

        self.velocity.x = 0.0;
        self.pos += self.velocity * step_s;
    }

    fn aabbf(&self) -> AABBf {
        AABBf {
            min: self.pos,
            max: self.pos + self.size,
        }
    }

    fn jump(&mut self) {
        if let MovementState::Running = self.state {
            self.velocity.y = -350.0;
            self.state = MovementState::Jumping;
        }
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
    fn draw(&self, ctx: &mut Context, interpolation: f32) -> Result<(), String> {
        let canvas = ctx.game_window.canvas_mut();

        let x_pos = self.pos.x + self.velocity_x * interpolation;

        canvas.set_draw_color(Color::RGB(0, 127, 0));
        canvas.fill_rect(Rect::new(
            x_pos as i32,
            self.pos.y as i32,
            self.size.x as u32,
            self.size.y as u32,
        ))?;

        Ok(())
    }

    fn update(&mut self, ctx: &Context) {
        self.pos.x += self.velocity_x * ctx.step_s;

        if self.pos.x + self.size.x < 0.0 {
            self.pos.x = ctx.game_window.config().width as f32;
        }

        if self.velocity_x > -2000.0 {
            self.velocity_x -= 30.0 * ctx.step_s;
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
    fn draw(&mut self, ctx: &mut Context, interpolation: f32) -> Result<(), String>;
    fn handle_input(&mut self, ctx: &mut Context) -> Result<(), String>;
    fn update(&mut self, ctx: &mut Context) -> Result<(), String>;
}

struct Environment {
    floor: Floor,
    obstacle: Obstacle,
}

struct DinaiGame {
    players: Vec<Player>,
    generation: u32,
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

        let mut players = Vec::new();
        for _ in 0..1000 {
            players.push(Player {
                pos: Vector2f::from_coords(100.0, floor_bot_y - 25.0),
                size: Vector2f::from_coords(25.0, 25.0),
                state: MovementState::Running,
                alive: true,
                score: 0.0,
                velocity: Vector2f::new(),
                nnet: NeuralNetwork::new(),
            });
        }

        let obstacle = Obstacle {
            pos: Vector2f::from_coords(win_width as f32, floor_bot_y - 35.0),
            size: Vector2f::from_coords(25.0, 35.0),
            velocity_x: -400.0,
        };

        Self {
            players,
            environment: Environment { floor, obstacle },
            generation: 0,
        }
    }

    fn restart_env(&mut self, ctx: &Context) {
        let win_width = ctx.game_window.config().width;
        self.environment.obstacle.pos.x = win_width as f32;
        self.environment.obstacle.velocity_x = -400.0;
    }

    fn next_generation(&mut self) {
        self.players
            .sort_unstable_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        let parent1_net = &self.players[0].nnet;
        let parent2_net = &self.players[1].nnet;
        let child_net = parent1_net.crossover(&parent2_net);

        let floor_bot_y = self.environment.floor.bounding_box.min.y;

        let mut children = Vec::with_capacity(self.players.len());
        for _ in 0..self.players.len() {
            let mut nnet = child_net.clone();
            nnet.mutate();

            children.push(Player {
                pos: Vector2f::from_coords(100.0, floor_bot_y - 25.0),
                size: Vector2f::from_coords(25.0, 25.0),
                state: MovementState::Running,
                alive: true,
                score: 0.0,
                velocity: Vector2f::new(),
                nnet,
            });
        }

        self.players = children;
        self.generation += 1;
    }
}

impl Game for DinaiGame {
    fn draw(&mut self, ctx: &mut Context, interpolation: f32) -> Result<(), String> {
        ctx.game_window.clear(Color::RGB(240, 240, 240));

        self.environment.obstacle.draw(ctx, interpolation)?;
        for player in self.players.iter() {
            if player.alive {
                player.draw(ctx, interpolation)?;
            }
        }
        self.environment.floor.draw(ctx)?;

        let canvas = ctx.game_window.canvas_mut();
        let mut p_iter = self.players.iter().skip_while(|p| !p.alive);
        if let Some(ref player) = p_iter.next() {
            let score = format!("Score: {:.2}", player.score);
            ctx.text_renderer.draw_text(&score, 10, 10, 0.2, canvas)?;
        }

        let gen = format!("Generation: {}", self.generation);
        ctx.text_renderer.draw_text(&gen, 10, 35, 0.2, canvas)?;

        let alive_cn = self
            .players
            .iter()
            .fold(0, |acc, p| if p.alive { acc + 1 } else { acc });
        let alive = format!("Alive: {}", alive_cn);
        ctx.text_renderer.draw_text(&alive, 10, 60, 0.2, canvas)?;

        let speed = format!("Speed: {:.1}", ctx.speed);
        ctx.text_renderer.draw_text(&speed, 10, 110, 0.2, canvas)?;

        ctx.game_window.present();

        Ok(())
    }

    fn handle_input(&mut self, ctx: &mut Context) -> Result<(), String> {
        if ctx.game_window.is_key_pressed(&Keycode::Q) {
            ctx.game_window.close();
        }

        if ctx.game_window.is_key_pressed(&Keycode::K) {
            ctx.speed += 0.3 * ctx.step_s;
        }
        if ctx.game_window.is_key_pressed(&Keycode::J) {
            ctx.speed -= 0.3 * ctx.step_s;
            ctx.speed = ctx.speed.max(0.1);
        }

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> Result<(), String> {
        let env = &mut self.environment;
        let step_s = ctx.step_s;

        self.players
            .par_iter_mut()
            .filter(|player| player.alive)
            .for_each(|player| {
                player.update(step_s, env);
            });

        let any_alive = self.players.par_iter().any(|player| player.alive);

        if any_alive {
            env.obstacle.update(ctx);
        } else {
            self.next_generation();
            self.restart_env(ctx);
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
        step_s: 1.0 / 30.0,
        speed: 1.0,
    };

    let mut the_game = DinaiGame::new(&mut ctx);

    let mut start_time = Instant::now();
    let mut lag = 0.0;

    while !ctx.game_window.should_close() {
        let delta_time = start_time.elapsed().as_secs_f32() * ctx.speed;
        start_time = Instant::now();
        lag += delta_time.min(0.3);

        ctx.game_window.poll();
        the_game.handle_input(&mut ctx)?;

        while lag > ctx.step_s {
            the_game.update(&mut ctx)?;
            lag -= ctx.step_s;
        }

        the_game.draw(&mut ctx, lag)?;
    }

    Ok(())
}
