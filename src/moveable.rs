use crate::grid::dir::Dir;
use crate::grid::grid::Grid;
use crate::grid::tile::Tile;
use crate::CELL_SIZE;
use quicksilver::geom::Vector;

pub fn sub_save(first: f32, second: f32) -> f32 {
    if first <= second {
        0.
    } else {
        first - second
    }
}
#[derive(Clone)]
pub struct Moveable {
    pub location: Vector,
    pub cell_loc: (usize, usize),
}
impl Moveable {
    pub fn new_not_center(location: Vector) -> Self {
        let cell_lock = (
            location.x as usize / CELL_SIZE,
            location.y as usize / CELL_SIZE,
        );
        Self {
            cell_loc: cell_lock,
            location: location,
        }
    }
    pub fn new(location: Vector) -> Self {
        let cell_lock = (
            location.x as usize / CELL_SIZE,
            location.y as usize / CELL_SIZE,
        );
        Self {
            cell_loc: cell_lock,
            location: Vector::new(
                location.x + CELL_SIZE as f32 / 2.,
                location.y + CELL_SIZE as f32 / 2.,
            ),
        }
    }
    pub fn move_some(
        &mut self,
        dir: Dir,
        speed: f32,
        grid: &Grid,
        moveable_size: usize,
    ) -> Option<Option<((usize, usize), Tile)>> {
        let moveable_size_as_f32 = moveable_size as f32;
        let half_moveable_size = moveable_size_as_f32 / 2.;
        match dir {
            Dir::Left => {
                let new_loc = Vector::new(sub_save(self.location.x, speed), self.location.y);
                if ((new_loc.x - half_moveable_size) / CELL_SIZE as f32).floor() as isize
                    != self.cell_loc.0 as isize
                {
                    if self.cell_loc.0 != 0 {
                        let next_cell = grid.get_cell((self.cell_loc.0 - 1, self.cell_loc.1));
                        match next_cell {
                            Some(next_cell) => {
                                if !(next_cell.1).can_move {
                                    self.location.x =
                                        (self.cell_loc.0 * CELL_SIZE + (moveable_size / 2)) as f32;
                                    return Some(Some(next_cell));
                                } else {
                                    self.location = new_loc;
                                    self.cell_loc =
                                        (new_loc.x as usize / CELL_SIZE, self.cell_loc.1);
                                }
                            }
                            None => return Some(None),
                        }
                    } else {
                        return Some(None);
                    }
                } else {
                    self.location = new_loc;
                }
            }
            Dir::Right => {
                let new_loc = Vector::new(self.location.x + speed, self.location.y);
                if ((new_loc.x + half_moveable_size) / CELL_SIZE as f32).floor() as usize
                    != self.cell_loc.0
                {
                    let next_cell = grid.get_cell((self.cell_loc.0 + 1, self.cell_loc.1));
                    match next_cell {
                        Some(next_cell) => {
                            if !(next_cell.1).can_move {
                                self.location.x = (self.cell_loc.0 * CELL_SIZE + CELL_SIZE
                                    - (moveable_size / 2))
                                    as f32;
                                return Some(Some(next_cell));
                            } else {
                                self.location = new_loc;
                                self.cell_loc = (new_loc.x as usize / CELL_SIZE, self.cell_loc.1);
                            }
                        }
                        None => return Some(None),
                    }
                } else {
                    self.location = new_loc;
                }
            }
            Dir::Up => {
                let new_loc = Vector::new(self.location.x, sub_save(self.location.y, speed));
                if ((new_loc.y - half_moveable_size) / CELL_SIZE as f32).floor() as isize
                    != self.cell_loc.1 as isize
                {
                    if self.cell_loc.1 != 0 {
                        let next_cell = grid.get_cell((self.cell_loc.0, self.cell_loc.1 - 1));
                        match next_cell {
                            Some(next_cell) => {
                                if !(next_cell.1).can_move {
                                    self.location.y =
                                        (self.cell_loc.1 * CELL_SIZE + (moveable_size / 2)) as f32;
                                    return Some(Some(next_cell));
                                } else {
                                    self.location = new_loc;
                                    self.cell_loc =
                                        (self.cell_loc.0, new_loc.y as usize / CELL_SIZE);
                                }
                            }
                            None => return Some(None),
                        }
                    } else {
                        return Some(None);
                    }
                } else {
                    self.location = new_loc;
                }
            }
            Dir::Down => {
                let new_loc = Vector::new(self.location.x, self.location.y + speed);
                if ((new_loc.y + half_moveable_size) / CELL_SIZE as f32).floor() as usize
                    != self.cell_loc.1 as usize
                {
                    let next_cell = grid.get_cell((self.cell_loc.0, self.cell_loc.1 + 1));
                    match next_cell {
                        Some(next_cell) => {
                            if !(next_cell.1).can_move {
                                self.location.y = (self.cell_loc.1 * CELL_SIZE + CELL_SIZE
                                    - (moveable_size / 2))
                                    as f32;
                                return Some(Some(next_cell));
                            } else {
                                self.location = new_loc;
                                self.cell_loc = (self.cell_loc.0, new_loc.y as usize / CELL_SIZE);
                            }
                        }
                        None => return Some(None),
                    }
                } else {
                    self.location = new_loc
                }
            }
        }
        None
    }
    pub fn reset_location(&mut self, location: Vector) {
        let cell_lock = (
            location.x as usize / CELL_SIZE,
            location.y as usize / CELL_SIZE,
        );
        self.cell_loc = cell_lock;
        self.location = Vector::new(
            location.x + CELL_SIZE as f32 / 2.,
            location.y + CELL_SIZE as f32 / 2.,
        );
    }
}
