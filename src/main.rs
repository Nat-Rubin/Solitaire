use std::{env, path};
use std::path::PathBuf;
use std::process::exit;
use ggez;
use ggez::{Context, event, GameError, GameResult, graphics};
use ggez::event::MouseButton;
use ggez::graphics::{Color, DrawParam, Image, Rect};
use crate::structs::{Card, GameState};

use image::io::Reader as ImageReader;

mod structs;
mod cards;

const CARD_WIDTH: f32 = 100.0;
const CARD_HEIGHT: f32 = 140.0;

const GRID_SIZE: (i16, i16) = (9, 15);

const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * CARD_HEIGHT,
    GRID_SIZE.1 as f32 * CARD_WIDTH,
);

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.05, 0.25, 0.15, 1.0]));

        for card in &self.deck.cards {
            let rect = Rect::new(
                card.position.0,
                card.position.1,
                CARD_WIDTH,
                CARD_HEIGHT,
            );
            canvas.draw(
                &graphics::Quad,
                DrawParam::new()
                    .dest(rect.point())
                    .scale(rect.size())
                    .color(Color::BLACK),
            );
            //let image = Image::from_path(ctx, "2_of_clubs.png")?;
            canvas.draw(&card.image, DrawParam::new().dest_rect(rect));
        }

        canvas.finish(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult {
        for mut card in &mut self.deck.cards {
            if button == event::MouseButton::Left &&
                x >= card.position.0 && x <= card.position.0 + CARD_WIDTH &&
                y >= card.position.1 && y <= card.position.1 + CARD_HEIGHT {
                card.set_dragging(true);
                break;
            }
        }
        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) -> GameResult {
        if button == MouseButton::Left {
            for mut card in &mut self.deck.cards {
                card.set_dragging(false);
            }
        }
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) -> GameResult {
        for mut card in &mut self.deck.cards {
            if card.dragging {
                card.set_position((x - CARD_WIDTH / 2.0, y - CARD_HEIGHT / 2.0));
            }
        }
        Ok(())
    }
}

fn main() -> GameResult {
    println!("Hello, world!");

    // let (mut ctx, events_loop) = ggez::ContextBuilder::new("Solitaire", "Nat R")
    //     .window_setup(ggez::conf::WindowSetup::default().title("Solitaire"))
    //     .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
    //     //.resources_dir_name("cards/")
    //     .build()?;
    //
    // let state = GameState::new(&mut ctx);
    // event::run(ctx, events_loop, state)



    let mut cb = ggez::ContextBuilder::new("Solitaire", "Nat R")
        .window_setup(ggez::conf::WindowSetup::default().title("Solitaire"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1));

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources//");
        path
    } else {
        path::PathBuf::from(".//cards//")
    };
    cb = cb.add_resource_path(resource_dir);

    let (mut ctx, event_loop) = cb.build()?;

    let state = GameState::new(&mut ctx);
    event::run(ctx, event_loop, state)
}
