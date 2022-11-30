use crate::mahjong_v2::game::Tile;
use crate::mahjong_v2::player::ClosedHand;
use crate::mahjong_v2::tile::{Meld, Protorun};
use std::cmp::Ordering;
use std::convert::From;
use std::mem::discriminant;

#[derive(Debug)]
pub struct ProvisionalHand {
    pub melds: Vec<Meld>,
    pub pair: Option<Protorun>,
    pub protoruns: Vec<Protorun>,
    pub floating: Vec<Tile>,
}

impl ProvisionalHand {
    pub fn new(tiles: &Vec<Tile>) -> Self {
        let mut hand = ProvisionalHand {
            melds: Vec::new(),
            pair: None,
            protoruns: Vec::new(),
            floating: tiles.clone(),
        };
        hand.find_melds();
        hand.find_pair();
        hand.find_protoruns();
        hand
    }
}

impl ProvisionalHand {
    pub fn winning(&self) -> bool {
        self.melds.len() == 4 && self.pair.is_some()
    }

    pub fn to_discard(&self) -> Option<Tile> {
        let should_discard = self.floating.first();
        if let Some(tile) = should_discard {
            return Some(*tile);
        }
        return self.first_protorun();
    }

    pub fn first_protorun(&self) -> Option<Tile> {
        let mut protos = self.protoruns.clone();
        protos.sort_by(|a, b| cmp_protos(*a, *b));
        match protos.first() {
            Some(Protorun::Pair(a, _)) => Some(a.clone()),
            Some(Protorun::Side(a, _)) => Some(a.clone()),
            Some(Protorun::Edge(a, _)) => Some(a.clone()),
            Some(Protorun::Closed(a, _)) => Some(a.clone()),
            _ => None,
        }
    }

    fn find_melds(&mut self) {
        let all_floating = self.floating.clone();
        for tile in all_floating {
            for meld in tile.possible_melds() {
                if tiles_have_meld(&self.floating, meld) {
                    self.melds.push(meld.clone());
                    remove_meld(&mut self.floating, meld)
                }
            }
        }
    }

    fn find_pair(&mut self) {
        let all_floating = self.floating.clone();
        for tile in all_floating {
            if tiles_have_protorun(&self.floating, Protorun::Pair(tile, tile)) {
                self.pair = Some(Protorun::Pair(tile, tile));
                remove_protorun(&mut self.floating, Protorun::Pair(tile, tile));
                return;
            }
        }
    }

    fn find_protoruns(&mut self) {
        let all_floating = self.floating.clone();
        for tile in all_floating {
            for protorun in tile.possible_protoruns() {
                if tiles_have_protorun(&self.floating, protorun) {
                    self.protoruns.push(protorun.clone());
                    remove_protorun(&mut self.floating, protorun)
                }
            }
        }
    }
}

fn cmp_protos(a: Protorun, b: Protorun) -> Ordering {
    if discriminant(&a) == discriminant(&b) {
        return Ordering::Equal;
    }
    if a.is_side() && (b.is_closed() || b.is_edge() || b.is_pair()) {
        return Ordering::Greater;
    }
    if a.is_pair() && (b.is_closed() || b.is_edge()) {
        return Ordering::Greater;
    }
    if a.is_closed() && b.is_edge() {
        return Ordering::Greater;
    }

    return Ordering::Less;
}

fn tiles_have_meld(tiles: &Vec<Tile>, meld: Meld) -> bool {
    match meld {
        Meld::Chow(a, b, c) => [a, b, c]
            .iter()
            .all(|tile| tiles.iter().find(|x| **x == *tile).is_some()),
        Meld::Pung(a) => tiles.iter().filter(|tile| **tile == a).count() >= 3,
    }
}

fn remove_meld(tiles: &mut Vec<Tile>, meld: Meld) {
    match meld {
        Meld::Chow(a, b, c) => {
            for tile in [a, b, c] {
                remove_tile(tiles, tile)
            }
        }
        Meld::Pung(a) => {
            for tile in [a, a, a] {
                remove_tile(tiles, tile)
            }
        }
    }
}

fn tiles_have_protorun(tiles: &Vec<Tile>, protorun: Protorun) -> bool {
    match protorun {
        Protorun::Pair(a, b) => [a, b]
            .iter()
            .all(|tile| tiles.iter().filter(|x| **x == *tile).count() > 1),
        Protorun::Closed(a, b) => [a, b]
            .iter()
            .all(|tile| tiles.iter().find(|x| **x == *tile).is_some()),
        Protorun::Edge(a, b) => [a, b]
            .iter()
            .all(|tile| tiles.iter().find(|x| **x == *tile).is_some()),
        Protorun::Side(a, b) => [a, b]
            .iter()
            .all(|tile| tiles.iter().find(|x| **x == *tile).is_some()),
    }
}

fn remove_protorun(tiles: &mut Vec<Tile>, protorun: Protorun) {
    match protorun {
        Protorun::Side(a, b) => {
            for tile in [a, b] {
                remove_tile(tiles, tile)
            }
        }
        Protorun::Closed(a, b) => {
            for tile in [a, a, a] {
                remove_tile(tiles, tile)
            }
        }
        Protorun::Edge(a, b) => {
            for tile in [a, a, a] {
                remove_tile(tiles, tile)
            }
        }
        Protorun::Pair(a, b) => {
            for tile in [a, a, a] {
                remove_tile(tiles, tile)
            }
        }
    }
}

fn remove_tile(tiles: &mut Vec<Tile>, tile: Tile) {
    let pos = tiles.iter().position(|x| *x == tile);
    if pos.is_some() {
        tiles.remove(pos.unwrap());
    }
}
