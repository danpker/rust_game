extern crate ggez;
use ggez::conf;
use ggez::event;
use ggez::event::{Keycode, Mod};
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::graphics::{DrawMode, Point};
use std::time::Duration;

struct MainState {
    player_location: Point,
    size: f32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            player_location: Point { x: 0.0, y: 0.0 },
            size: 100.0,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, keymod: Mod, repeat: bool) {

        match keycode {
            Keycode::Right => self.player_location.x += 2.0,
            Keycode::Left => self.player_location.x -= 2.0,
            Keycode::Up => self.player_location.y -= 2.0,
            Keycode::Down => self.player_location.y += 2.0,
            Keycode::A => self.size -= 1.0,
            Keycode::D => self.size += 1.0,
            _ => println!("huh {}", keycode),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::circle(ctx, DrawMode::Fill, self.player_location, self.size, 32)?;
        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let c = conf::Conf {
        window_title: String::from("Dan's House of Fun"),
        window_icon: String::from(""),
        window_height: 600,
        window_width: 800,
        resizable: false,
        vsync: true,
    };
    let ctx = &mut Context::load_from_conf("rust_game", "danpker", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}

