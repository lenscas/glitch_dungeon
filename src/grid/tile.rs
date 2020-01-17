use crate::grid::dir::Dir;

pub struct Tile {
	pub is_start: bool,
	pub is_end: bool,
	pub can_move: bool,
}
