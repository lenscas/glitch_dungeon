use crate::grid::grid::Grid;
use crate::grid::tile::Tile;
use crate::player::Player;
use quicksilver::{
    geom::{Rectangle, Transform, Vector},
    graphics::{Background::Col, Color},
    lifecycle::{run, Event, Settings, State, Window},
    Result,
};

const CELL_SIZE: usize = 32;
const PLAYER_SIZE: usize = 16;
const GRID_SIZE: usize = 30;
mod grid;
mod player;

pub struct MainState {
    grid: Grid,
    player: Player,
    drawn_grid: Vec<(Rectangle, Tile)>,
}
impl MainState {
    fn calc_start(cam: f32, line_size: usize) -> usize {
        let cam = cam.floor() as usize;
        let halved = line_size / 2;
        if cam < halved || cam == 1 {
            0
        } else {
            let calced = cam - halved;
            if calced <= 1 {
                0
            } else {
                calced - 1
            }
        }
    }

    pub fn pos_to_full_square_on_grid(&mut self, loc: &(f32, f32)) -> Rectangle {
        let screen_pos = self.player.grid_to_screen(loc);
        let cell_sizef = CELL_SIZE as f32;
        Rectangle::new(screen_pos, (cell_sizef, cell_sizef))
    }
    pub fn get_outer_cell_points(&self) -> ((usize, usize), (usize, usize)) {
        let height = 600;
        let width = 800;
        let mid_point = {
            let mut mid_point = self.player.location.clone();
            if mid_point.x < 0. {
                mid_point.x = 0.;
            }
            if mid_point.y < 0. {
                mid_point.y = 0.;
            }
            mid_point
        };
        let start_x = Self::calc_start(mid_point.x / CELL_SIZE as f32, 800 / CELL_SIZE);
        let start_y = Self::calc_start(mid_point.y / CELL_SIZE as f32, 600 / CELL_SIZE);
        let end_x = 1 + start_x + width;
        let end_y = 1 + start_y + height;
        ((start_x, start_y), (end_x, end_y))
    }
}
impl State for MainState {
    fn new() -> Result<Self> {
        let grid = Grid::new(GRID_SIZE, GRID_SIZE);
        let start = grid.start;
        let loc = Vector::new((start.0 * CELL_SIZE) as i32, (start.1 * CELL_SIZE) as i32);
        Ok(Self {
            grid,
            player: Player::new(loc),
            drawn_grid: Vec::new(),
        })
    }
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        let (start, end) = self.get_outer_cell_points();
        let part = self.grid.get_part(start, end);
        let mut z = 0;
        self.drawn_grid = part
            .into_iter()
            .enumerate()
            .map(|(_, tile)| {
                let (loc2, tile) = tile;
                let loc = self.player.grid_to_screen(&(loc2.0 as f32, loc2.1 as f32));
                let to_draw = if tile.can_move {
                    if tile.is_start {
                        Color::PURPLE
                    } else if tile.is_end {
                        Color::RED
                    } else {
                        Color::GREEN
                    }
                } else {
                    Color::BLUE
                };
                let rec = Rectangle::new(loc, (32, 32));
                window.draw_ex(&rec, Col(to_draw), Transform::IDENTITY, z);
                z = z + 1;
                (rec, tile)
            })
            .collect();
        window.draw_ex(
            &self.player.get_rectangle(),
            Col(Color::WHITE),
            Transform::IDENTITY,
            z,
        );
        Ok(())
    }
    fn update(&mut self, window: &mut Window) -> Result<()> {
        let reached_end = self.player.update(window, &self.grid);
        if reached_end {
            self.grid = Grid::new(GRID_SIZE, GRID_SIZE);
            let start = self.grid.start;
            self.player.reset_location(Vector::new(
                (start.0 * CELL_SIZE) as i32,
                (start.1 * CELL_SIZE) as i32,
            ));
        }
        Ok(())
    }
    fn event(&mut self, _event: &Event, _window: &mut Window) -> Result<()> {
        Ok(())
    }
}

pub fn main() {
    run::<MainState>("Glitch Dungeon", Vector::new(800, 600), Settings::default());
}
