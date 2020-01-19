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

use rand::distributions::Standard;
use rand::seq::SliceRandom;
use rand::Rng;

use rand::distributions::Distribution;

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
			{
				let mut rng = rand::thread_rng();
				let mut patterns = Vec::new();
				for _ in 1..4 {
					let mut pattern = Vec::new();
					for _ in 1..4 {
						pattern.push(rng.gen_range(0, 4))
					}
					patterns.push(pattern)
				}

				Gun::new(
					rng.gen_range(15, 25),
					patterns,
					rng.gen_range(-5, 5),
					rng.gen_range(10., 20.),
					font,
					style,
					&get_random_name(),
					rng.gen(),
				)?
			},
			{
				let mut rng = rand::thread_rng();
				let mut patterns = Vec::new();
				for _ in 1..4 {
					let mut pattern = Vec::new();
					for _ in 1..4 {
						pattern.push(rng.gen_range(0, 4))
					}
					patterns.push(pattern)
				}

				Gun::new(
					rng.gen_range(15, 25),
					patterns,
					rng.gen_range(-5, 5),
					rng.gen_range(10., 20.),
					font,
					style,
					&get_random_name(),
					rng.gen(),
				)?
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
	pub fn update(
		&mut self,
		window: &Window,
		grid: &mut Grid,
		font: &Font,
		style: &FontStyle,
	) -> Result<(u64, Action)> {
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
		let mut extra_points = 0;
		let current = grid.get_cell(self.location.cell_loc);
		if self.shoot_timer > 0 {
			self.shoot_timer -= 1;
		}
		if let Some(current) = &current {
			if current.1.has_gun {
				if let Some(gun) = grid.get_gun(&self.location.cell_loc, font, style)? {
					self.guns.push(gun);
					extra_points += 20;
					if self.guns.len() > 4 {
						self.guns.remove(0);
					}
				}
			}
		}
		Ok((
			extra_points,
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
				.unwrap_or(Action::None),
		))
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

pub fn get_random_name() -> String {
	let glitch_chars: Vec<char> = vec![
		'█', '█', '█', '▒', '▒', '░', '█', '█', '▒', '௵', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
		'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'F', 'W', 'X', 'Y', 'Z',
		'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
		's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0',
		'!', '@', '#', '$', '%', '^', '&', '*', '(', ')',
	];
	let mut rng = rand::thread_rng();
	let mut name = Vec::new();
	for _ in 0..rng.gen_range(5, 8) {
		name.push(glitch_chars.choose(&mut rng).expect("No chars available"));
	}
	name.into_iter().collect()
}

#[derive(Clone, Debug)]
pub enum Action {
	Shoot(Gun),
	NextScreen,
	None,
}
#[derive(Clone, Debug)]
pub struct Gun {
	pub cooldown: usize,
	pub patterns: Vec<Vec<i8>>,
	pub damage: isize,
	pub speed: f32,
	pub rendered_name: Image,
	pub shape: ShapeChoise,
	pub rendered_patterns: Vec<Image>,
	pub rendered_damage: Image,
	pub rendered_cooldown: Image,
}
impl Gun {
	pub fn new(
		cooldown: usize,
		patterns: Vec<Vec<i8>>,
		damage: isize,
		speed: f32,
		font: &Font,
		style: &FontStyle,
		name: &str,
		shape: ShapeChoise,
	) -> Result<Self> {
		let rendered_name = font.render(name, style)?;
		let mut rendered_patterns = Vec::new();
		for pattern in &patterns {
			let pattern: String = pattern
				.iter()
				.map(|v| {
					if *v < 0 {
						v + 4
					} else if *v > 3 {
						v - 4
					} else {
						*v
					}
				})
				.map(|v| {
					if v == 0 {
						'F'
					} else if v == 1 {
						'R'
					} else if v == 2 {
						'B'
					} else {
						'L'
					}
				})
				.collect();
			rendered_patterns.push(font.render(&pattern, style)?);
		}
		let rendered_cooldown = font.render(&cooldown.to_string(), style)?;
		let rendered_damage = font.render(&damage.to_string(), style)?;
		Ok(Gun {
			rendered_name,
			speed,
			damage,
			cooldown,
			patterns,
			rendered_patterns,
			rendered_cooldown,
			rendered_damage,
			shape,
		})
	}
}
#[derive(Debug, Clone, Copy)]
pub enum ShapeChoise {
	Rectangle,
	Circle,
	Triangle,
}
impl Distribution<ShapeChoise> for Standard {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ShapeChoise {
		rng.gen_range(0, 3).into()
	}
}
impl From<u8> for ShapeChoise {
	fn from(v: u8) -> ShapeChoise {
		match v {
			0 => ShapeChoise::Rectangle,
			1 => ShapeChoise::Circle,
			_ => ShapeChoise::Triangle,
		}
	}
}
