use crate::grid::grid::Grid;
use crate::moveable::Moveable;
use quicksilver::geom::Vector;
use rand::Rng;

pub struct Monster {
	pub location: Moveable,
	pub size: usize,
	pub health: isize,
	pub damage: isize,
	pub speed: f32,
}
impl Monster {
	pub fn new(location: Vector) -> Self {
		Self {
			location: Moveable::new(location),
			size: 15,
			health: 10,
			damage: 10,
			speed: 5.,
		}
	}
	pub fn move_a_bit(&mut self, grid: &Grid) {
		let mut rng = rand::thread_rng();
		self.location
			.move_some(rng.gen(), self.speed, grid, self.size);
	}
}
