use strum_macros::EnumIter;

#[derive(Debug, PartialEq, EnumIter, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
pub enum Wind {
  East,
  South,
  North,
  West
}

impl Wind {
  pub fn next(&self) -> Wind {
    match self {
      Wind::East => Wind::South,
      Wind::South => Wind::West,
      Wind::West => Wind::North,
      Wind::North => Wind::East
    }
  }
}