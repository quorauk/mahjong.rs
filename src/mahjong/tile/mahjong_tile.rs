use crate::mahjong::strategy::block_strategy::Meld;
use crate::mahjong::strategy::block_strategy::Protorun;

use super::enums::Dragon;
use super::enums::Suit;
use super::enums::Wind;
use super::suited_tile::SuitedTile;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MahjongTile {
    Suit(SuitedTile),
    Dragon(Dragon),
    Wind(Wind),
}

pub trait Nextable {
    fn next(&self) -> Self;
}

pub trait Meldable where Self: Sized {
    fn melds(&self) -> Vec<Meld>;
    fn protoruns(&self) -> Vec<Protorun>;
}

impl MahjongTile {
    pub fn new_suit(suit: Suit, value: i8) -> Self {
        MahjongTile::Suit(
            SuitedTile {
                suit,
                value
            }
        )
    }

    fn next(&self) -> Self {
        match self {
            MahjongTile::Suit(suit) => MahjongTile::Suit(suit.next()),
            MahjongTile::Dragon(dragon) => MahjongTile::Dragon( dragon.next()),
            MahjongTile::Wind(wind) => MahjongTile::Wind(wind.next()),
        }
    }

    pub fn possible_melds(&self) -> Vec<Meld> {
        match self {
            MahjongTile::Suit(suit) => suit.melds(),
            MahjongTile::Dragon(dragon) => dragon.melds(),
            MahjongTile::Wind(wind) => wind.melds(),
        }
    }

    pub fn possible_protoruns(&self) -> Vec<Protorun> {
        match self {
            MahjongTile::Suit(suit) => suit.protoruns(),
            MahjongTile::Dragon(dragon) => dragon.protoruns(),
            MahjongTile::Wind(wind) => wind.protoruns(),
        }
    }
}