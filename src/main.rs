use crate::bullet::Bullet;
use crate::grid::grid::Grid;
use crate::grid::tile::Tile;
use crate::monster::Monster;
use crate::player::Action;
use crate::player::Player;
use quicksilver::{
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Background::Col, Color},
    lifecycle::{run, Event, Settings, State, Window},
    Result,
};
use rand::seq::SliceRandom;

const CELL_SIZE: usize = 32;
const PLAYER_SIZE: usize = 16;
const GRID_SIZE: usize = 30;
mod bullet;
mod grid;
mod monster;
mod moveable;
mod player;

pub struct MainState {
    grid: Grid,
    player: Player,
    drawn_grid: Vec<(Rectangle, Tile)>,
    bullets: Vec<Bullet>,
    monsters: Vec<Monster>,
}
impl MainState {
    fn calc_start(cam: f32, line_size: usize) -> usize {
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

    pub fn pos_to_full_square_on_grid(&mut self, loc: &(f32, f32)) -> Rectangle {
        let screen_pos = self.player.grid_to_screen(loc);
        let cell_sizef = CELL_SIZE as f32;
        Rectangle::new(screen_pos, (cell_sizef, cell_sizef))
    }
    pub fn get_outer_cell_points(&self) -> ((usize, usize), (usize, usize)) {
        let height = 600;
        let width = 800;
        let mid_point = {
            let mut mid_point = self.player.location.location.clone();
            if mid_point.x < 0. {
                mid_point.x = 0.;
            }
            if mid_point.y < 0. {
                mid_point.y = 0.;
            }
            mid_point
        };
        let start_x = Self::calc_start(mid_point.x / CELL_SIZE as f32, 800 / CELL_SIZE);
        let start_y = Self::calc_start(mid_point.y / CELL_SIZE as f32, 600 / CELL_SIZE);
        let end_x = 1 + start_x + width;
        let end_y = 1 + start_y + height;
        ((start_x, start_y), (end_x, end_y))
    }
    pub fn reset(&mut self) {
        self.grid = Grid::new(GRID_SIZE, GRID_SIZE);
        self.bullets = Vec::new();
        let start = self.grid.start;
        self.player.reset_location(Vector::new(
            (start.0 * CELL_SIZE) as i32,
            (start.1 * CELL_SIZE) as i32,
        ));
        let possible_spawns: Vec<_> = self
            .grid
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, v)| v.can_move && !v.is_start)
            .map(|(key, _)| Grid::calc_pos_from_index(key, self.grid.length, self.grid.height))
            .collect();
        let amount = possible_spawns.len() / 20;
        let mut monsters = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..amount {
            let chosen = possible_spawns.choose(&mut rng).unwrap();
            monsters.push(Monster::new(Vector::new(
                (chosen.0 * CELL_SIZE) as i32,
                (chosen.1 * CELL_SIZE) as i32,
            )));
        }
        self.monsters = monsters;
    }
}
impl State for MainState {
    fn new() -> Result<Self> {
        let grid = Grid::new(GRID_SIZE, GRID_SIZE);
        let start = grid.start;
        let loc = Vector::new((start.0 * CELL_SIZE) as i32, (start.1 * CELL_SIZE) as i32);
        let possible_spawns: Vec<_> = grid
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, v)| v.can_move && !v.is_start)
            .map(|(key, _)| Grid::calc_pos_from_index(key, grid.length, grid.height))
            .collect();
        let amount = possible_spawns.len() / 20;
        let mut monsters = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..amount {
            let chosen = possible_spawns.choose(&mut rng).unwrap();
            monsters.push(Monster::new(Vector::new(
                (chosen.0 * CELL_SIZE) as i32,
                (chosen.1 * CELL_SIZE) as i32,
            )));
        }
        Ok(Self {
            grid,
            player: Player::new(loc),
            drawn_grid: Vec::new(),
            bullets: Vec::new(),
            monsters: monsters,
        })
    }
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        let (start, end) = self.get_outer_cell_points();
        let part = self.grid.get_part(start, end);
        let mut z = 0;
        self.drawn_grid = part
            .into_iter()
            .enumerate()
            .map(|(_, tile)| {
                let (loc2, tile) = tile;
                let loc = self.player.grid_to_screen(&(loc2.0 as f32, loc2.1 as f32));
                let to_draw = if tile.can_move {
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
                let rec = Rectangle::new(loc, (32, 32));
                window.draw_ex(&rec, Col(to_draw), Transform::IDENTITY, z);
                z = z + 1;
                (rec, tile)
            })
            .collect();
        self.bullets.iter().for_each(|bullet| {
            let screen_pos = self.player.grid_to_screen(&(
                bullet.location.location.x / CELL_SIZE as f32,
                bullet.location.location.y / CELL_SIZE as f32,
            ));
            window.draw_ex(
                &Rectangle::new(screen_pos.clone(), (bullet.size as f32, bullet.size as f32))
                    .with_center(screen_pos),
                Col(Color::MAGENTA),
                Transform::IDENTITY,
                z,
            );
            z = z + 1;
        });
        self.monsters.iter().for_each(|monster| {
            let screen_pos = self.player.grid_to_screen(&(
                monster.location.location.x / CELL_SIZE as f32,
                monster.location.location.y / CELL_SIZE as f32,
            ));
            window.draw_ex(
                &Rectangle::new(
                    screen_pos.clone(),
                    (monster.size as f32, monster.size as f32),
                )
                .with_center(screen_pos),
                Col(Color::INDIGO),
                Transform::IDENTITY,
                z,
            );
            z = z + 1;
        });
        window.draw_ex(
            &self.player.get_rectangle(),
            Col(Color::WHITE),
            Transform::IDENTITY,
            z,
        );
        Ok(())
    }
    fn update(&mut self, window: &mut Window) -> Result<()> {
        let action = self.player.update(window, &self.grid);
        match action {
            Action::None => {}
            Action::NextScreen => self.reset(),
            Action::Shoot => {
                let bullet = Bullet::new(self.player.location.location, 15., self.player.dir);
                self.bullets.push(bullet)
            }
        }
        let mut bullets = Vec::new();
        for mut bullet in self.bullets.drain(0..self.bullets.len()) {
            if !bullet.update(&self.grid) {
                bullets.push(bullet)
            }
        }
        let bullets = bullets;
        let grid = &self.grid;
        self.monsters.iter_mut().for_each(|v| v.move_a_bit(&grid));
        let mut monsters = Vec::new();
        for mut monster in self.monsters.drain(0..self.monsters.len()) {
            for bullet in &bullets {
                if bullet.location.cell_loc == monster.location.cell_loc {
                    monster.health -= 1;
                }
            }
            if monster.health > 0 {
                if monster.location.cell_loc == self.player.location.cell_loc {
                    self.player.health -= monster.damage;
                    if self.player.health <= 0 {
                        break;
                    }
                }
                monsters.push(monster);
            }
        }
        if self.player.health <= 0 {
            self.reset();
            let loc = self.player.location.clone();
            self.player = Player::new(Vector::new(0, 0));
            self.player.location = loc;
        } else {
            self.monsters = monsters;
            self.bullets = bullets;
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
