use rand::distributions::Distribution;
use rand::distributions::Standard;
use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl Distribution<Dir> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Dir {
        rng.gen_range(0, 4).into()
    }
}
impl From<u8> for Dir {
    fn from(v: u8) -> Dir {
        match v {
            0 => Dir::Up,
            1 => Dir::Left,
            2 => Dir::Down,
            _ => Dir::Right,
        }
    }
}
impl From<Dir> for u8 {
    fn from(v: Dir) -> u8 {
        match v {
            Dir::Up => 0,
            Dir::Left => 1,
            Dir::Down => 2,
            Dir::Right => 3,
        }
    }
}
