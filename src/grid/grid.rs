use crate::grid::{Dir, Tile};
use rand::Rng;
use std::collections::HashMap;

pub struct Grid {
	pub tiles: Vec<Tile>,
	pub length: usize,
	pub height: usize,
}

impl Grid {
	pub fn new(length: usize, height: usize) -> Self {
		let mut grid = Vec::<Tile>::new();
		let amount = length * height;
		grid.reserve(amount);
		let mut rng = rand::thread_rng();
		let player_start: (usize, usize) = (rng.gen_range(0, length), rng.gen_range(0, height));
		let exit: (usize, usize) = (rng.gen_range(0, length), rng.gen_range(0, height));
		let mut path = Vec::<(usize, usize)>::new();
		path.push(player_start);
		loop {
			let _next: Dir = rng.gen();
			let at = path.last().unwrap();
			if *at == exit {
				break;
			} else {
				let mut new_at = (at.0, at.1);
				let random = rng.gen();
				println!("{:?}", random);
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
				println!("{:?}", new_at);
				if res {
					path.push(new_at);
				}
			}
		}

		let mut iter = path.iter().enumerate().peekable();
		let mut path = HashMap::new();
		while let Some((key, v)) = iter.next() {
			println!("{:?}", key);
			let loc = Grid::calc_cell(v, length, height);
			let is_last = iter.peek().is_none();
			let room = Tile {
				is_start: key == 1,
				is_end: is_last,
				can_move: true,
			};
			let room = path.entry(loc).or_insert(room);
		}
		for v in 0..length * height {
			grid.push(match path.remove(&v) {
				Some(x) => x,
				None => Tile {
					is_start: false,
					is_end: false,
					can_move: false,
				},
			});
		}
		Self {
			tiles: grid,
			length,
			height,
		}
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
}
