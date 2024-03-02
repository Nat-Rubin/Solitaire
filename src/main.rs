use crate::structs::{Card, GameState, Pile, Piles};
use ggez;
use ggez::event::MouseButton;
use ggez::graphics::{Canvas, Color, DrawParam, Rect};
use ggez::{event, graphics, Context, GameError, GameResult};
use std::path::PathBuf;
use std::process::exit;
use std::{env, path};

mod cards;
mod structs;

const CARD_WIDTH: f32 = 100.0;
const CARD_HEIGHT: f32 = 140.0;
const CARD_IMAGE_SCALE: f32 = 0.215;
const GRID_SIZE: (i16, i16) = (11, 18);

const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * CARD_HEIGHT,
    GRID_SIZE.1 as f32 * CARD_WIDTH,
);

fn within_card(x: f32, y: f32, card: &Card) -> bool {
    return x >= card.position.0
        && x <= card.position.0 + CARD_WIDTH
        && y >= card.position.1
        && y <= card.position.1 + CARD_HEIGHT
}

fn within_pile(x: f32, y: f32, pile: &Pile) -> bool {
    if pile.pile != Piles::Deck || pile.pile != Piles::Discard || pile.pile != Piles::Hearts ||
        pile.pile != Piles::Discard || pile.pile != Piles::Clubs || pile.pile != Piles::Spades {

        return x >= pile.position.0
            && x <= pile.position.0 + CARD_WIDTH
            && y >= pile.position.1
            && y <= pile.position.1 + (CARD_HEIGHT + pile.cards.last().unwrap().position.1);
    }

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
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.05, 0.25, 0.15, 1.0]));

        // DRAW PILES
        draw_cards(ctx, &mut self.deck, &mut canvas, false);
        if !self.discard.cards.is_empty() {
            draw_cards(ctx, &mut self.discard, &mut canvas, false);
        }
        draw_cards(ctx, &mut self.first, &mut canvas, true);
        draw_cards(ctx, &mut self.second, &mut canvas, true);
        draw_cards(ctx, &mut self.third, &mut canvas, true);
        draw_cards(ctx, &mut self.fourth, &mut canvas, true);
        draw_cards(ctx, &mut self.fifth, &mut canvas, true);
        draw_cards(ctx, &mut self.sixth, &mut canvas, true);
        draw_cards(ctx, &mut self.seventh, &mut canvas, true);;
        if self.current_cards.is_some() {
            draw_cards(ctx, self.current_cards.as_mut().unwrap(), &mut canvas, false);
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
            self.mouse_position = (x, y);
            let is_discard_empty: bool = self.discard.cards.is_empty();

            // check deck
            if within_pile(x, y, &self.deck) {
                self.current_pile = Some(Piles::Deck);
            }
            // check discard
            else if within_pile(x, y, &self.discard) {
                if is_discard_empty { return Ok(()); }  // No card to move, just return
                let mut new_cards = Pile {
                    pile: Piles::NewCards,
                    cards: vec![],
                    direction: None,
                    position: (x, y),
                };
                self.discard.move_card(&mut new_cards, 0, false, true);

                self.current_cards = Some(new_cards.clone());
                self.current_pile = Some(Piles::Discard);
            }
            else if within_pile(x, y, &self.hearts_pile) {
                let mut new_cards = Pile {
                    pile: Piles::NewCards,
                    cards: vec![],
                    direction: None,
                    position: (x, y),
                };

                self.hearts_pile.move_card(&mut new_cards, 0, false, true);

                self.current_cards = Some(new_cards.clone());
                self.current_pile = Some(Piles::Hearts);
            }
            else if within_pile(x, y, &self.diamonds_pile) {
                let mut new_cards = Pile {
                    pile: Piles::NewCards,
                    cards: vec![],
                    direction: None,
                    position: (x, y),
                };

                self.hearts_pile.move_card(&mut new_cards, 0, false, true);

                self.current_cards = Some(new_cards.clone());
                self.current_pile = Some(Piles::Diamonds);
            }
            else if within_pile(x, y, &self.clubs_pile) {
                let mut new_cards = Pile {
                    pile: Piles::NewCards,
                    cards: vec![],
                    direction: None,
                    position: (x, y),
                };

                self.clubs_pile.move_card(&mut new_cards, 0, false, true);

                self.current_cards = Some(new_cards.clone());
                self.current_pile = Some(Piles::Clubs);
            }
            else if within_pile(x, y, &self.spades_pile) {
                let mut new_cards = Pile {
                    pile: Piles::NewCards,
                    cards: vec![],
                    direction: None,
                    position: (x, y),
                };

                self.spades_pile.move_card(&mut new_cards, 0, false, true);

                self.current_cards = Some(new_cards.clone());
                self.current_pile = Some(Piles::Spades);
            }

            else if within_pile(x, y, &self.first) {
                for (i, card) in self.first.cards.iter().enumerate().rev() {
                    if within_card(x, y, card) {
                        if card.flipped == false {
                            break;
                        }
                        let mut new_cards = Pile {
                            pile: Piles::NewCards,
                            cards: vec![],
                            direction: None,
                            position: (x, y),
                        };

                        self.first.move_card(&mut new_cards, (i-1) as i32, false, true);

                        self.current_cards = Some(new_cards.clone());
                        break;
                    }
                }
                self.current_pile = Some(Piles::First);
            }
            else if within_pile(x, y, &self.second) {
                for (i, card) in self.second.cards.iter().enumerate().rev() {
                    if within_card(x, y, card) {
                        if card.flipped == false {
                            break;
                        }
                        let mut new_cards = Pile {
                            pile: Piles::NewCards,
                            cards: vec![],
                            direction: None,
                            position: (x, y),
                        };

                        self.second.move_card(&mut new_cards, (i-1) as i32, true, true);

                        self.current_cards = Some(new_cards.clone());
                        break;
                    }
                }
                self.current_pile = Some(Piles::Second);
            }
            else if within_pile(x, y, &self.third) {
                for (i, card) in self.third.cards.iter().enumerate().rev() {
                    if within_card(x, y, card) {
                        if card.flipped == false {
                            break;
                        }
                        let mut new_cards = Pile {
                            pile: Piles::NewCards,
                            cards: vec![],
                            direction: None,
                            position: (x, y),
                        };

                        self.third.move_card(&mut new_cards, (i-1) as i32, false, true);

                        self.current_cards = Some(new_cards.clone());
                        break;
                    }
                }
                self.current_pile = Some(Piles::Third);
            }
            else if within_pile(x, y, &self.fourth) {
                for (i, card) in self.fourth.cards.iter().enumerate().rev() {
                    if within_card(x, y, card) {
                        if card.flipped == false {
                            break;
                        }
                        let mut new_cards = Pile {
                            pile: Piles::NewCards,
                            cards: vec![],
                            direction: None,
                            position: (x, y),
                        };

                        self.fourth.move_card(&mut new_cards, (i-1) as i32, false, true);

                        self.current_cards = Some(new_cards.clone());
                        break;
                    }
                }
                self.current_pile = Some(Piles::Fourth);
            }
            else if within_pile(x, y, &self.fifth) {
                for (i, card) in self.fifth.cards.iter().enumerate().rev() {
                    if within_card(x, y, card) {
                        if card.flipped == false {
                            break;
                        }
                        let mut new_cards = Pile {
                            pile: Piles::NewCards,
                            cards: vec![],
                            direction: None,
                            position: (x, y),
                        };

                        self.fifth.move_card(&mut new_cards, (i-1) as i32, false, true);

                        self.current_cards = Some(new_cards.clone());
                        break;
                    }
                }
                self.current_pile = Some(Piles::Fifth);
            }
            else if within_pile(x, y, &self.sixth) {
                for (i, card) in self.sixth.cards.iter().enumerate().rev() {
                    if within_card(x, y, card) {
                        if card.flipped == false {
                            break;
                        }
                        let mut new_cards = Pile {
                            pile: Piles::NewCards,
                            cards: vec![],
                            direction: None,
                            position: (x, y),
                        };

                        self.sixth.move_card(&mut new_cards, (i-1) as i32, false, true);

                        self.current_cards = Some(new_cards.clone());
                        break;
                    }
                }
                self.current_pile = Some(Piles::Sixth);
            }
            else if within_pile(x, y, &self.seventh) {
                for (i, card) in self.seventh.cards.iter().enumerate().rev() {
                    if within_card(x, y, card) {
                        if card.flipped == false {
                            break;
                        }
                        let mut new_cards = Pile {
                            pile: Piles::NewCards,
                            cards: vec![],
                            direction: None,
                            position: (x, y),
                        };

                        self.seventh.move_card(&mut new_cards, (i-1) as i32, false, true);

                        self.current_cards = Some(new_cards.clone());
                        break;
                    }
                }
                self.current_pile = Some(Piles::Seventh);
            }
            else {

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
                    if (x, y) == self.mouse_position {  // check if mouse (not) moved
                        // check deck
                        //if within_pile(x, y, &self.deck) {
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
                                    self.deck.move_card(&mut self.discard, 0, false, true);
                                }
                            }
                        //} else {
                    } else {

                    }
                }
                Some(_) => {
                    //if (x, y) != self.mouse_position {
                    if self.current_cards.is_some() {
                        let mut card: &mut Card = self.current_cards.as_mut().unwrap().cards.first_mut().unwrap();
                        card.set_dragging(false);
                        println!("HERE");
                        let mut is_valid: bool = false;

                        if within_pile(x, y, &self.hearts_pile) && self.current_pile.unwrap() == Piles::Hearts {
                            self.hearts_pile.is_aces_valid(self.current_cards.as_mut().unwrap());
                        } else if within_pile(x, y, &self.diamonds_pile) && self.current_pile.unwrap() == Piles::Diamonds {
                            self.diamonds_pile.is_aces_valid(self.current_cards.as_mut().unwrap());
                        } else if within_pile(x, y, &self.clubs_pile) && self.current_pile.unwrap() == Piles::Clubs {
                            self.clubs_pile.is_aces_valid(self.current_cards.as_mut().unwrap());
                        } else if within_pile(x, y, &self.spades_pile) && self.current_pile.unwrap() == Piles::Spades {
                            self.spades_pile.is_aces_valid(self.current_cards.as_mut().unwrap());
                        } else if within_pile(x, y, &self.first) && self.current_pile.unwrap() == Piles::First {
                            is_valid = self.first.is_number_valid(self.current_cards.as_mut().unwrap());
                        } else if within_pile(x, y, &self.second) && self.current_pile.unwrap() == Piles::Second {
                            is_valid = self.second.is_number_valid(self.current_cards.as_mut().unwrap());
                        } else if within_pile(x, y, &self.third) && self.current_pile.unwrap() == Piles::Third {
                            is_valid = self.third.is_number_valid(self.current_cards.as_mut().unwrap());
                        } else if within_pile(x, y, &self.fourth) && self.current_pile.unwrap() == Piles::Fourth {
                            is_valid = self.fourth.is_number_valid(self.current_cards.as_mut().unwrap());
                        } else if within_pile(x, y, &self.fifth) && self.current_pile.unwrap() == Piles::First {
                            is_valid = self.fifth.is_number_valid(self.current_cards.as_mut().unwrap());
                        } else if within_pile(x, y, &self.sixth) && self.current_pile.unwrap() == Piles::Sixth {
                            is_valid = self.sixth.is_number_valid(self.current_cards.as_mut().unwrap());
                        } else if within_pile(x, y, &self.seventh) && self.current_pile.unwrap() == Piles::Seventh {
                            is_valid = self.seventh.is_number_valid(self.current_cards.as_mut().unwrap());
                        } else {
                            println!("huh?");
                        }

                        if !is_valid {
                            match self.current_pile.unwrap() {
                                Piles::Discard => {
                                    let index: i32 = (self.current_cards.as_mut().unwrap().cards.len() - 1) as i32;
                                    self.current_cards.as_mut().unwrap().move_card(&mut self.discard, index, false, true);
                                }
                                Piles::Hearts => {
                                    let index: i32 = (self.current_cards.as_mut().unwrap().cards.len() - 1) as i32;
                                    self.current_cards.as_mut().unwrap().move_card(&mut self.hearts_pile, index, false, true);
                                }
                                Piles::Diamonds => {
                                    let index: i32 = (self.current_cards.as_mut().unwrap().cards.len() - 1) as i32;
                                    self.current_cards.as_mut().unwrap().move_card(&mut self.diamonds_pile, index, false, true);
                                }
                                Piles::Clubs => {
                                    let index: i32 = (self.current_cards.as_mut().unwrap().cards.len() - 1) as i32;
                                    self.current_cards.as_mut().unwrap().move_card(&mut self.clubs_pile, index, false, true);
                                }
                                Piles::Spades => {
                                    let index: i32 = (self.current_cards.as_mut().unwrap().cards.len() - 1) as i32;
                                    self.current_cards.as_mut().unwrap().move_card(&mut self.spades_pile, index, false, true);
                                }
                                Piles::First => {
                                    let index: i32 = (self.current_cards.as_mut().unwrap().cards.len() - 1) as i32;
                                    self.current_cards.as_mut().unwrap().move_card(&mut self.first, index, false, true);
                                }
                                Piles::Second => {
                                    let index: i32 = (self.current_cards.as_mut().unwrap().cards.len() - 1) as i32;
                                    self.current_cards.as_mut().unwrap().move_card(&mut self.second, index, false, true);
                                }
                                Piles::Third => {
                                    let index: i32 = (self.current_cards.as_mut().unwrap().cards.len() - 1) as i32;
                                    self.current_cards.as_mut().unwrap().move_card(&mut self.third, index, false, true);
                                }
                                Piles::Fourth => {
                                    let index: i32 = (self.current_cards.as_mut().unwrap().cards.len() - 1) as i32;
                                    self.current_cards.as_mut().unwrap().move_card(&mut self.fourth, index, false, true);
                                }
                                Piles::Fifth => {
                                    let index: i32 = (self.current_cards.as_mut().unwrap().cards.len() - 1) as i32;
                                    self.current_cards.as_mut().unwrap().move_card(&mut self.fifth, index, false, true);
                                }
                                Piles::Sixth => {
                                    let index: i32 = (self.current_cards.as_mut().unwrap().cards.len() - 1) as i32;
                                    self.current_cards.as_mut().unwrap().move_card(&mut self.sixth, index, false, true);
                                }
                                Piles::Seventh => {
                                    let index: i32 = (self.current_cards.as_mut().unwrap().cards.len() - 1) as i32;
                                    self.current_cards.as_mut().unwrap().move_card(&mut self.seventh, index, false, true);
                                }
                                _ => { println!("HUUUUH???") }
                            }
                        }
                    }
                    //self.current_cards.unwrap()
                }

                // Some(_) => {
                //     let mut card: &mut Card = self.discard.cards.last_mut().unwrap();
                //     card.set_dragging(false);
                //     println!("HERE");
                //     if within_pile(x, y, &self.hearts_pile) && self.current_pile.unwrap() == Piles::Hearts {
                //         self.hearts_pile.is_aces_valid(self.current_cards.as_mut().unwrap());
                //     } else if within_pile(x, y, &self.diamonds_pile) && self.current_pile.unwrap() == Piles::Diamonds {
                //         self.diamonds_pile.is_aces_valid(self.current_cards.as_mut().unwrap());
                //     } else if within_pile(x, y, &self.clubs_pile) && self.current_pile.unwrap() == Piles::Clubs {
                //         self.clubs_pile.is_aces_valid(self.current_cards.as_mut().unwrap());
                //     } else if within_pile(x, y, &self.spades_pile) && self.current_pile.unwrap() == Piles::Spades {
                //         self.spades_pile.is_aces_valid(self.current_cards.as_mut().unwrap());
                //     } else if within_pile(x, y, &self.first) && self.current_pile.unwrap() == Piles::First {
                //         self.first.is_number_valid(self.current_cards.as_mut().unwrap());
                //     } else if within_pile(x, y, &self.second) && self.current_pile.unwrap() == Piles::Second {
                //         self.second.is_number_valid(self.current_cards.as_mut().unwrap());
                //     } else if within_pile(x, y, &self.third) && self.current_pile.unwrap() == Piles::Third {
                //         self.third.is_number_valid(self.current_cards.as_mut().unwrap());
                //     } else if within_pile(x, y, &self.fourth) && self.current_pile.unwrap() == Piles::Fourth {
                //         self.fourth.is_number_valid(self.current_cards.as_mut().unwrap());
                //     } else if within_pile(x, y, &self.fifth) && self.current_pile.unwrap() == Piles::First {
                //         self.fifth.is_number_valid(self.current_cards.as_mut().unwrap());
                //     } else if within_pile(x, y, &self.sixth) && self.current_pile.unwrap() == Piles::Sixth {
                //         self.sixth.is_number_valid(self.current_cards.as_mut().unwrap());
                //     } else if within_pile(x, y, &self.seventh) && self.current_pile.unwrap() == Piles::Seventh {
                //         self.seventh.is_number_valid(self.current_cards.as_mut().unwrap());
                //     } else {
                //         println!("huh?");
                //     }
                // }

                (_) => println!("{:?} is not yet implemented!", self.current_pile)
            }
            println!("Heerree");
            self.current_cards = None;
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

