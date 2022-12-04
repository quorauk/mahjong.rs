use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::mahjong::{strategy::block_strategy::{Pung, Meld, Protorun}};

use super::mahjong_tile::{Nextable, Meldable, MahjongTile};

#[derive(EnumIter, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Wind {
    North,
    East,
    South,
    West,
}

impl Nextable for Wind {
    fn next(&self) -> Wind {
        match self {
            Wind::North => Wind::East,
            Wind::East => Wind::South,
            Wind::South => Wind::West,
            Wind::West => Wind::North,
        }
    }
}

impl Meldable for Wind {
    fn melds(&self) -> Vec<Meld> {
        vec![Meld::Pung(Pung::new(MahjongTile::Wind(*self)))]
    }

    fn protoruns(&self) -> Vec<Protorun> {
        vec![Protorun{ tiles: [MahjongTile::Wind(*self), MahjongTile::Wind(*self)]}]
    }
}

#[derive(EnumIter, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Dragon {
    Green,
    White,
    Red,
}

impl Nextable for Dragon {
    fn next(&self) -> Dragon {
        match self {
            Dragon::Green => Dragon::White,
            Dragon::White => Dragon::Red,
            Dragon::Red => Dragon::Green,
        }
    }
}

impl Meldable for Dragon {
    fn melds(&self) -> Vec<Meld> {
        vec![Meld::Pung(Pung::new(MahjongTile::Dragon(*self)))]
    }

    fn protoruns(&self) -> Vec<Protorun> {
        vec![Protorun { tiles: [MahjongTile::Dragon(*self), MahjongTile::Dragon(*self)]}]
    }
}


#[derive(EnumIter, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Suit {
    Pin,
    Man,
    Sou,
}