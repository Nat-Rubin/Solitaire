use crate::structs::{Card, GameState, Pile, Piles};
use ggez;
use ggez::event::MouseButton;
use ggez::graphics::{Canvas, Color, DrawParam, Rect};
use ggez::{event, graphics, Context, GameError, GameResult};
use std::path::PathBuf;
use std::process::exit;
use std::{env, path};
use crate::structs::Piles::Deck;

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

fn within_pile(x:f32, y: f32, pile: &Pile) -> bool {
    return x >= pile.position.0
        && x <= pile.position.0 + CARD_WIDTH
        && y >= pile.position.1
        && y <= pile.position.1 + CARD_HEIGHT
}

impl event::EventHandler<ggez::GameError> for GameState<> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let card_back: graphics::Image = graphics::Image::from_path(ctx, PathBuf::from("/cards/card_back.png")).unwrap();
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.05, 0.25, 0.15, 1.0]));

        if self.current_cards.is_some() {
            draw_cards(self.current_cards.as_ref().unwrap(), &mut canvas);
        }

        draw_cards(&self.discard, &mut canvas);

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

        // for card in self.discard.cards.iter() {
        //     let rect = Rect::new(card.position.0, card.position.1, CARD_WIDTH, CARD_HEIGHT);
        //     canvas.draw(
        //         &graphics::Quad,
        //         DrawParam::new()
        //             .dest(rect.point())
        //             .scale(rect.size())
        //             .color(Color::BLACK),
        //     );
        //     // let image = Image::from_path(ctx, &card.image)?;
        //     canvas.draw(&card.image, DrawParam::new().dest(rect.point()).scale([CARD_IMAGE_SCALE, CARD_IMAGE_SCALE]));
        // }

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
            self.mouse_position = (x, y);
            let is_discard_empty: bool = self.discard.cards.is_empty();

            // check discard
            if within_pile(x, y, &self.discard) {
                if is_discard_empty { return Ok(()); }  // No card to move, just return
                //let card: &Card = self.discard.cards.last().unwrap();
                let mut new_cards = Pile {
                    cards: vec![],
                    direction: None,
                    position: (x, y),
                };
                self.discard.move_card(&mut new_cards, 0, true);
                self.current_cards = Some(new_cards.clone());
                self.current_pile = Some(Piles::Discard);
            }
            // check deck
            else if within_pile(x, y, &self.deck) {
                    self.current_pile = Some(Piles::Deck);
            }
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
            match self.current_pile {
                Some(Piles::Deck) => {
                    if (x, y) == self.mouse_position {  // check if mouse moved
                        // check deck
                        if within_pile(x, y, &self.deck) {
                            let is_deck_empty: bool = self.deck.cards.is_empty();
                            if is_deck_empty {
                                println!("Here");
                                self.deck.reset(&mut self.discard);
                                return Ok(());
                            } else {
                                let mut card: &mut Card = self.deck.cards.first_mut().unwrap();
                                if (x, y) == self.mouse_position {
                                    card.set_flipped(true);
                                    card.set_dragging(false);
                                    self.deck.move_card(&mut self.discard, 0, false);
                                }
                            }
                        } else {

                        }
                    } else {
                        //for card in
                        // let mut card: &mut Card = self.deck.cards.first_mut().unwrap();
                        // card.set_dragging(false);
                    }
                }
                Some(Piles::Discard) => {
                    let mut card: &mut Card = self.discard.cards.last_mut().unwrap();
                    card.set_dragging(false);

                    if within_pile(x, y, &self.hearts_pile) {

                    } else if within_pile(x, y, &self.diamonds_pile) {

                    } else if within_pile(x, y, &self.clubs_pile) {

                    } else if within_pile(x, y, &self.spades_pile) {

                    }

                }

                (_) => println!("{:?} is not yet implemented!", self.current_pile)
            }
        }
        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
    ) -> GameResult {
        // let is_deck_empty: bool = self.discard.cards.is_empty();
        // if is_deck_empty {
        //     return Ok(());
        // } else {
        //     let card: &mut Card = self.discard.cards.last_mut().unwrap();
        //     if card.dragging {
        //         card.set_position((x - CARD_WIDTH / 2.0, y - CARD_HEIGHT / 2.0));
        //     }
        // }
        let mut current_cards: &mut Option<&mut Pile> = &mut self.current_cards.as_mut();
        match current_cards {
            None => {}
            Some(_) => {
                for card in &mut current_cards.as_mut().unwrap().cards {
                    card.set_position((x, y));
                }
                return Ok(());
            }
        }
        Ok(())
    }
}

fn draw_cards(pile: &Pile, canvas: &mut Canvas) {
    for card in pile.cards.iter() {
        let rect = Rect::new(card.position.0, card.position.1, CARD_WIDTH, CARD_HEIGHT);
        canvas.draw(
            &graphics::Quad,
            DrawParam::new()
                .dest(rect.point())
                .scale(rect.size())
                .color(Color::BLACK),
        );
        canvas.draw(&card.image, DrawParam::new().dest(rect.point()).scale([CARD_IMAGE_SCALE, CARD_IMAGE_SCALE]));
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
