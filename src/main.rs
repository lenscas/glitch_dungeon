use quicksilver::graphics::Image;
use quicksilver::{
    geom::Vector,
    graphics::{Color, Font, FontStyle},
    lifecycle::{run, Event, Settings, State, Window},
    Result,
};
use screens::screen::Screen;
use screens::start::StartScreen;

const CELL_SIZE: usize = 32;
const PLAYER_SIZE: usize = 16;
const GRID_SIZE: usize = 30;

mod bullet;
mod game_state;
mod grid;
mod gun;
mod monster;
mod moveable;
mod player;
mod screens;

pub fn calc_start(cam: f32, line_size: usize) -> usize {
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

pub struct MainState {
    screen: Box<dyn Screen>,
    font: Font,
    default_style: FontStyle,

    _rendered_pattern_reminder: Image,
}
impl State for MainState {
    fn new() -> Result<Self> {
        let font = Font::from_bytes(include_bytes!("../static/font.ttf").to_vec())?;
        let style = FontStyle::new(100.0, Color::WHITE);

        let screen = Box::new(StartScreen::new()?);
        let rendered_pattern_reminder = font.render("Patterns", &style)?;
        Ok(Self {
            screen,
            font,
            default_style: style,
            _rendered_pattern_reminder: rendered_pattern_reminder,
        })
    }
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        self.screen.draw(window, &self.font, &self.default_style)?;
        Ok(())
    }
    fn update(&mut self, window: &mut Window) -> Result<()> {
        let next_screen = self
            .screen
            .update(window, &self.font, &self.default_style)?;
        if let Some(next_screen) = next_screen {
            self.screen = next_screen;
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
