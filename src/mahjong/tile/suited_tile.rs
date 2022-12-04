use std::cmp::{min, max};

use strum_macros::EnumIter;

use crate::mahjong::{tile::enums::Suit, strategy::block_strategy::{Meld, Pung, Chow, Protorun}};

use super::mahjong_tile::{Meldable, Nextable, MahjongTile};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SuitedTile {
    pub suit: Suit,
    pub value: i8,
}

impl Nextable for SuitedTile {
    fn next(&self) -> Self {
        SuitedTile {
            suit: self.suit,
            value: (self.value % 9) + 1,
        }
    }
}

impl Meldable for SuitedTile {
    fn melds(&self) -> Vec<Meld> {
        let mut melds = vec![Meld::Pung(Pung::new(MahjongTile::Suit(*self)))];
        let chiis: Vec<Meld> = (max(1, self.value - 2)..min(8, self.value + 2))
                .into_iter()
                .map(|v|
                  Meld::Chow(Chow::new(
                    MahjongTile::Suit(SuitedTile{ suit: self.suit, value: v }),
                    MahjongTile::Suit(SuitedTile{ suit: self.suit, value: v + 1 }),
                    MahjongTile::Suit(SuitedTile{ suit: self.suit, value: v + 2 }),
                  ))
                ).collect();
        melds.extend(chiis);
        melds
    }

    fn protoruns(&self) -> Vec<Protorun> {
        (max(1, self.value - 2)..min(7, self.value + 2))
                .into_iter()
                .map(|v|
                  Protorun {
                    tiles: [
                        MahjongTile::Suit(*self),
                        MahjongTile::Suit(SuitedTile{ suit: self.suit, value: v})
                    ]
                  }
                ).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next() {
        let tile = SuitedTile {
            suit: Suit::Man,
            value: 8,
        };
        assert_eq!(tile.next().value, 9);
        assert_eq!(tile.next().suit, Suit::Man);
        let tile = SuitedTile {
            suit: Suit::Man,
            value: 9,
        };
        assert_eq!(tile.next().value, 1);
        assert_eq!(tile.next().suit, Suit::Man);
    }

    fn gen_chow(suit: Suit, val: i8) -> Meld {
        Meld::Chow(
            Chow::new(
                MahjongTile::new_suit(suit, val),
                MahjongTile::new_suit(suit, val + 1),
                MahjongTile::new_suit(suit, val + 2)
            )
        )
    }

    #[test]
    fn test_melds() {
        let tile = SuitedTile {
            suit: Suit::Man,
            value: 8,
        };
        assert_eq!(tile.melds().contains(&Meld::Pung(Pung::new(MahjongTile::Suit(tile)))), true);
        assert_eq!(tile.melds().contains(&gen_chow(Suit::Man, 5)), false);
        assert_eq!(tile.melds().contains(&gen_chow(Suit::Man, 6)), true);
        assert_eq!(tile.melds().contains(&gen_chow(Suit::Man, 7)), true);
        assert_eq!(tile.melds().contains(&gen_chow(Suit::Man, 8)), false);
        let tile = SuitedTile {
            suit: Suit::Man,
            value: 9,
        };
        assert_eq!(tile.melds().contains(&gen_chow(Suit::Man, 6)), false);
        assert_eq!(tile.melds().contains(&gen_chow(Suit::Man, 7)), true);
    }
}
