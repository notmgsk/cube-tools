use std::convert::TryFrom;

mod alg;
mod corner_pos;
mod edge_pos;
mod sticker_cube;

pub use alg::{parse_alg, Alg, Move};
pub use corner_pos::CornerPos;
pub use edge_pos::EdgePos;
pub use sticker_cube::StickerCube;

/// Represents a face of a cube.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Face {
  U,
  R,
  F,
  D,
  B,
  L,
}

impl TryFrom<char> for Face {
  type Error = ();

  fn try_from(c: char) -> Result<Self, Self::Error> {
    match c {
      'U' => Ok(Face::U),
      'R' => Ok(Face::R),
      'F' => Ok(Face::F),
      'D' => Ok(Face::D),
      'B' => Ok(Face::B),
      'L' => Ok(Face::L),
      _ => Err(()),
    }
  }
}

/// Represents a particular centre position on a cube.
#[derive(Clone, Copy, Debug)]
enum CentrePos {
  U,
  R,
  F,
  D,
  B,
  L,
}
