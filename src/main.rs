use crate::structs::{Card, GameState};
use ggez;
use ggez::event::MouseButton;
use ggez::graphics::{Color, DrawParam, Rect};
use ggez::{event, graphics, Context, GameError, GameResult};
use std::path::PathBuf;
use std::process::exit;
use std::{env, path};
use ggez::mint::Vector2;

use image::io::Reader as ImageReader;

mod cards;
mod structs;

const CARD_WIDTH: f32 = 100.0;
const CARD_HEIGHT: f32 = 140.0;
const CARD_IMAGE_SCALE: f32 = 0.215;
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
        let card_back: graphics::Image = graphics::Image::from_path(ctx, PathBuf::from("/cards/card_back.png")).unwrap();
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.05, 0.25, 0.15, 1.0]));

        for card in self.deck.cards.iter() {
            let rect = Rect::new(card.position.0, card.position.1, CARD_WIDTH, CARD_HEIGHT);
            canvas.draw(
                &graphics::Quad,
                DrawParam::new()
                    .dest(rect.point())
                    .scale(rect.size())
                    .color(Color::BLACK),
            );
            // let image = Image::from_path(ctx, &card.image)?;
            if card.flipped {
                let card_back_scale = 0.215;
                canvas.draw(&card_back, DrawParam::new().dest(rect.point()).scale([card_back_scale, card_back_scale]));
                //canvas.draw(&card.image, DrawParam::new().dest(rect.point()).scale([CARD_IMAGE_SCALE, CARD_IMAGE_SCALE]));
            } else {
                canvas.draw(&card.image, DrawParam::new().dest(rect.point()).scale([CARD_IMAGE_SCALE, CARD_IMAGE_SCALE]));
            }
        }

        for card in self.discard.cards.iter() {
            let rect = Rect::new(card.position.0, card.position.1, CARD_WIDTH, CARD_HEIGHT);
            canvas.draw(
                &graphics::Quad,
                DrawParam::new()
                    .dest(rect.point())
                    .scale(rect.size())
                    .color(Color::BLACK),
            );
            // let image = Image::from_path(ctx, &card.image)?;
                canvas.draw(&card.image, DrawParam::new().dest(rect.point()).scale([CARD_IMAGE_SCALE, CARD_IMAGE_SCALE]));
        }

        canvas.finish(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        if button == MouseButton::Left {
            let is_deck_empty: bool = self.deck.cards.is_empty();
            if is_deck_empty {
                self.mouse_position = (x, y);
            } else {
                let mut card: &mut Card = self.deck.cards.first_mut().unwrap();
                //for mut card in &mut self.deck.cards {
                if x >= card.position.0
                    && x <= card.position.0 + CARD_WIDTH
                    && y >= card.position.1
                    && y <= card.position.1 + CARD_HEIGHT {
                    self.mouse_position = (x, y);
                    card.set_dragging(true);
                }
            }
            //}
            // Click on deck, put card in discard pile, flip card
            //for mut card in
        }
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        if button == MouseButton::Left {
            let is_deck_empty: bool = self.deck.cards.is_empty();
            // TODO: change this to mouse movement
            if is_deck_empty {
                if (x, y) == self.mouse_position {
                    println!("Here");
                    self.deck.reset(&mut self.discard);
                    return Ok(());
                }
            } else {
                let mut card: &mut Card = self.deck.cards.first_mut().unwrap();
                if (x, y) == self.mouse_position {
                    if is_deck_empty {
                        println!("Here");
                        self.deck.reset(&mut self.discard);
                        return Ok(());
                    }
                    card.set_flipped(true);
                    self.deck.move_card(&mut self.discard);
                }
            }
            //let mut card: &mut Card = self.deck.cards.first_mut().unwrap();
            //card.set_dragging(false);
        }
        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        x: f32,
        y: f32,
        _dx: f32,
        _dy: f32,
    ) -> GameResult {
        /* already an &mut Card*/
        for card in &mut self.deck.cards {
            if card.dragging {
                card.set_position((x - CARD_WIDTH / 2.0, y - CARD_HEIGHT / 2.0));
            }
        }
        Ok(())
    }
}

fn main() -> GameResult {
    println!("Hello, world!");

    let mut cb = ggez::ContextBuilder::new("Solitaire", "Nat R")
        .window_setup(ggez::conf::WindowSetup::default().title("Solitaire"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1));

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources/");
        path
    } else {
        path::PathBuf::from("./resources/")
    };
    cb = cb.add_resource_path(resource_dir);

    let (mut ctx, event_loop) = cb.build()?;

    let state = GameState::new(&mut ctx);
    println!("{:?}", &state.discard.position);
    event::run(ctx, event_loop, state)
}
