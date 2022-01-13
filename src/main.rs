use bracket_lib::prelude::*;

struct Platform {
    position: i32,
}

struct Ball {
    x: i32,
    y: i32,
    velocity: i32,
    angle: f64,
}

struct State {
    player: Platform,
    cpu: Platform,
    ball: Ball,
    timer: instant::Instant,
    ongoing: bool,
}

fn initialize(s: &mut State) {
    s.player = Platform { position: 21 };
    s.cpu = Platform { position: 21 };
    // place ball in middle of screen
    s.ball = Ball {
        x: 40,
        y: 25,
        velocity: 2,
        angle: 125.0,
    };
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let bg_color = bracket_lib::prelude::RGBA::from_hex(String::from("#1a1b26ff"));
        match bg_color {
            Ok(color) => ctx.cls_bg(color),
            Err(_e) => ctx.print(40, 25, "error converting color"),
        }
        ctx.draw_hollow_box(
            0,
            0,
            79,
            49,
            RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
            RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
        );

        if self.ongoing {
            match ctx.key {
                Some(bracket_lib::prelude::VirtualKeyCode::Down) => {
                    if self.player.position + 8 > 46 {
                        self.player.position = self.player.position
                    } else {
                        self.player.position += 2
                    }
                }
                Some(bracket_lib::prelude::VirtualKeyCode::Up) => {
                    if self.player.position <= 2 {
                        self.player.position = self.player.position
                    } else {
                        self.player.position -= 2
                    }
                }
                Some(bracket_lib::prelude::VirtualKeyCode::R) => {
                    self.ongoing = false;
                }
                None => (),
                _ => (),
            }

            if self.timer.elapsed().as_millis() > 20 {
                if self.ball.x == 75 {
                    if !(self.ball.y < self.cpu.position || self.ball.y > self.cpu.position + 8) {
                        self.ball.angle = 180.0 - self.ball.angle;
                    } else {
                        self.ball.velocity = 0;
                    }
                } else if self.ball.x == 4 {
                    if !(self.ball.y < self.player.position
                        || self.ball.y > self.player.position + 8)
                    {
                        self.ball.angle = 180.0 - self.ball.angle;
                    } else {
                        self.ball.velocity = 0;
                    }
                } else if self.ball.y == 2 || self.ball.y == 46 {
                    self.ball.angle = 360.0 - self.ball.angle;
                }
                let geom: f64 = self.ball.angle * std::f64::consts::PI / 180.0;
                self.ball.x += (self.ball.velocity as f64 * geom.cos()) as i32;
                self.ball.y += (self.ball.velocity as f64 * geom.sin()) as i32;
                if self.ball.y >= 6 && self.ball.y < 45 {
                    self.cpu.position = self.ball.y - 4;
                }
                self.timer = instant::Instant::now();
            }

            // draw player platform
            ctx.draw_box(
                2,
                self.player.position,
                1,
                8,
                RGBA::from_f32(0.0, 1.0, 0.0, 1.0),
                RGBA::from_f32(0.0, 1.0, 0.0, 1.0),
            );

            // draw cpu platform
            ctx.draw_box(
                76,
                self.cpu.position,
                1,
                8,
                RGBA::from_f32(0.0, 1.0, 0.0, 1.0),
                RGBA::from_f32(0.0, 1.0, 0.0, 1.0),
            );

            ctx.set_bg(self.ball.x, self.ball.y, RGBA::from_f32(1.0, 0.0, 0.0, 1.0));
        } else {
            if !ctx.left_click {
                ctx.print_centered(25, "Click screen to start.");
                initialize(self);
            } else {
                self.ongoing = true;
            }
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Pong with Rust")
        .build()?;

    let now = instant::Instant::now();

    let gs: State = State {
        player: Platform { position: 21 },
        cpu: Platform { position: 21 },
        ball: Ball {
            x: 40,
            y: 25,
            velocity: 2,
            angle: 10.0,
        },
        timer: now,
        ongoing: false,
    };

    main_loop(context, gs)
}
