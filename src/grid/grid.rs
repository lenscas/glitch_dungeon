use crate::grid::{Dir, Tile};
use crate::gun::Gun;
use quicksilver::graphics::Font;
use quicksilver::graphics::FontStyle;
use quicksilver::Result;
use rand::Rng;
use std::collections::HashMap;

pub struct Grid {
    pub tiles: Vec<Tile>,
    pub length: usize,
    pub height: usize,
    pub start: (usize, usize),
}

impl Grid {
    pub fn new(length: usize, height: usize) -> Result<Self> {
        let mut grid = Vec::<Tile>::new();
        let amount = length * height;
        grid.reserve(amount);
        let mut rng = rand::thread_rng();
        let player_start: (usize, usize) = (rng.gen_range(0, length), rng.gen_range(0, height));
        let exit: (usize, usize) = (rng.gen_range(0, length), rng.gen_range(0, height));
        let mut path = Vec::<(usize, usize)>::new();
        path.push((player_start.0, player_start.1));
        loop {
            let at = path.last().unwrap();
            if *at == exit {
                break;
            } else {
                let mut new_at = (at.0, at.1);
                let random = rng.gen();
                let res = match random {
                    Dir::Down => {
                        if new_at.1 > 0 {
                            new_at.1 -= 1;
                            true
                        } else {
                            false
                        }
                    }
                    Dir::Up => {
                        if new_at.1 < height {
                            new_at.1 += 1;
                            true
                        } else {
                            false
                        }
                    }
                    Dir::Left => {
                        if new_at.0 > 0 {
                            new_at.0 -= 1;
                            true
                        } else {
                            false
                        }
                    }
                    Dir::Right => {
                        if new_at.0 < length {
                            new_at.0 += 1;
                            true
                        } else {
                            false
                        }
                    }
                };
                if res {
                    path.push(new_at);
                }
            }
        }
        let mut iter = path.iter().enumerate().peekable();
        let mut path2 = HashMap::new();
        while let Some((key, v)) = iter.next() {
            let loc = Grid::calc_cell(v, length, height);
            let next = iter.peek();
            let is_last = next.is_none();
            let room = Tile {
                is_start: key == 0,
                is_end: is_last,
                can_move: true,
                has_gun: rng.gen_range(0, 100) < 2,
            };
            path2.insert(loc, room);
        }
        for v in 0..length * height {
            grid.push(match path2.remove(&v) {
                Some(x) => x,
                None => Tile {
                    is_start: false,
                    is_end: false,
                    can_move: false,
                    has_gun: rng.gen_range(0, 100) < 2,
                },
            });
        }
        Ok(Self {
            tiles: grid,
            length,
            height,
            start: player_start,
        })
    }
    fn calc_cell(point: &(usize, usize), length: usize, height: usize) -> usize {
        let mut x = point.0;
        let mut y = point.1;
        if x >= length {
            x = length - 1
        }

        if y >= height {
            y = height - 1
        };
        (y * length) + x
    }
    fn calc_cell_unbound(point: &(usize, usize), length: usize, _height: usize) -> usize {
        let x = point.0;
        let y = point.1;
        (y * length) + x
    }
    pub fn calc_pos_from_index(key: usize, length: usize, _height: usize) -> (usize, usize) {
        let res = ((key % length) as usize, (key / length) as usize);
        res
    }
    pub fn get_part(
        &self,
        start: (usize, usize),
        end: (usize, usize),
    ) -> Vec<((usize, usize), Tile)> {
        self.tiles
            .iter()
            .enumerate()
            .map(|(key, v)| {
                let loc = Grid::calc_pos_from_index(key, self.length, self.height);
                (loc, v)
            })
            .filter(|(loc, _)| {
                loc.0 >= start.0 as usize
                    && loc.0 <= end.0 as usize
                    && loc.1 >= start.1 as usize
                    && loc.1 <= end.1 as usize
            })
            .map(|(loc, v)| (loc, v.clone()))
            .collect()
    }
    pub fn get_cell(&self, cell: (usize, usize)) -> Option<((usize, usize), Tile)> {
        if cell.0 > self.length - 1 || cell.1 > self.height - 1 {
            return None;
        }
        let index = Grid::calc_cell_unbound(&cell, self.length, self.height);
        self.tiles.get(index).map(|v| {
            (
                Grid::calc_pos_from_index(index, self.length, self.height),
                v.clone(),
            )
        })
    }
    pub fn get_gun(
        &mut self,
        cell: &(usize, usize),
        font: &Font,
        style: &FontStyle,
    ) -> Result<Option<Gun>> {
        if cell.0 > self.length - 1 || cell.1 > self.height - 1 {
            return Ok(None);
        }
        let index = Grid::calc_cell_unbound(&cell, self.length, self.height);
        if let Some(tile) = self.tiles.get_mut(index) {
            let gun = tile.get_gun(font, style);
            tile.has_gun = false;
            gun
        } else {
            Ok(None)
        }
    }
}
