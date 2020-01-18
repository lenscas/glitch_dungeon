use rand::distributions::Distribution;
use rand::distributions::Standard;
use rand::Rng;

#[derive(Debug)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl Distribution<Dir> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Dir {
        match rng.gen_range(0, 4) {
            0 => Dir::Up,
            1 => Dir::Down,
            2 => Dir::Left,
            _ => Dir::Right,
        }
    }
}
