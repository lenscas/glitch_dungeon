use crate::grid::grid::Grid;
use crate::grid::Dir;
use crate::moveable::Moveable;
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

pub struct Player {
	pub location: Moveable,
	pub speed: f32,
	pub dir: Dir,
	pub health: isize,
	pub invis_timer: usize,
}
impl Player {
	pub fn new(location: Vector) -> Self {
		Self {
			location: Moveable::new(location),
			speed: 10.,
			dir: Dir::Up,
			health: 100,
			invis_timer: 30,
		}
	}
	pub fn reset_location(&mut self, location: Vector) {
		self.location.reset_location(location);
	}
	pub fn update(&mut self, window: &Window, grid: &Grid) -> Action {
		let board = window.keyboard();
		if check_multiple(board, &[Key::A]) {
			self.location
				.move_some(Dir::Left, self.speed, grid, PLAYER_SIZE);
		}
		if check_multiple(board, &[Key::D]) {
			self.location
				.move_some(Dir::Right, self.speed, grid, PLAYER_SIZE);
		}
		if check_multiple(board, &[Key::W]) {
			self.location
				.move_some(Dir::Up, self.speed, grid, PLAYER_SIZE);
		}
		if check_multiple(board, &[Key::S]) {
			self.location
				.move_some(Dir::Down, self.speed, grid, PLAYER_SIZE);
		}
		if check_multiple(board, &[Key::Up]) {
			self.dir = Dir::Up
		}
		if check_multiple(board, &[Key::Down]) {
			self.dir = Dir::Down
		}
		if check_multiple(board, &[Key::Left]) {
			self.dir = Dir::Left
		}
		if check_multiple(board, &[Key::Right]) {
			self.dir = Dir::Right
		}
		let current = grid.get_cell(self.location.cell_loc);
		current
			.and_then(|(_, tile)| {
				if tile.is_end {
					Some(Action::NextScreen)
				} else {
					None
				}
			})
			.or_else(|| {
				if check_multiple(board, &[Key::F, Key::Space]) {
					return Some(Action::Shoot);
				} else {
					None
				}
			})
			.unwrap_or(Action::None)
	}

	pub fn get_rectangle(&self) -> Rectangle {
		let player_on_screen = self.grid_to_screen(&(
			self.location.location.x / CELL_SIZE as f32,
			self.location.location.y / CELL_SIZE as f32,
		));
		Rectangle::new(player_on_screen, (PLAYER_SIZE as i32, PLAYER_SIZE as i32))
			.with_center(player_on_screen)
	}
	pub fn grid_to_screen(&self, loc: &(f32, f32)) -> (f32, f32) {
		let cell_size = CELL_SIZE as f32;
		let width = 800. / cell_size;
		let height = 600. / cell_size;

		let x =
			(loc.0 as f32 - (self.location.location.x / cell_size - width / 2.)) * cell_size as f32;
		let y = (loc.1 as f32 - (self.location.location.y / cell_size - height / 2.))
			* cell_size as f32;
		(x, y)
	}
}
#[derive(Clone, Debug, PartialEq)]
pub enum Action {
	Shoot,
	NextScreen,
	None,
}
