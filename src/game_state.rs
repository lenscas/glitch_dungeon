use crate::bullet::Bullet;
use crate::grid::Grid;
use crate::monster::Monster;
use crate::player::Player;
use crate::{CELL_SIZE, GRID_SIZE};
use quicksilver::Result;

use crate::player::Action;
use quicksilver::geom::Vector;
use quicksilver::graphics::Font;
use quicksilver::graphics::FontStyle;
use quicksilver::lifecycle::Window;
use rand::seq::SliceRandom;

pub struct GameState {
    pub grid: Grid,
    pub player: Player,
    pub monsters: Vec<Monster>,
    pub bullets: Vec<Bullet>,
    pub score: u64,
}
impl GameState {
    fn basic_setup(font: &Font, style: &FontStyle) -> Result<(Grid, Vec<Monster>)> {
        let grid = Grid::new(GRID_SIZE, GRID_SIZE)?;
        let mut monsters = Vec::new();
        let mut rng = rand::thread_rng();
        let possible_spawns: Vec<_> = grid
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, v)| v.can_move && !v.is_start)
            .map(|(key, _)| Grid::calc_pos_from_index(key, grid.length, grid.height))
            .collect();
        let amount = possible_spawns.len() / 20;
        for _ in 0..amount {
            let chosen = possible_spawns.choose(&mut rng).unwrap();
            monsters.push(Monster::new(
                Vector::new((chosen.0 * CELL_SIZE) as i32, (chosen.1 * CELL_SIZE) as i32),
                font,
                style,
            )?);
        }
        Ok((grid, monsters))
    }

    pub fn new(font: &Font, style: &FontStyle) -> Result<Self> {
        let (grid, monsters) = Self::basic_setup(font, style)?;
        let start = grid.start;
        let mut player = Player::new(start, &font, &style)?;
        player.invis_timer = 30;
        Ok(Self {
            grid,
            player,
            monsters,
            bullets: Vec::new(),
            score: 0,
        })
    }

    pub fn reset(&mut self, font: &Font, style: &FontStyle) -> Result<()> {
        let (grid, monsters) = Self::basic_setup(font, style)?;
        self.grid = grid;
        self.monsters = monsters;

        let start = self.grid.start;
        self.player.reset_location(Vector::new(
            (start.0 * CELL_SIZE) as i32,
            (start.1 * CELL_SIZE) as i32,
        ));

        self.player.invis_timer = 30;
        self.bullets = Vec::new();
        Ok(())
    }

    pub fn draw(&self, window: &mut Window) {
        let (start, end) = self.player.get_outer_cell_points();
        let part = self.grid.get_part(start, end);
        let mut z = 0;
        part.into_iter().for_each(|(loc2, tile)| {
            tile.draw(loc2, window, z, &self.player);
            z = z + 1;
        });
        self.bullets.iter().for_each(|bullet| {
            bullet.draw(window, z, &self.player);
            z = z + 1;
        });
        self.monsters.iter().for_each(|monster| {
            monster.draw(window, z, &self.player);
            z = z + 1;
        });
        self.player.draw(window, z);
    }

    pub fn update(
        &mut self,
        window: &mut Window,
        font: &Font,
        style: &FontStyle,
    ) -> Result<StateAction> {
        let (points, action) = self.player.update(window, &mut self.grid, &font, &style)?;
        self.score += points;
        match action {
            Action::None => {}
            Action::NextScreen => return Ok(StateAction::NextLevel),
            Action::Shoot(gun) => {
                let speed = gun.speed;
                let damage = gun.damage;
                let bullets: Vec<_> = gun
                    .patterns
                    .into_iter()
                    .map(|v| {
                        Bullet::new_with_pattern(
                            self.player.location.location,
                            speed,
                            self.player.dir,
                            v,
                            damage,
                        )
                    })
                    .collect();
                self.bullets.extend(bullets);
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
        let player = &self.player;
        self.monsters
            .iter_mut()
            .for_each(|v| v.move_a_bit(&grid, player));
        let mut monsters = Vec::new();
        for mut monster in self.monsters.drain(0..self.monsters.len()) {
            for bullet in &bullets {
                if bullet.location.cell_loc == monster.location.cell_loc {
                    monster.get_damage(bullet.damage, &font, &style)?;
                }
            }
            if monster.is_alive() {
                if monster.location.cell_loc == self.player.location.cell_loc
                    && self.player.invis_timer == 0
                {
                    self.player.health -= monster.damage;
                    self.player.rendered_health =
                        font.render(&self.player.health.to_string(), &style)?;
                    self.player.invis_timer = 30;
                    if self.player.health <= 0 {
                        break;
                    }
                }
                monsters.push(monster);
            } else {
                self.score += 10;
            }
        }
        if self.player.health <= 0 {
            return Ok(StateAction::Die);
        } else {
            if self.player.invis_timer > 0 {
                self.player.invis_timer -= 1;
            }
            self.monsters = monsters;
            self.bullets = bullets;
        }

        Ok(StateAction::None)
    }
}
pub enum StateAction {
    NextLevel,
    Die,
    None,
}
