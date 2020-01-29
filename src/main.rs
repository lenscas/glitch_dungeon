use crate::grid::grid::Grid;
use crate::monster::Monster;
use crate::player::check_multiple_pressed;
use crate::player::Player;
use game_state::StateAction;
use quicksilver::graphics::Image;
use quicksilver::prelude::Background::Img;
use quicksilver::{
    geom::{Rectangle, Transform, Vector},
    graphics::{Color, Font, FontStyle},
    lifecycle::{run, Event, Settings, State, Window},
    prelude::Key,
    Result,
};
use rand::seq::SliceRandom;
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
    state: game_state::GameState,
    font: Font,
    default_style: FontStyle,
    rendered_score: Image,
    is_dead: bool,
    rendered_dead_text: Image,
    is_at_main: bool,

    _rendered_pattern_reminder: Image,
}
impl MainState {
    pub fn pos_to_full_square_on_grid(&mut self, loc: &(f32, f32)) -> Rectangle {
        let screen_pos = self.state.player.grid_to_screen(loc);
        let cell_sizef = CELL_SIZE as f32;
        Rectangle::new(screen_pos, (cell_sizef, cell_sizef))
    }

    pub fn reset(&mut self) -> Result<()> {
        let new_state = game_state::GameState::new(
            self.state.player.clone(),
            self.state.score,
            &self.font,
            &self.default_style,
        )?;
        self.state = new_state;
        let start = self.state.grid.start;
        self.state.player.reset_location(Vector::new(
            (start.0 * CELL_SIZE) as i32,
            (start.1 * CELL_SIZE) as i32,
        ));

        self.state.player.invis_timer = 30;
        Ok(())
    }
}
impl State for MainState {
    fn new() -> Result<Self> {
        let font = Font::from_bytes(include_bytes!("../static/font.ttf").to_vec())?;
        let style = FontStyle::new(100.0, Color::WHITE);

        let grid = Grid::new(GRID_SIZE, GRID_SIZE)?;
        let start = grid.start;
        let possible_spawns: Vec<_> = grid
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, v)| v.can_move && !v.is_start)
            .map(|(key, _)| Grid::calc_pos_from_index(key, grid.length, grid.height))
            .collect();
        let amount = possible_spawns.len() / 30;
        let mut monsters = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..amount {
            let chosen = possible_spawns.choose(&mut rng).unwrap();
            monsters.push(Monster::new(
                Vector::new((chosen.0 * CELL_SIZE) as i32, (chosen.1 * CELL_SIZE) as i32),
                &font,
                &style,
            )?);
        }

        let player = Player::new(start, &font, &style)?;
        let rendered_score = font.render("0", &style)?;
        let rendered_dead_text =
            font.render("You died, press Esc to continue\nYour score:", &style)?;

        let screen = Box::new(StartScreen::new()?);
        let rendered_pattern_reminder = font.render("Patterns", &style)?;
        Ok(Self {
            screen,
            state: game_state::GameState::new(player, 0, &font, &style)?,
            font,
            default_style: style,
            rendered_score,
            is_dead: false,
            rendered_dead_text,
            is_at_main: true,
            _rendered_pattern_reminder: rendered_pattern_reminder,
        })
    }
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        if self.is_dead {
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
        self.state.draw(window);
        Ok(())
    }
    fn update(&mut self, window: &mut Window) -> Result<()> {
        if self.is_at_main {
            let board = window.keyboard();
            if check_multiple_pressed(&board, &[Key::Escape, Key::Return]) {
                self.is_at_main = false;
            }
            return Ok(());
        }
        if self.is_dead {
            let board = window.keyboard();
            if check_multiple_pressed(&board, &[Key::Escape, Key::Return]) {
                self.reset()?;
                self.state.player =
                    Player::new(self.state.grid.start, &self.font, &self.default_style)?;
                self.is_dead = false;
                self.state.score = 0;
            }
            return Ok(());
        }
        let action = self.state.update(window, &self.font, &self.default_style)?;
        match action {
            StateAction::None => {}
            StateAction::Die => {
                self.rendered_score = self
                    .font
                    .render(&self.state.score.to_string(), &self.default_style)?;
                self.is_dead = true;
                self.state.score = 0;
            }
            StateAction::Reset => self.reset()?,
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
