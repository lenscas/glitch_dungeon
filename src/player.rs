use crate::grid::grid::Grid;
use crate::CELL_SIZE;
use crate::PLAYER_SIZE;
use quicksilver::geom::Rectangle;
use quicksilver::lifecycle::Window;
use quicksilver::{
	geom::{Shape, Vector},
	{input::Keyboard, prelude::Key},
};

pub fn check_multiple(board: &Keyboard, to_check: &[Key]) -> bool {
	to_check
		.iter()
		.map(|v| board[*v])
		.map(|v| v.is_down())
		.any(|v| v)
}

pub fn sub_save(first: f32, second: f32) -> f32 {
	if first <= second {
		0.
	} else {
		first - second
	}
}

pub struct Player {
	pub location: Vector,
	pub cell_loc: (usize, usize),
	pub speed: f32,
}
impl Player {
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
			speed: 10.,
		}
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
	pub fn update(&mut self, window: &Window, grid: &Grid) -> bool {
		let board = window.keyboard();
		let psize_as_f32 = PLAYER_SIZE as f32;
		let half_player_size = psize_as_f32 / 2.;
		if check_multiple(board, &[Key::Left, Key::A]) {
			let new_loc = Vector::new(sub_save(self.location.x, self.speed), self.location.y);
			if ((new_loc.x - half_player_size) / CELL_SIZE as f32).floor() as usize
				!= self.cell_loc.0
			{
				if self.cell_loc.0 != 0 {
					let next_cell = grid.get_cell((self.cell_loc.0 - 1, self.cell_loc.1));
					match next_cell {
						Some(next_cell) => {
							if !(next_cell.1).can_move {
								self.location.x =
									(self.cell_loc.0 * CELL_SIZE + (PLAYER_SIZE / 2)) as f32;
							} else {
								self.location = new_loc;
								self.cell_loc = (new_loc.x as usize / CELL_SIZE, self.cell_loc.1);
							}
						}
						None => {}
					}
				}
			} else {
				self.location = new_loc
			}
		}
		if check_multiple(board, &[Key::Right, Key::D]) {
			let new_loc = Vector::new(self.location.x + self.speed, self.location.y);
			if ((new_loc.x + half_player_size) / CELL_SIZE as f32).floor() as usize
				!= self.cell_loc.0
			{
				let next_cell = grid.get_cell((self.cell_loc.0 + 1, self.cell_loc.1));
				match next_cell {
					Some(next_cell) => {
						if !(next_cell.1).can_move {
							self.location.x = (self.cell_loc.0 * CELL_SIZE + CELL_SIZE
								- (PLAYER_SIZE / 2)) as f32;
						} else {
							self.location = new_loc;
							self.cell_loc = (new_loc.x as usize / CELL_SIZE, self.cell_loc.1);
						}
					}
					None => {}
				}
			} else {
				self.location = new_loc
			}
		}

		if check_multiple(board, &[Key::Up, Key::W]) {
			let new_loc = Vector::new(self.location.x, sub_save(self.location.y, self.speed));
			if ((new_loc.y - half_player_size) / CELL_SIZE as f32).floor() as usize
				!= self.cell_loc.1
			{
				if self.cell_loc.1 != 0 {
					let next_cell = grid.get_cell((self.cell_loc.0, self.cell_loc.1 - 1));
					match next_cell {
						Some(next_cell) => {
							if !(next_cell.1).can_move {
								self.location.y =
									(self.cell_loc.1 * CELL_SIZE + (PLAYER_SIZE / 2)) as f32;
							} else {
								self.location = new_loc;
								self.cell_loc = (self.cell_loc.0, new_loc.y as usize / CELL_SIZE);
							}
						}
						None => {}
					}
				}
			} else {
				self.location = new_loc
			}
		}
		if check_multiple(board, &[Key::Down, Key::S]) {
			let new_loc = Vector::new(self.location.x, self.location.y + self.speed);
			if ((new_loc.y + half_player_size) / CELL_SIZE as f32).floor() as usize
				!= self.cell_loc.1
			{
				let next_cell = grid.get_cell((self.cell_loc.0, self.cell_loc.1 + 1));
				match next_cell {
					Some(next_cell) => {
						if !(next_cell.1).can_move {
							self.location.y = (self.cell_loc.1 * CELL_SIZE + CELL_SIZE
								- (PLAYER_SIZE / 2)) as f32;
						} else {
							self.location = new_loc;
							self.cell_loc = (self.cell_loc.0, new_loc.y as usize / CELL_SIZE);
						}
					}
					None => {}
				}
			} else {
				self.location = new_loc
			}
		}
		let current = grid.get_cell(self.cell_loc);
		current.map(|(_, tile)| tile.is_end).unwrap_or(false)
	}

	pub fn get_rectangle(&self) -> Rectangle {
		let player_on_screen = self.grid_to_screen(&(
			self.location.x / CELL_SIZE as f32,
			self.location.y / CELL_SIZE as f32,
		));
		Rectangle::new(player_on_screen, (PLAYER_SIZE as i32, PLAYER_SIZE as i32))
			.with_center(player_on_screen)
	}
	pub fn grid_to_screen(&self, loc: &(f32, f32)) -> (f32, f32) {
		let cell_size = CELL_SIZE as f32;
		let width = 800. / cell_size;
		let height = 600. / cell_size;
		//(loc.x as f32 - (self.location.x as f32 - width as f32 / 2.)) * cell_size as f32;
		let x = (loc.0 as f32 - (self.location.x / cell_size - width / 2.)) * cell_size as f32;
		let y = (loc.1 as f32 - (self.location.y / cell_size - height / 2.)) * cell_size as f32;
		(x, y)
	}
}
