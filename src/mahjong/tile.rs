use std::{fmt};

pub trait Tile {
    fn is_honor(self) -> bool;
    fn is_simple(self) -> bool;
    fn value(&self) -> String;
}

impl fmt::Display for dyn Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}