fn draw_cards(ctx: &mut Context, pile: &mut Pile, canvas: &mut Canvas, is_numbered_pile: bool) {
    let card_back: graphics::Image = graphics::Image::from_path(ctx, PathBuf::from("/cards/card_back.png")).unwrap();
    println!("{:?}", pile.pile);
    if pile.cards.first().is_none() {
        println!("Here");
    }
    let mut prev_y_position: f32 = pile.cards.first().unwrap().position.1;
    let position_y_diff: f32 = CARD_HEIGHT/3.0;

    for card in pile.cards.iter_mut() {
        if is_numbered_pile {
            card.set_position((card.position.0, prev_y_position));
            prev_y_position += position_y_diff
        }
        let rect = Rect::new(card.position.0, card.position.1, CARD_WIDTH, CARD_HEIGHT);
        canvas.draw(
            &graphics::Quad,
            DrawParam::new()
                .dest(rect.point())
                .scale(rect.size())
                .color(Color::BLACK),
        );
        if card.flipped {
            canvas.draw(&card.image, DrawParam::new().dest(rect.point()).scale([CARD_IMAGE_SCALE, CARD_IMAGE_SCALE]));
        } else {
            let card_back_scale = 0.215;
            canvas.draw(&card_back, DrawParam::new().dest(rect.point()).scale([card_back_scale, card_back_scale]));
        }
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
    event::run(ctx, event_loop, state)
}
