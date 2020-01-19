use crate::player::get_random_name;
use crate::player::Gun;
use quicksilver::graphics::{Font, FontStyle};
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
            rng.gen_range(-5, 5),
            rng.gen_range(10., 20.),
            font,
            style,
            &get_random_name(),
            rng.gen(),
        )?))
    }
}
