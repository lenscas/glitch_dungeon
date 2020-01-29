use quicksilver::graphics::Font;
use quicksilver::graphics::FontStyle;
use quicksilver::graphics::Image;
use quicksilver::Result;
use rand::distributions::Distribution;
use rand::distributions::Standard;
use rand::Rng;

use rand::seq::SliceRandom;

pub fn get_random_name() -> String {
    let glitch_chars: Vec<char> = vec![
        '💐', '█', '█', '█', '▒', '▒', '░', '█', '█', '▒', '௵', 'A', 'B', 'C', 'D', 'E', 'F', 'G',
        'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'F', 'W', 'X', 'Y',
        'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
        'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        '0', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')',
    ];
    let mut rng = rand::thread_rng();
    let mut name = Vec::new();
    for _ in 0..rng.gen_range(5, 8) {
        name.push(glitch_chars.choose(&mut rng).expect("No chars available"));
    }
    name.into_iter().collect()
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
    pub fn new_random(font: &Font, style: &FontStyle) -> Result<Self> {
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
            rng.gen_range(-7, 7),
            rng.gen_range(10., 20.),
            font,
            style,
            &get_random_name(),
            rng.gen(),
        )
    }
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
        let rendered_name = font.render(&format!("Name: {:0}", name), style)?;
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
        let rendered_cooldown = font.render(&format!("Cooldown: {:0}", &cooldown), style)?;
        let rendered_damage = font.render(&format!("Damage: {:0}", damage), style)?;
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
