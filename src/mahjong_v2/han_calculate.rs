trait Yaku {
  fn concealed_only(&self) -> bool;
  fn riichi_only(&self) -> bool;
  fn han(&self) -> i32;
}

enum MahjongYaku {
  Riichi,
  Ippatsu
}

impl Yaku for MahjongYaku {
  fn concealed_only(&self) -> bool {
    match self {
      MahjongYaku::Riichi => true,
      MahjongYaku::Ippatsu => true,
      _ => false
    }
  }

  fn riichi_only(&self) -> bool {
      match self {
        MahjongYaku::Riichi => true,
        MahjongYaku::Ippatsu => true,
        _ => false
      }
  }

  fn han(&self) -> i32 {
    match self {
      MahjongYaku::Riichi => 1,
      MahjongYaku::Ippatsu => 1,
      _ => 1
    }
  }
}