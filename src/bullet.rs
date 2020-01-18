use crate::grid::dir::Dir;
use crate::grid::grid::Grid;
use crate::moveable::Moveable;
use quicksilver::geom::Vector;

pub struct Bullet {
	pub location: Moveable,
	pub speed: f32,
	pub direction: Dir,
	pub size: usize,
}
impl Bullet {
	pub fn new(location: Vector, speed: f32, dir: Dir) -> Self {
		Self {
			location: Moveable::new_not_center(location),
			speed,
			direction: dir,
			size: 20,
		}
	}
	pub fn update(&mut self, grid: &Grid) -> bool {
		self.location
			.move_some(self.direction, self.speed, grid, self.size)
			.is_some()
	}
}
