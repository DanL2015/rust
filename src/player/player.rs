use std::collections::{HashMap, HashSet};

use sdl2::{pixels::Color, keyboard::{Keycode, KeyboardState, Scancode}, rect::Rect, render::Canvas, video::Window};

use crate::world::world::World;

pub struct Player {
    pub pos: (f64, f64),
    pub color: Color,
    pub size: (u32, u32),
    mv_mult: f64,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: (0.0, 0.0),
            color: Color::BLACK,
            size: (20, 20),
            mv_mult: 0.1,
        }
    }

    // movement function, takes in tuple for delta
    fn mv(&mut self, delta: (f64, f64)) {
        self.pos.0 += delta.0 * self.mv_mult;
        self.pos.1 += delta.1 * self.mv_mult;
    }

    // takes in input 
    pub fn input(&mut self, keys_pressed: &HashSet<Keycode>) {
        if keys_pressed.get(&Keycode::W).is_some() {
            self.mv((0.0, -1.0));
        }
        if keys_pressed.get(&Keycode::S).is_some() {
            self.mv((0.0, 1.0));
        }
        if keys_pressed.get(&Keycode::A).is_some() {
            self.mv((-1.0, 0.0));
        }
        if keys_pressed.get(&Keycode::D).is_some() {
            self.mv((1.0, 0.0));
        }

    }
}