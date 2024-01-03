use std::collections::HashMap;

use rand::{Rng, rngs::ThreadRng};
use sdl2::pixels::Color;

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
}

impl World {
    pub fn new() -> Self {
        let mut tiles = HashMap::new();
        init_tiles(&mut tiles);

        return Self {
            world: HashMap::new(),
            tiles: tiles,
            rng: rand::thread_rng(),
        };
    }

    //Generates a random tile id from the tilemap
    pub fn random_tile_id(&mut self) -> i32 {
        let id: i32 = self.rng.gen_range(0..(self.tiles.len() as i32 - 1));
        return id;
    }

    //Generates a uniformly random 21x21 square of tiles (from coords -10 to 10 in both directions)
    pub fn random_load_debug(&mut self) {
        for i in -5..5 {
            for j in -5..5 {
                let id = self.random_tile_id();
                self.world.insert((i, j), id);
            }
        }
    }

    pub fn print_debug(&self) {
        println!("{}", self.world.get(&(0, 0)).unwrap().clone());
        for i in -10..10 {
            for j in -10..10 {
                if self.world.contains_key(&(j, i)) {
                    print!("{}", self.world.get(&(j, i)).unwrap().clone());
                }
                else {
                    print!("-");
                }
            }
            println!();
        }
    }
}
