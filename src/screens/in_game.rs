use super::screen::Screen;
use crate::game_state::GameState;
use quicksilver::graphics::Font;
use quicksilver::graphics::FontStyle;
use quicksilver::lifecycle::Window;
use quicksilver::Result;
pub struct InGameScreen {
    state: GameState,
}
impl Screen for InGameScreen {
    fn update(&mut self, window: &mut Window, font: &Font, style: &FontStyle) -> Result<()> {
        unimplemented!()
    }
    fn draw(&self, window: &mut Window, font: &Font, style: &FontStyle) -> Result<()> {
        unimplemented!()
    }
}
