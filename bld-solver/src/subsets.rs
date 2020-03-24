use crate::State;
use cube::{CornerPos, StickerCube};
use cycles::{cycle_len, get_piece_cycles, Piece};

pub(crate) fn exec_3cycle<P: Piece + std::fmt::Debug>(
  c: &mut StickerCube,
  cycle: [P; 3],
) {
  assert_ne!(cycle[0].orient(), cycle[1].orient());
  assert_ne!(cycle[0].orient(), cycle[2].orient());
  assert_ne!(cycle[1].orient(), cycle[2].orient());
  let p0 = P::lookup(c, cycle[0]);
  let p1 = P::lookup(c, cycle[1]);
  let p2 = P::lookup(c, cycle[2]);
  P::set(c, cycle[2], p1);
  P::set(c, cycle[1], p0);
  P::set(c, cycle[0], p2);
}

pub(crate) fn try_3cycle<P: Piece + std::fmt::Debug>(
  state: &State,
) -> Option<(Vec<P>, State)> {
  let c = &state.cube;
  let buffer = P::oriented_iter().next().unwrap();

  let p0 = P::lookup(c, buffer);
  let p1 = P::lookup(c, p0);

  if buffer.orient() == p0.orient() || buffer.orient() == p1.orient() {
    return None;
  }

  let cycle = [buffer, p0, p1];
  let mut next_cube = c.clone();
  exec_3cycle(&mut next_cube, cycle);
  Some((cycle.to_vec(), State { cube: next_cube }))
}

fn exec_2twist<P: Piece + std::fmt::Debug>(c: &mut StickerCube, cycle: [P; 2]) {
  assert_ne!(cycle[0].orient(), cycle[1].orient());
  P::set(c, cycle[0].orient(), cycle[0].orient());
  P::set(c, cycle[1].orient(), cycle[1].orient());
}

pub(crate) fn try_2twist<P: Piece + std::fmt::Debug>(
  state: &State,
) -> Option<(Vec<P>, State)> {
  let pieces = get_piece_cycles::<P>(&state.cube);
  if !pieces.iter().all(|c| cycle_len(&c) == 1)
    || pieces.iter().filter(|c| cycle_len(&c) == 1).count() % 2 != 0
  {
    return None;
  }

  let cycle = [pieces[0][1], pieces[1][1]];
  let mut next_cube = state.cube.clone();
  exec_2twist(&mut next_cube, cycle);
  Some((cycle.to_vec(), State { cube: next_cube }))
}

fn exec_corner_3twist(c: &mut StickerCube, cycle: [CornerPos; 3]) {
  c.set_corner(cycle[0].orient(), cycle[0].orient());
  c.set_corner(cycle[1].orient(), cycle[1].orient());
  c.set_corner(cycle[2].orient(), cycle[2].orient());
}

pub(crate) fn try_corner_3twist(
  state: &State,
) -> Option<(Vec<CornerPos>, State)> {
  let c = &state.cube;
  let corners = get_piece_cycles::<CornerPos>(&c);
  if corners.len() < 3
    || !corners.iter().all(|c| cycle_len(&c) == 1)
    || corners.iter().filter(|c| cycle_len(&c) == 1).count() % 2 != 1
  {
    return None;
  }

  let cycle = [corners[0][1], corners[1][1], corners[2][1]];
  let mut next_cube = c.clone();
  exec_corner_3twist(&mut next_cube, cycle);
  Some((cycle.to_vec(), State { cube: next_cube }))
}

#[cfg(test)]
mod tests {
  use super::*;
  use cube::CornerPos::*;
  use cube::EdgePos::*;

  #[test]
  fn basic_3cycle() {
    let mut c = StickerCube::solved();
    exec_3cycle(&mut c, [URF, RDF, LDB]);
    let result = try_3cycle(&State { cube: c });
    assert_eq!(
      Some((
        vec![URF, LDB, RDF],
        State {
          cube: StickerCube::solved()
        }
      )),
      result
    );

    let mut c = StickerCube::solved();
    exec_3cycle(&mut c, [UF, UB, DF]);
    let result = try_3cycle(&State { cube: c });
    assert_eq!(
      Some((
        vec![UF, DF, UB],
        State {
          cube: StickerCube::solved()
        }
      )),
      result
    );
  }

  #[test]
  fn basic_2twist() {
    let mut c = StickerCube::solved();
    c.set_corner(URF, FUR);
    c.set_corner(UFL, FLU);
    let result = try_2twist(&State { cube: c });
    assert_eq!(
      Some((
        vec![FUR, FLU],
        State {
          cube: StickerCube::solved()
        }
      )),
      result
    );

    let mut c = StickerCube::solved();
    c.set_edge(UF, FU);
    c.set_edge(UR, RU);
    let result = try_2twist(&State { cube: c });
    assert_eq!(
      Some((
        vec![FU, RU],
        State {
          cube: StickerCube::solved()
        }
      )),
      result
    );
  }

  #[test]
  fn basic_3twist() {
    let mut c = StickerCube::solved();
    c.set_corner(URF, FUR);
    c.set_corner(UFL, FLU);
    c.set_corner(ULB, LBU);
    let result = try_corner_3twist(&State { cube: c });
    assert_eq!(
      Some((
        vec![FUR, FLU, LBU],
        State {
          cube: StickerCube::solved()
        }
      )),
      result
    );
  }
}
