use crate::grid::grid::Grid;
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Background::Col, Color},
    input::{ButtonState, GamepadAxis, GamepadButton, Key, MouseButton},
    lifecycle::{run, Asset, Event, Settings, State, Window},
    Future, Result,
};

const CELL_SIZE: usize = 32;
mod grid;

pub struct MainState {
    grid: Grid,
    location: Vector,
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
    pub fn grid_to_screen(&self, loc: &(usize, usize)) -> (f32, f32) {
        let cell_size = CELL_SIZE as f32;
        let width = 800. / cell_size;
        let height = 600. / cell_size;
        //(loc.x as f32 - (self.location.x as f32 - width as f32 / 2.)) * cell_size as f32;
        let x = (loc.0 as f32 - (self.location.x / cell_size - width / 2.)) * cell_size as f32;
        let y = (loc.1 as f32 - (self.location.y / cell_size - height / 2.)) * cell_size as f32;
        (x, y)
    }
    pub fn pos_to_full_square_on_grid(&mut self, loc: &(usize, usize)) -> Rectangle {
        let screen_pos = self.grid_to_screen(loc);
        let cell_sizef = CELL_SIZE as f32;
        Rectangle::new(screen_pos, (cell_sizef, cell_sizef))
    }
    pub fn get_outer_cell_points(&self) -> ((usize, usize), (usize, usize)) {
        let height = 600;
        let width = 800;
        let start_x = Self::calc_start(self.location.x / CELL_SIZE as f32, 800 / CELL_SIZE);
        let start_y = Self::calc_start(self.location.y / CELL_SIZE as f32, 600 / CELL_SIZE);
        let end_x = 1 + start_x + width;
        let end_y = 1 + start_y + height;
        ((start_x, start_y), (end_x, end_y))
    }
}
impl State for MainState {
    fn new() -> Result<Self> {
        let grid = Grid::new(30, 30);
        let start = grid.start;
        Ok(Self {
            grid,
            location: Vector::new((start.0 * CELL_SIZE) as i32, (start.1 * CELL_SIZE) as i32),
        })
    }
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        let (start, end) = self.get_outer_cell_points();
        let part = self.grid.get_part(start, end);
        part.iter().enumerate().for_each(|(key, tile)| {
            let (loc2, tile) = tile;
            let loc = self.grid_to_screen(loc2);
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
            window.draw(&Rectangle::new(loc, (32, 32)), Col(to_draw))
        });
        Ok(())
    }
    fn update(&mut self, _window: &mut Window) -> Result<()> {
        Ok(())
    }
    fn event(&mut self, event: &Event, _window: &mut Window) -> Result<()> {
        match event {
            Event::Key(key, state) => match (key, state) {
                (Key::W, state)
                    if *state == ButtonState::Pressed || *state == ButtonState::Held =>
                {
                    self.location.y -= 5.;
                }
                (Key::S, state)
                    if *state == ButtonState::Pressed || *state == ButtonState::Held =>
                {
                    self.location.y += 5.;
                }
                (Key::D, state)
                    if *state == ButtonState::Pressed || *state == ButtonState::Held =>
                {
                    self.location.x += 5.;
                }
                (Key::A, state)
                    if *state == ButtonState::Pressed || *state == ButtonState::Held =>
                {
                    self.location.x -= 5.;
                }
                (_, _) => {}
            },
            _ => {}
        }
        Ok(())
    }
}

pub fn main() {
    run::<MainState>("Glitch Dungeon", Vector::new(800, 600), Settings::default());
}
