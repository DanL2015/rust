extern crate sdl2;
use std::collections::HashSet;

use sdl2::{event::Event, keyboard::Keycode};

mod world;
use world::world::World;
mod renderer;
use renderer::renderer::Renderer;
mod player;
use player::player::Player;

pub fn main() {
    let screen_area = (800, 600);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Rust Game", screen_area.0, screen_area.1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut running = true;
    let mut event_queue = sdl_context.event_pump().unwrap();

    let mut world = World::new(Player::new());
    world.gen((-100, 100), (-100, 100)); //initial world generation around the player
    world.print_debug();
    let mut render = Renderer::new(screen_area.0, screen_area.1);

    let mut keys_pressed: HashSet<Keycode> = HashSet::new();

    while running {
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit{ .. } => {
                    running = false;
                }
                Event::KeyDown { keycode, .. } => {
                    keys_pressed.insert(keycode.unwrap());
                }
                Event::KeyUp { keycode, .. } => {
                    keys_pressed.remove(&keycode.unwrap());
                }
                _ => {}
            }
        }
        //Game loop
        world.player.input(&keys_pressed, &world.world, &world.tiles, render.tile_size);
        render.render(&mut canvas, &mut world);
    }

}