use crate::grid::grid::Grid;
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Background::Col, Color},
    lifecycle::{run, Asset, Event, Settings, State, Window},
    Future, Result,
};

mod grid;

pub struct MainState {
    grid: Grid,
}
impl State for MainState {
    fn new() -> Result<Self> {
        Ok(Self {
            grid: Grid::new(20, 20),
        })
    }
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        for (key, tile) in self.grid.tiles.iter().enumerate() {
            let loc = (((key % 20) * 20) as i32, ((key / 20) * 20) as i32);
            let color = if tile.can_move {
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
            window.draw(&Rectangle::new(loc, (20, 20)), Col(color))
        }
        Ok(())
    }
    fn update(&mut self, _window: &mut Window) -> Result<()> {
        Ok(())
    }
    fn event(&mut self, _event: &Event, _window: &mut Window) -> Result<()> {
        Ok(())
    }
}

pub fn main() {
    run::<MainState>("Glitch Dungeon", Vector::new(800, 600), Settings::default());
}
