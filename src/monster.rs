use crate::grid::grid::Grid;
use crate::moveable::Moveable;
use quicksilver::geom::Vector;
use quicksilver::graphics::Font;
use quicksilver::graphics::FontStyle;
use quicksilver::graphics::Image;
use quicksilver::Result;
use rand::Rng;

pub struct Monster {
	pub location: Moveable,
	pub size: usize,
	pub health: isize,
	pub started_negative: bool,
	pub damage: isize,
	pub speed: f32,
	pub rendered_health: Image,
}
impl Monster {
	pub fn new(location: Vector, font: &Font, style: &FontStyle) -> Result<Self> {
		let mut rng = rand::thread_rng();
		let health = rng.gen_range(-25, 25);
		let rendered_health = font.render(&health.to_string(), style)?;
		Ok(Self {
			location: Moveable::new(location),
			size: 15,
			health,
			damage: 10,
			speed: 5.,
			started_negative: health < 0,
			rendered_health,
		})
	}
	pub fn move_a_bit(&mut self, grid: &Grid) {
		let mut rng = rand::thread_rng();
		self.location
			.move_some(rng.gen(), self.speed, grid, self.size);
	}
	pub fn get_damage(&mut self, damage: isize, font: &Font, style: &FontStyle) -> Result<bool> {
		self.health -= damage;
		self.rendered_health = font.render(&self.health.to_string(), style)?;
		Ok(self.is_alive())
	}
	pub fn is_alive(&self) -> bool {
		(self.started_negative && self.health < 0) || ((!self.started_negative) && self.health > 0)
	}
}
