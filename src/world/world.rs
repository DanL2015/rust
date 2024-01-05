use noise::{NoiseFn, Perlin};
use rand::{rngs::ThreadRng, Rng};
use sdl2::{pixels::Color, render};
use std::collections::HashMap;

use crate::{player::player::Player, renderer::renderer::Renderer};

//One world tile
pub struct Tile {
    pub name: String,
    pub id: i32,
    pub color: Color, //Color of the tile
    pub symbol: char, //Text symbol drawn on tile (decoration)
    pub solid: bool,  //If entities can walk through the tile
}

//Holds information about the world
pub struct World {
    pub world: HashMap<(i32, i32), i32>, //Stores rendered tiles (by id)
    pub tiles: HashMap<i32, Tile>,       //Stores all tiles based on id (0..n)
    pub rng: ThreadRng,
    pub player: Player,
    pub noise: Perlin,
    scale: f64,
}

fn init_tiles(tiles: &mut HashMap<i32, Tile>) {
    tiles.insert(
        -1,
        Tile {
            name: "Unrendered".to_string(),
            id: -1,
            color: Color::RGB(0, 0, 0),
            symbol: ' ',
            solid: false,
        },
    );
    tiles.insert(
        0,
        Tile {
            name: "Grass".to_string(),
            id: 0,
            color: Color::RGB(0, 255, 0),
            symbol: ',',
            solid: false,
        },
    );
    tiles.insert(
        1,
        Tile {
            name: "Dirt".to_string(),
            id: 1,
            color: Color::RGB(150, 75, 0),
            symbol: '.',
            solid: false,
        },
    );
    tiles.insert(
        2,
        Tile {
            name: "Water".to_string(),
            id: 2,
            color: Color::RGB(0, 0, 255),
            symbol: ' ',
            solid: true,
        },
    );
    tiles.insert(
        3,
        Tile {
            name: "Sand".to_string(),
            id: 3,
            color: Color::RGB(255, 255, 0),
            symbol: '.',
            solid: false,
        },
    );
    tiles.insert(
        4,
        Tile {
            name: "Snow".to_string(),
            id: 4,
            color: Color::RGB(255, 255, 255),
            symbol: ' ',
            solid: false,
        },
    );
}

impl World {
    pub fn new(player: Player) -> Self {
        let mut tiles = HashMap::new();
        init_tiles(&mut tiles);

        return Self {
            world: HashMap::new(),
            tiles: tiles,
            rng: rand::thread_rng(),
            player: player,
            noise: Perlin::new(1),
            scale: 0.05,
        };
    }

    // Creates noise from x and y coords
    fn get_noise(&self, x: i32, y: i32) -> f64 {
        let val: f64 = self
            .noise
            .get([x as f64 * self.scale, y as f64 * self.scale])
            / 2.0
            + 0.5;
        return val;
    }

    // Gets absolute world coords from relative coords (on screen) using player position
    pub fn get_abs_from_rel(&self, rel_pos: (i32, i32), renderer: &Renderer) -> (i32, i32) {

        let hx = (renderer.screen_area.w as i32 / renderer.tile_size) / 2;
        let hy = (renderer.screen_area.h as i32 / renderer.tile_size) / 2;

        let p = self.player.pos;

        // player position translated to tiles and rounded (what tile the player is currently on)
        let pt: (i32, i32) = (
            (p.0 as f64 / renderer.tile_size as f64).floor() as i32,
            (p.1 as f64 / renderer.tile_size as f64).floor() as i32,
        );

        // player relative position on the current player tile
        let prt: (f64, f64) = (
            ((pt.0 * renderer.tile_size) as f64 - p.0 as f64),
            ((pt.1 * renderer.tile_size) as f64 - p.1 as f64),
        );

        // screen offset when rendering tiles
        let so: (i32, i32) = ((prt.0.floor() as i32), (prt.1.floor() as i32));

        // relative tile position
        let rt = ((rel_pos.0 - so.0) / renderer.tile_size, (rel_pos.1 - so.1) / renderer.tile_size);

        // get absolute world coords
        return (pt.0 + rt.0 - hx, pt.1 + rt.1 - hy);
    }

    // Gets tile id from relative position
    pub fn get_tile_id_from_rel(&self, rel_pos: (i32, i32), renderer: &Renderer) -> i32 {
        let coords = self.get_abs_from_rel(rel_pos, &renderer);
        let tile_id = self.world.get(&coords).unwrap();
        let tile = self.tiles.get(&tile_id).unwrap();
        return tile.id;
    }

    // Gets tile id based off elevation (created by noise)
    fn get_tile(&self, e: f64) -> i32 {
        if e < 0.3 {
            return 2; // Water
        } else if e < 0.4 {
            return 3; // Sand
        } else if e < 0.5 {
            return 1; // Dirt
        } else if e < 0.8 {
            return 0; // Grass
        }
        return 4; // Snow
    }

    //Uses Perlin noise to generate tile and insert into map (careful, this may override already existing tiles!)
    pub fn generate_tile(&mut self, x: i32, y: i32) {
        let value =
            (self.get_noise(x, y) + 0.5 * self.get_noise(x, y) + 0.25 * self.get_noise(x, y))
                / (1.0 + 0.5 + 0.25);
        self.world.insert((x, y), self.get_tile(value));
    }

    //Generates a random tile id from the tilemap
    pub fn get_random_tile_id(&mut self) -> i32 {
        let id: i32 = self.rng.gen_range(0..(self.tiles.len() as i32 - 1));
        return id;
    }

    //Generates a uniformly random 21x21 square of tiles (from coords -10 to 10 in both directions)
    pub fn random_load_debug(&mut self, x: (i32, i32), y: (i32, i32)) {
        for i in x.0..x.1 {
            for j in y.0..y.1 {
                let id = self.get_random_tile_id();
                self.world.insert((i, j), id);
            }
        }
    }

    pub fn gen(&mut self, x: (i32, i32), y: (i32, i32)) {
        for i in x.0..x.1 {
            for j in y.0..y.1 {
                if self.world.contains_key(&(i, j)) {
                    continue;
                };
                self.generate_tile(i, j);
            }
        }
    }

    pub fn print_debug(&self) {
        println!("{}", self.world.get(&(0, 0)).unwrap().clone());
        for i in -10..10 {
            for j in -10..10 {
                if self.world.contains_key(&(j, i)) {
                    print!("{}", self.world.get(&(j, i)).unwrap().clone());
                } else {
                    print!("-");
                }
            }
            println!();
        }
    }
}
