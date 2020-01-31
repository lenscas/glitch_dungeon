use super::screen::Screen;
use crate::game_state::GameState;
use quicksilver::graphics::Font;
use quicksilver::graphics::FontStyle;
use quicksilver::lifecycle::Window;
use quicksilver::Result;

use super::DeadScreen;
use crate::game_state::StateAction;
pub struct InGameScreen {
    state: GameState,
}
impl Screen for InGameScreen {
    fn update(
        &mut self,
        window: &mut Window,
        font: &Font,
        style: &FontStyle,
    ) -> Result<Option<Box<dyn Screen>>> {
        match self.state.update(window, font, style)? {
            StateAction::NextLevel => {
                self.state.reset(font, style)?;
                Ok(None)
            }
            StateAction::Die => Ok(Some(Box::new(DeadScreen::new(
                self.state.score,
                font,
                style,
            )?))),
            StateAction::None => Ok(None),
        }
    }

    fn draw(&self, window: &mut Window, _: &Font, _: &FontStyle) -> Result<()> {
        self.state.draw(window);
        Ok(())
    }
}
impl InGameScreen {
    pub fn new(font: &Font, style: &FontStyle) -> Result<Self> {
        let state = GameState::new(font, style)?;
        Ok(Self { state })
    }
}
