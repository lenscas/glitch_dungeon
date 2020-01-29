use crate::grid::grid::Grid;
use crate::grid::Dir;
use crate::moveable::Moveable;
use crate::CELL_SIZE;
use crate::PLAYER_SIZE;
use quicksilver::geom::Rectangle;
use quicksilver::graphics::{Font, FontStyle, Image};
use quicksilver::lifecycle::Window;
use quicksilver::Result;
use quicksilver::{
    geom::{Shape, Vector},
    {
        input::ButtonState,
        input::Keyboard,
        prelude::{Col, Img, Key},
    },
};

use crate::gun::Gun;
use quicksilver::geom::Transform;
use quicksilver::graphics::Color;

pub fn check_multiple(board: &Keyboard, to_check: &[Key]) -> bool {
    to_check
        .iter()
        .map(|v| board[*v])
        .map(|v| v.is_down())
        .any(|v| v)
}

pub fn check_multiple_pressed(board: &Keyboard, to_check: &[Key]) -> bool {
    to_check
        .iter()
        .map(|v| board[*v])
        .map(|v| v == ButtonState::Pressed)
        .any(|v| v)
}

#[derive(Clone)]
pub struct Player {
    pub location: Moveable,
    pub speed: f32,
    pub dir: Dir,
    pub health: isize,
    pub invis_timer: usize,
    pub guns: Vec<Gun>,
    pub selected_gun: usize,
    pub shoot_timer: usize,
    pub rendered_health: Image,
}
impl Player {
    pub fn get_outer_cell_points(&self) -> ((usize, usize), (usize, usize)) {
        let height = 600;
        let width = 800;
        let mid_point = {
            let mut mid_point = self.location.location.clone();
            if mid_point.x < 0. {
                mid_point.x = 0.;
            }
            if mid_point.y < 0. {
                mid_point.y = 0.;
            }
            mid_point
        };
        let start_x = crate::calc_start(mid_point.x / CELL_SIZE as f32, 800 / CELL_SIZE);
        let start_y = crate::calc_start(mid_point.y / CELL_SIZE as f32, 600 / CELL_SIZE);
        let end_x = 1 + start_x + width;
        let end_y = 1 + start_y + height;
        ((start_x, start_y), (end_x, end_y))
    }
    pub fn new(loc: (usize, usize), font: &Font, style: &FontStyle) -> Result<Self> {
        let loc = Vector::new((loc.0 * CELL_SIZE) as i32, (loc.1 * CELL_SIZE) as i32);
        let guns = vec![Gun::new_random(font, style)?, Gun::new_random(font, style)?];
        let health = 100;
        let rendered_health = font.render(&health.to_string(), &style)?;
        Ok(Self {
            rendered_health,
            location: Moveable::new(loc),
            speed: 10.,
            dir: Dir::Up,
            health,
            invis_timer: 30,
            guns,
            selected_gun: 0,
            shoot_timer: 0,
        })
    }
    pub fn reset_location(&mut self, location: Vector) {
        self.location.reset_location(location);
    }
    pub fn update(
        &mut self,
        window: &Window,
        grid: &mut Grid,
        font: &Font,
        style: &FontStyle,
    ) -> Result<(u64, Action)> {
        let board = window.keyboard();
        if check_multiple(board, &[Key::A]) {
            self.location
                .move_some(Dir::Left, self.speed, grid, PLAYER_SIZE);
        }
        if check_multiple(board, &[Key::D]) {
            self.location
                .move_some(Dir::Right, self.speed, grid, PLAYER_SIZE);
        }
        if check_multiple(board, &[Key::W]) {
            self.location
                .move_some(Dir::Up, self.speed, grid, PLAYER_SIZE);
        }
        if check_multiple(board, &[Key::S]) {
            self.location
                .move_some(Dir::Down, self.speed, grid, PLAYER_SIZE);
        }
        if check_multiple(board, &[Key::Up]) {
            self.dir = Dir::Up
        }
        if check_multiple(board, &[Key::Down]) {
            self.dir = Dir::Down
        }
        if check_multiple(board, &[Key::Left]) {
            self.dir = Dir::Left
        }
        if check_multiple(board, &[Key::Right]) {
            self.dir = Dir::Right
        }
        if check_multiple_pressed(board, &[Key::Q]) {
            if self.selected_gun == 0 {
                self.selected_gun = self.guns.len() - 1;
            } else {
                self.selected_gun -= 1;
            }
        }
        if check_multiple_pressed(board, &[Key::E]) {
            if self.selected_gun == self.guns.len() - 1 {
                self.selected_gun = 0;
            } else {
                self.selected_gun += 1;
            }
        }
        let mut extra_points = 0;
        let current = grid.get_cell(self.location.cell_loc);
        if self.shoot_timer > 0 {
            self.shoot_timer -= 1;
        }
        if let Some(current) = &current {
            if current.1.has_gun {
                if let Some(gun) = grid.get_gun(&self.location.cell_loc, font, style)? {
                    self.guns.push(gun);
                    extra_points += 20;
                    if self.guns.len() > 4 {
                        self.guns.remove(0);
                    }
                }
            }
        }
        Ok((
            extra_points,
            current
                .and_then(|(_, tile)| {
                    if tile.is_end {
                        Some(Action::NextScreen)
                    } else {
                        None
                    }
                })
                .or_else(|| {
                    if check_multiple(board, &[Key::F, Key::Space]) && self.shoot_timer == 0 {
                        let selected_gun = self.guns[self.selected_gun].clone();
                        self.shoot_timer = selected_gun.cooldown;
                        return Some(Action::Shoot(selected_gun));
                    } else {
                        None
                    }
                })
                .unwrap_or(Action::None),
        ))
    }
    pub fn draw(&self, window: &mut Window, z: i32) {
        let mut player_rec = self.get_rectangle();
        window.draw_ex(
            &player_rec,
            Col(if self.invis_timer == 0 {
                Color::WHITE
            } else {
                Color::ORANGE
            }),
            Transform::IDENTITY,
            z,
        );
        player_rec.pos.y += 20.;
        player_rec.size.y = 15.;
        player_rec.size.x = 20.;
        window.draw_ex(
            &player_rec,
            Img(&self.rendered_health),
            Transform::IDENTITY,
            z,
        );
    }
    pub fn get_rectangle(&self) -> Rectangle {
        let player_on_screen = self.grid_to_screen(&(
            self.location.location.x / CELL_SIZE as f32,
            self.location.location.y / CELL_SIZE as f32,
        ));
        Rectangle::new(player_on_screen, (PLAYER_SIZE as i32, PLAYER_SIZE as i32))
            .with_center(player_on_screen)
    }
    pub fn grid_to_screen(&self, loc: &(f32, f32)) -> (f32, f32) {
        let cell_size = CELL_SIZE as f32;
        let width = 800. / cell_size;
        let height = 600. / cell_size;

        let x =
            (loc.0 as f32 - (self.location.location.x / cell_size - width / 2.)) * cell_size as f32;
        let y = (loc.1 as f32 - (self.location.location.y / cell_size - height / 2.))
            * cell_size as f32;
        (x, y)
    }
}

#[derive(Clone, Debug)]
pub enum Action {
    Shoot(Gun),
    NextScreen,
    None,
}
