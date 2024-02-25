use std::path::PathBuf;
use ggez::{Context, graphics};
use oorandom::Rand32;
use rand::seq::SliceRandom;


#[derive(Clone, Copy, Debug)]
enum Direction {
    Down,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}

#[derive(Clone, Copy, Debug)]
enum Color {
    Red,
    Black,
}

pub struct GridPosition {

}

#[derive(Clone, Debug)]
pub struct Card {
    pub num: u8,
    pub suit: Suit,
    pub color: Color,
    pub image: ggez::graphics::Image,
    pub flipped: bool,
    pub position: (f32, f32),
    pub rect: graphics::Mesh,
    pub dragging: bool,
}

impl Card {
    pub fn from_heart(num: u8, image: ggez::graphics::Image, ctx: &mut Context) -> Self {
        let mb = &mut graphics::MeshBuilder::new();
        Card {
            num,
            suit: Suit::Hearts,
            color: Color::Red,
            image,
            flipped: false,
            position: (10.0, 10.0),
            rect: graphics::Mesh::from_data(ctx, mb.build()),
            dragging: false,
        }
    }
    pub fn from_diamond(num: u8, image: ggez::graphics::Image, ctx: &mut Context) -> Self {
        let mb = &mut graphics::MeshBuilder::new();
        Card {
            num,
            suit: Suit::Diamonds,
            color: Color::Red,
            image,
            flipped: false,
            position: (10.0, 10.0),
            rect: ggez::graphics::Mesh::from_data(ctx, mb.build()),
            dragging: false,
        }
    }

    pub fn from_club(num: u8, image: ggez::graphics::Image, ctx: &mut Context) -> Self {
        let mb = &mut graphics::MeshBuilder::new();
        Card {
            num,
            suit: Suit::Clubs,
            color: Color::Black,
            image,
            flipped: false,
            position: (10.0, 10.0),
            rect: graphics::Mesh::from_data(ctx, mb.build()),
            dragging: false,
        }
    }

    pub fn from_spade(num: u8, image: ggez::graphics::Image, ctx: &mut Context) -> Self {
        let mb = &mut graphics::MeshBuilder::new();
        Card {
            num,
            suit: Suit::Spades,
            color: Color::Black,
            image,
            flipped: false,
            position: (10.0, 10.0),
            rect: graphics::Mesh::from_data(ctx, mb.build()),
            dragging: false,
        }
    }

    pub fn set_dragging(&mut self, dragging: bool) {
        self.dragging = dragging;
    }

    pub fn set_position(&mut self, position: (f32, f32)) {
        self.position = position;
    }
}

#[derive(Clone, Debug)]
pub struct Deck {
    pub cards: Vec<Card>
}

