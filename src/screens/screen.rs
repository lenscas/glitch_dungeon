use quicksilver::graphics::Font;
use quicksilver::graphics::FontStyle;
use quicksilver::lifecycle::Window;
use quicksilver::Result;
pub trait Screen {
    fn update(&mut self, window: &mut Window, font: &Font, style: &FontStyle) -> Result<()>;
    fn draw(&self, window: &mut Window, font: &Font, style: &FontStyle) -> Result<()>;
}
