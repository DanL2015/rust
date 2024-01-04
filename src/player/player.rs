use sdl2::{keyboard::Keycode, pixels::Color, render::Canvas, video::Window};
use std::collections::{HashMap, HashSet};

use crate::world::world::Tile;

pub struct Player {
    pub pos: (f64, f64),
    pub color: Color,
    pub size: (u32, u32),
    mv_mult: f64,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: (25.0, 25.0),
            color: Color::BLACK,
            size: (20, 20),
            mv_mult: 0.05,
        }
    }

    fn collision(
        &self,
        world: &HashMap<(i32, i32), i32>,
        tiles: &HashMap<i32, Tile>,
        tile_size: i32,
    ) -> bool {
        // player position translated to tiles and rounded (what tile the player is currently on)
        let pt: (i32, i32) = (
            (self.pos.0 as f64 / tile_size as f64).floor() as i32,
            (self.pos.1 as f64 / tile_size as f64).floor() as i32,
        );

        let dir = vec![(0, 1), (0, -1), (1, 0), (-1, 0), (0, 0)];

        // let ptid: i32;
        // if world.contains_key(&pt) {
        //     ptid = world.get(&pt).unwrap().clone();
        // } else {
        //     ptid = -1;
        // }
        // let ptile: &Tile = tiles.get(&ptid).unwrap();
        // println!("{}", ptile.name);

        for d in dir {
            let n: (i32, i32) = (pt.0 + d.0, pt.1 + d.1);
            let tid: i32;
            if world.contains_key(&n) {
                tid = world.get(&n).unwrap().clone();
            } else {
                tid = -1;
            }
            let t: &Tile = tiles.get(&tid).unwrap();

            if !t.solid {
                continue;
            }

            //check for collision
            // Player: self.pos +/- self.size / 2
            // Tile: n +/0 tile_size
            let p_left: f64 = self.pos.0 - (self.size.0 as f64 / 2.0);
            let p_right: f64 = self.pos.0 + (self.size.0 as f64 / 2.0);
            let p_up: f64 = self.pos.1 - (self.size.1 as f64 / 2.0);
            let p_down: f64 = self.pos.1 + (self.size.1 as f64 / 2.0);

            let t_left: f64 = (n.0 * tile_size) as f64;
            let t_right: f64 = ((n.0 + 1) * tile_size) as f64;
            let t_up: f64 = (n.1 * tile_size) as f64;
            let t_down: f64 = ((n.1 + 1) * tile_size) as f64;

            if (p_down <= t_up || p_up >= t_down || p_right <= t_left || p_left >= t_right) {
                continue;
            }
            return true;
        }
        return false;
    }

    // movement function, takes in tuple for delta
    fn mv(
        &mut self,
        delta: (f64, f64),
        world: &HashMap<(i32, i32), i32>,
        tiles: &HashMap<i32, Tile>,
        tile_size: i32,
    ) {
        self.pos.0 += delta.0 * self.mv_mult;
        self.pos.1 += delta.1 * self.mv_mult;
        if self.collision(world, tiles, tile_size) {
            self.pos.0 -= delta.0 * self.mv_mult;
            self.pos.1 -= delta.1 * self.mv_mult;
        }
    }

    // takes in input
    pub fn input(
        &mut self,
        keys_pressed: &HashSet<Keycode>,
        world: &HashMap<(i32, i32), i32>,
        tiles: &HashMap<i32, Tile>,
        tile_size: i32,
    ) {
        if keys_pressed.get(&Keycode::W).is_some() {
            self.mv((0.0, -1.0), world, tiles, tile_size);
        }
        if keys_pressed.get(&Keycode::S).is_some() {
            self.mv((0.0, 1.0), world, tiles, tile_size);
        }
        if keys_pressed.get(&Keycode::A).is_some() {
            self.mv((-1.0, 0.0), world, tiles, tile_size);
        }
        if keys_pressed.get(&Keycode::D).is_some() {
            self.mv((1.0, 0.0), world, tiles, tile_size);
        }
    }
}
