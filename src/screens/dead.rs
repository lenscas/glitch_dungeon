use super::screen::Screen;
use quicksilver::graphics::Font;
use quicksilver::graphics::FontStyle;
use quicksilver::lifecycle::Window;
use quicksilver::Result;
pub struct DeadScreen {}

impl Screen for DeadScreen {
    fn update(&mut self, window: &mut Window, font: &Font, style: &FontStyle) -> Result<()> {
        Ok(())
    }
    fn draw(&self, window: &mut Window, font: &Font, style: &FontStyle) -> Result<()> {
        Ok(())
    }
}
