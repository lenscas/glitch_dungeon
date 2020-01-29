use crate::gun::get_random_name;
use crate::gun::Gun;
use crate::player::Player;
use quicksilver::geom::Rectangle;
use quicksilver::geom::Transform;
use quicksilver::graphics::{Background::Col, Color, Font, FontStyle};
use quicksilver::lifecycle::Window;
use quicksilver::Result;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Tile {
    pub is_start: bool,
    pub is_end: bool,
    pub can_move: bool,
    pub has_gun: bool,
}
impl Tile {
    pub fn get_gun(&self, font: &Font, style: &FontStyle) -> Result<Option<Gun>> {
        if !self.has_gun {
            return Ok(None);
        }
        let mut rng = rand::thread_rng();
        let mut patterns = Vec::new();
        for _ in 1..rng.gen_range(2, 4) {
            let mut pattern = Vec::new();
            for _ in 1..rng.gen_range(2, 4) {
                pattern.push(rng.gen_range(0, 4))
            }
            patterns.push(pattern)
        }

        Ok(Some(Gun::new(
            rng.gen_range(15, 25),
            patterns,
            rng.gen_range(-10, 10),
            rng.gen_range(10., 20.),
            font,
            style,
            &get_random_name(),
            rng.gen(),
        )?))
    }

    pub fn draw(&self, loc2: (usize, usize), window: &mut Window, z: i32, player: &Player) {
        let loc = player.grid_to_screen(&(loc2.0 as f32, loc2.1 as f32));
        let to_draw = if self.has_gun {
            Color::YELLOW
        } else if self.can_move {
            if self.is_start {
                Color::PURPLE
            } else if self.is_end {
                Color::GREEN
            } else {
                Color::from_rgba(128, 64, 128, 1.)
            }
        } else {
            Color::BLACK
        };
        let rec = Rectangle::new(loc, (32, 32));
        window.draw_ex(&rec, Col(to_draw), Transform::IDENTITY, z);
    }
}
