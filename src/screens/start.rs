use super::screen::Screen;
use quicksilver::geom::Rectangle;
use quicksilver::graphics::Font;
use quicksilver::graphics::FontStyle;
use quicksilver::graphics::Image;
use quicksilver::lifecycle::Window;
use quicksilver::prelude::Img;
use quicksilver::Result;

use super::in_game::InGameScreen;
use crate::player::check_multiple_pressed;
use quicksilver::input::Key;
pub struct StartScreen {
    rendered_main: Image,
}

impl StartScreen {
    pub fn new() -> Result<Self> {
        let rendered_main = Image::from_bytes(include_bytes!("../../static/start.png"))?;
        Ok(Self { rendered_main })
    }
}

impl Screen for StartScreen {
    fn update(
        &mut self,
        window: &mut Window,
        font: &Font,
        style: &FontStyle,
    ) -> Result<Option<Box<dyn Screen>>> {
        let board = window.keyboard();
        if check_multiple_pressed(&board, &[Key::Escape, Key::Return]) {
            return Ok(Some(Box::new(InGameScreen::new(font, style)?)));
        }
        Ok(None)
    }

    fn draw(&self, window: &mut Window, _: &Font, _: &FontStyle) -> Result<()> {
        window.draw(
            &Rectangle::new((0, 0), (800, 600)),
            Img(&self.rendered_main),
        );
        Ok(())
    }
}
