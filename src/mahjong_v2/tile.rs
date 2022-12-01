use crate::mahjong_v2::game::Tile;
use std::cmp::min;

#[derive(Clone, Copy, Debug)]
pub enum Meld {
    Pung(Tile),
    Chow(Tile, Tile, Tile),
}

#[derive(Clone, Copy, Debug)]
pub enum Protorun {
    Side(Tile, Tile),
    Closed(Tile, Tile),
    Edge(Tile, Tile),
    Pair(Tile, Tile),
}

impl Protorun {
    pub fn is_pair(&self) -> bool {
        if let Protorun::Pair(_, _) = self {
            return true;
        }
        false
    }

    pub fn is_side(&self) -> bool {
        if let Protorun::Side(_, _) = self {
            return true;
        }
        false
    }

    pub fn is_edge(&self) -> bool {
        if let Protorun::Edge(_, _) = self {
            return true;
        }
        false
    }

    pub fn is_closed(&self) -> bool {
        if let Protorun::Closed(_, _) = self {
            return true;
        }
        false
    }
}

impl Tile {
    pub fn possible_melds(&self) -> Vec<Meld> {
        let mut melds = Vec::new();

        match self {
            Tile::Suit(suit, val) => {
                for tile_value in min(val - 2, 1)..*val {
                    melds.push(Meld::Chow(
                        Tile::Suit(suit.clone(), tile_value),
                        Tile::Suit(suit.clone(), tile_value + 1),
                        Tile::Suit(suit.clone(), tile_value + 2),
                    ));
                }
            }
            _ => (),
        }

        melds.push(Meld::Pung(self.clone()));

        melds
    }

    pub fn possible_protoruns(&self) -> Vec<Protorun> {
        let mut protoruns = Vec::new();

        match self {
            Tile::Suit(suit, val) => {
                for tile_value in min(*val - 2, 1)..min(val + 2, 9) {
                    let protorun = form_protorun(
                        Tile::Suit(suit.clone(), *val),
                        Tile::Suit(suit.clone(), tile_value),
                    );
                    if protorun.is_some() {
                        protoruns.push(protorun.unwrap());
                    }
                }
            }
            _ => (),
        }
        protoruns
    }
}

fn form_protorun(a: Tile, b: Tile) -> Option<Protorun> {
    match (a, b) {
        (Tile::Suit(suit_a, a_val), Tile::Suit(suit_b, b_val)) => {
            if suit_a == suit_b {
                if a_val == b_val {
                    return Some(Protorun::Pair(a, b));
                }
                if a_val.abs_diff(b_val) == 1 {
                    return Some(Protorun::Side(a, b));
                }
                if a_val == 1 || b_val == 1 || a_val == 9 || b_val == 9 {
                    return Some(Protorun::Edge(a, b));
                }
                if a_val.abs_diff(b_val) == 2 {
                    return Some(Protorun::Closed(a, b));
                }
            }
        }
        _ => (),
    }
    None
}