impl Deck {
    pub fn new(ctx: &mut Context) -> Self {
        let heart_ace: Card = Card::from_heart(1, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\ace_of_hearts.png")).unwrap(), ctx);
        let heart_two: Card = Card::from_heart(2, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\2_of_hearts.png")).unwrap(), ctx);
        let heart_three: Card = Card::from_heart(3, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\3_of_hearts.png")).unwrap(), ctx);
        let heart_four: Card = Card::from_heart(4, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\4_of_hearts.png")).unwrap(), ctx);
        let heart_five: Card = Card::from_heart(5, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\5_of_hearts.png")).unwrap(), ctx);
        let heart_six: Card = Card::from_heart(6, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\6_of_hearts.png")).unwrap(), ctx);
        let heart_seven: Card = Card::from_heart(7, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\7_of_hearts.png")).unwrap(), ctx);
        let heart_eight: Card = Card::from_heart(8, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\8_of_hearts.png")).unwrap(), ctx);
        let heart_nine: Card = Card::from_heart(9, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\9_of_hearts.png")).unwrap(), ctx);
        let heart_ten: Card = Card::from_heart(10, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\10_of_hearts.png")).unwrap(), ctx);
        let heart_jack: Card = Card::from_heart(11, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\jack_of_hearts2.png")).unwrap(), ctx);
        let heart_queen: Card = Card::from_heart(12, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\queen_of_hearts2.png")).unwrap(), ctx);
        let heart_king: Card = Card::from_heart(13, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\king_of_hearts2.png")).unwrap(), ctx);

        let diamond_ace: Card = Card::from_diamond(1, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\ace_of_diamonds.png")).unwrap(), ctx);
        let diamond_two: Card = Card::from_diamond(2, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\2_of_diamonds.png")).unwrap(), ctx);
        let diamond_three: Card = Card::from_diamond(3, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\3_of_diamonds.png")).unwrap(), ctx);
        let diamond_four: Card = Card::from_diamond(4, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\4_of_diamonds.png")).unwrap(), ctx);
        let diamond_five: Card = Card::from_diamond(5, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\5_of_diamonds.png")).unwrap(), ctx);
        let diamond_six: Card = Card::from_diamond(6, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\6_of_diamonds.png")).unwrap(), ctx);
        let diamond_seven: Card = Card::from_diamond(7, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\7_of_diamonds.png")).unwrap(), ctx);
        let diamond_eight: Card = Card::from_diamond(8, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\8_of_diamonds.png")).unwrap(), ctx);
        let diamond_nine: Card = Card::from_diamond(9, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\9_of_diamonds.png")).unwrap(), ctx);
        let diamond_ten: Card = Card::from_diamond(10, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\10_of_diamonds.png")).unwrap(), ctx);
        let diamond_jack: Card = Card::from_diamond(11, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\jack_of_diamonds2.png")).unwrap(), ctx);
        let diamond_queen: Card = Card::from_diamond(12, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\queen_of_diamonds2.png")).unwrap(), ctx);
        let diamond_king: Card = Card::from_diamond(13, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\king_of_diamonds2.png")).unwrap(), ctx);

        let club_ace: Card = Card::from_club(1, ggez::graphics::Image::from_path(ctx,PathBuf::from("\\cards\\ace_of_clubs.png")).unwrap(), ctx);
        let club_two: Card = Card::from_club(2, ggez::graphics::Image::from_path(ctx,PathBuf::from("\\cards\\2_of_clubs.png")).unwrap(), ctx);
        let club_three: Card = Card::from_club(3, ggez::graphics::Image::from_path(ctx,PathBuf::from("\\cards\\3_of_clubs.png")).unwrap(), ctx);
        let club_four: Card = Card::from_club(4, ggez::graphics::Image::from_path(ctx,PathBuf::from("\\cards\\4_of_clubs.png")).unwrap(), ctx);
        let club_five: Card = Card::from_club(5, ggez::graphics::Image::from_path(ctx,PathBuf::from("\\cards\\5_of_clubs.png")).unwrap(), ctx);
        let club_six: Card = Card::from_club(6, ggez::graphics::Image::from_path(ctx,PathBuf::from("\\cards\\6_of_clubs.png")).unwrap(), ctx);
        let club_seven: Card = Card::from_club(7, ggez::graphics::Image::from_path(ctx,PathBuf::from("\\cards\\7_of_clubs.png")).unwrap(), ctx);
        let club_eight: Card = Card::from_club(8, ggez::graphics::Image::from_path(ctx,PathBuf::from("\\cards\\8_of_clubs.png")).unwrap(), ctx);
        let club_nine: Card = Card::from_club(9, ggez::graphics::Image::from_path(ctx,PathBuf::from("\\cards\\9_of_clubs.png")).unwrap(), ctx);
        let club_ten: Card = Card::from_club(10, ggez::graphics::Image::from_path(ctx,PathBuf::from("\\cards\\10_of_clubs.png")).unwrap(), ctx);
        let club_jack: Card = Card::from_club(11, ggez::graphics::Image::from_path(ctx,PathBuf::from("\\cards\\jack_of_clubs2.png")).unwrap(), ctx);
        let club_queen: Card = Card::from_club(12, ggez::graphics::Image::from_path(ctx,PathBuf::from("\\cards\\queen_of_clubs2.png")).unwrap(), ctx);
        let club_king: Card = Card::from_club(13, ggez::graphics::Image::from_path(ctx,PathBuf::from("\\cards\\king_of_clubs2.png")).unwrap(), ctx);

        let spade_ace: Card = Card::from_spade(1, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\ace_of_spades.png")).unwrap(), ctx);
        let spade_two: Card = Card::from_spade(2, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\2_of_spades.png")).unwrap(), ctx);
        let spade_three: Card = Card::from_spade(3, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\3_of_spades.png")).unwrap(), ctx);
        let spade_four: Card = Card::from_spade(4, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\4_of_spades.png")).unwrap(), ctx);
        let spade_five: Card = Card::from_spade(5, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\5_of_spades.png")).unwrap(), ctx);
        let spade_six: Card = Card::from_spade(6, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\6_of_spades.png")).unwrap(), ctx);
        let spade_seven: Card = Card::from_spade(7, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\7_of_spades.png")).unwrap(), ctx);
        let spade_eight: Card = Card::from_spade(8, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\8_of_spades.png")).unwrap(), ctx);
        let spade_nine: Card = Card::from_spade(9, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\9_of_spades.png")).unwrap(), ctx);
        let spade_ten: Card = Card::from_spade(10, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\10_of_spades.png")).unwrap(), ctx);
        let spade_jack: Card = Card::from_spade(11, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\jack_of_spades2.png")).unwrap(), ctx);
        let spade_queen: Card = Card::from_spade(12, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\queen_of_spades2.png")).unwrap(), ctx);
        let spade_king: Card = Card::from_spade(13, ggez::graphics::Image::from_path(ctx, PathBuf::from("\\cards\\king_of_spades2.png")).unwrap(), ctx);

        let mut deck: Vec<Card> = Vec::new();

        deck.push(heart_ace);
        deck.push(heart_two);
        deck.push(heart_three);
        deck.push(heart_four);
        deck.push(heart_five);
        deck.push(heart_six);
        deck.push(heart_seven);
        deck.push(heart_eight);
        deck.push(heart_nine);
        deck.push(heart_ten);
        deck.push(heart_jack);
        deck.push(heart_queen);
        deck.push(heart_king);

        deck.push(diamond_ace);
        deck.push(diamond_two);
        deck.push(diamond_three);
        deck.push(diamond_four);
        deck.push(diamond_five);
        deck.push(diamond_six);
        deck.push(diamond_seven);
        deck.push(diamond_eight);
        deck.push(diamond_nine);
        deck.push(diamond_ten);
        deck.push(diamond_jack);
        deck.push(diamond_queen);
        deck.push(diamond_king);

        deck.push(club_ace);
        deck.push(club_two);
        deck.push(club_three);
        deck.push(club_four);
        deck.push(club_five);
        deck.push(club_six);
        deck.push(club_seven);
        deck.push(club_eight);
        deck.push(club_nine);
        deck.push(club_ten);
        deck.push(club_jack);
        deck.push(club_queen);
        deck.push(club_king);

        deck.push(spade_ace);
        deck.push(spade_two);
        deck.push(spade_three);
        deck.push(spade_four);
        deck.push(spade_five);
        deck.push(spade_six);
        deck.push(spade_seven);
        deck.push(spade_eight);
        deck.push(spade_nine);
        deck.push(spade_ten);
        deck.push(spade_jack);
        deck.push(spade_queen);
        deck.push(spade_king);

        Deck {
            cards: deck.clone()
        }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::thread_rng());
    }
}

pub struct CardHolderDeck {
    pub cards: Vec<Card>,
    pub direction: Option<Direction>,
    pub face_down: bool,
}

pub struct CardHolderCards {
    pub cards: Vec<Card>,
    pub direction: Option<Direction>,
    pub face_down: bool,
}

pub struct CardHolderAces {
    pub cards: Vec<Card>,
    pub direction: Option<Direction>,
    pub face_down: bool,
}

pub struct GameState {
    pub screen: graphics::ScreenImage,
    pub deck: Deck,
    pub gameover: bool,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> Self {
        GameState {
            screen: graphics::ScreenImage::new(ctx, graphics::ImageFormat::Rgba8UnormSrgb, 1., 1., 1),
            deck: Deck::new(ctx),
            gameover: false,
        }
    }
}