use super::in_game::InGameScreen;
use super::screen::Screen;

use crate::player::check_multiple_pressed;
use quicksilver::geom::Rectangle;
use quicksilver::geom::Transform;
use quicksilver::graphics::Font;
use quicksilver::graphics::FontStyle;
use quicksilver::graphics::Image;
use quicksilver::input::Key;
use quicksilver::lifecycle::Window;
use quicksilver::prelude::Img;
use quicksilver::Result;

pub struct DeadScreen {
    rendered_score: Image,
    rendered_dead_text: Image,
}

impl DeadScreen {
    pub fn new(score: u64, font: &Font, style: &FontStyle) -> Result<Self> {
        let rendered_score = font.render(&score.to_string(), &style)?;
        let rendered_dead_text =
            font.render("You died, press Esc to continue\nYour score:", &style)?;
        Ok(DeadScreen {
            rendered_dead_text,
            rendered_score,
        })
    }
}

impl Screen for DeadScreen {
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
        window.draw_ex(
            &Rectangle::new((200, 150), (380, 200)),
            Img(&self.rendered_dead_text),
            Transform::IDENTITY,
            2,
        );
        window.draw_ex(
            &Rectangle::new((350, 350), (100, 100)),
            Img(&self.rendered_score),
            Transform::IDENTITY,
            1,
        );
        return Ok(());
    }
}
