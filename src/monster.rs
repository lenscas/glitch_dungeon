use crate::grid::grid::Grid;
use crate::grid::Dir;
use crate::moveable::Moveable;
use crate::player::Player;
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
	pub damage_cooldown: usize,
}
impl Monster {
	pub fn new(location: Vector, font: &Font, style: &FontStyle) -> Result<Self> {
		let mut rng = rand::thread_rng();
		let health = rng.gen_range(-10, 10);
		let rendered_health = font.render(&health.to_string(), style)?;
		Ok(Self {
			location: Moveable::new(location),
			size: 15,
			health,
			damage: 5,
			speed: 5.,
			started_negative: health < 0,
			rendered_health,
			damage_cooldown: 0,
		})
	}
	pub fn move_a_bit(&mut self, grid: &Grid, player: &Player) {
		if self.damage_cooldown > 0 {
			self.damage_cooldown -= 1;
		}
		let mut rng = rand::thread_rng();
		let dir = if rng.gen_range(0, 10) > 7 {
			if self.location.cell_loc.0 > player.location.cell_loc.0 {
				Dir::Left
			} else if self.location.cell_loc.0 < player.location.cell_loc.0 {
				Dir::Right
			} else if self.location.cell_loc.1 > player.location.cell_loc.1 {
				Dir::Up
			} else if self.location.cell_loc.1 < player.location.cell_loc.1 {
				Dir::Down
			} else {
				rng.gen()
			}
		} else {
			rng.gen()
		};
		self.location.move_some(dir, self.speed, grid, self.size);
	}
	pub fn get_damage(&mut self, damage: isize, font: &Font, style: &FontStyle) -> Result<bool> {
		if self.damage_cooldown > 0 {
			return Ok(self.is_alive());
		}
		if (!self.started_negative) && damage < 0 {
			if self.health - damage < 12 {
				self.health -= damage;
				self.damage_cooldown = 20;
				self.rendered_health = font.render(&self.health.to_string(), style)?;
			}
		} else if self.started_negative && damage > 0 {
			if self.health - damage > -12 {
				self.health -= damage;
				self.damage_cooldown = 20;
				self.rendered_health = font.render(&self.health.to_string(), style)?;
			}
		} else {
			self.health -= damage;
			self.damage_cooldown = 20;
			self.rendered_health = font.render(&self.health.to_string(), style)?;
		}
		Ok(self.is_alive())
	}
	pub fn is_alive(&self) -> bool {
		(self.started_negative && self.health < 0) || ((!self.started_negative) && self.health > 0)
	}
}
