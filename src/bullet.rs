use crate::grid::dir::Dir;
use crate::grid::grid::Grid;
use crate::moveable::Moveable;
use quicksilver::geom::Vector;

pub struct Bullet {
	pub location: Moveable,
	pub speed: f32,
	pub pattern: Vec<Dir>,
	pub size: usize,
	pub damage: isize,
}
impl Bullet {
	pub fn new_with_pattern(
		location: Vector,
		speed: f32,
		dir: Dir,
		pattern: Vec<i8>,
		damage: isize,
	) -> Self {
		let dir = u8::from(dir) as i8;
		let pattern = pattern
			.iter()
			.map(|v| dir - v)
			.map(|v| {
				if v < 0 {
					v + 4
				} else if v > 3 {
					v - 4
				} else {
					v
				}
			})
			.map(|v| v as u8)
			.map(|v| v.into())
			.collect();

		Self {
			location: Moveable::new_not_center(location),
			speed,
			pattern,
			size: 20,
			damage,
		}
	}
	pub fn update(&mut self, grid: &Grid) -> bool {
		for dir in &self.pattern {
			let hit = self
				.location
				.move_some(*dir, self.speed, grid, self.size)
				.is_some();
			if hit {
				return true;
			}
		}
		false
	}
}
