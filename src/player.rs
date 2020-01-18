use crate::grid::grid::Grid;
use crate::grid::Dir;
use crate::moveable::Moveable;
use crate::CELL_SIZE;
use crate::PLAYER_SIZE;
use quicksilver::geom::Rectangle;
use quicksilver::graphics::{Font, FontStyle, Image};
use quicksilver::lifecycle::Window;
use quicksilver::Result;
use quicksilver::{
	geom::{Shape, Vector},
	{input::ButtonState, input::Keyboard, prelude::Key},
};

pub fn check_multiple(board: &Keyboard, to_check: &[Key]) -> bool {
	to_check
		.iter()
		.map(|v| board[*v])
		.map(|v| v.is_down())
		.any(|v| v)
}

pub fn check_multiple_pressed(board: &Keyboard, to_check: &[Key]) -> bool {
	to_check
		.iter()
		.map(|v| board[*v])
		.map(|v| v == ButtonState::Pressed)
		.any(|v| v)
}

pub struct Player {
	pub location: Moveable,
	pub speed: f32,
	pub dir: Dir,
	pub health: isize,
	pub invis_timer: usize,
	pub guns: Vec<Gun>,
	pub selected_gun: usize,
	pub shoot_timer: usize,
}
impl Player {
	pub fn new(location: Vector, font: &Font, style: &FontStyle) -> Result<Self> {
		let guns = vec![
			Gun {
				rendered_name: font.render("Sniper", style)?,
				speed: 3.,
				damage: 2,
				cooldown: 20,
				patterns: vec![vec![0], vec![0, 1], vec![0, -1]],
			},
			Gun {
				rendered_name: font.render("Minigun", style)?,
				speed: 4.,
				damage: 10,
				cooldown: 10,
				patterns: vec![vec![0]],
			},
			Gun {
				rendered_name: font.render("Nuke", style)?,
				speed: 4.,
				damage: -2,
				cooldown: 10,
				patterns: vec![vec![-2]],
			},
		];
		Ok(Self {
			location: Moveable::new(location),
			speed: 10.,
			dir: Dir::Up,
			health: 100,
			invis_timer: 30,
			guns,
			selected_gun: 0,
			shoot_timer: 0,
		})
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
		if check_multiple_pressed(board, &[Key::Q]) {
			if self.selected_gun == 0 {
				self.selected_gun = self.guns.len() - 1;
			} else {
				self.selected_gun -= 1;
			}
		}
		if check_multiple_pressed(board, &[Key::E]) {
			if self.selected_gun == self.guns.len() - 1 {
				self.selected_gun = 0;
			} else {
				self.selected_gun += 1;
			}
		}
		let current = grid.get_cell(self.location.cell_loc);
		if self.shoot_timer > 0 {
			self.shoot_timer -= 1;
		}
		current
			.and_then(|(_, tile)| {
				if tile.is_end {
					Some(Action::NextScreen)
				} else {
					None
				}
			})
			.or_else(|| {
				if check_multiple(board, &[Key::F, Key::Space]) && self.shoot_timer == 0 {
					let selected_gun = self.guns[self.selected_gun].clone();
					self.shoot_timer = selected_gun.cooldown;
					return Some(Action::Shoot(selected_gun));
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
#[derive(Clone, Debug)]
pub enum Action {
	Shoot(Gun),
	NextScreen,
	None,
}
#[derive(Clone, Debug)]
pub struct Gun {
	pub rendered_name: Image,
	pub cooldown: usize,
	pub patterns: Vec<Vec<i8>>,
	pub damage: isize,
	pub speed: f32,
}
