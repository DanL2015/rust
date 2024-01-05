use sdl2::{render::{Canvas, self}, video::Window, pixels::Color, ttf::Font};

use crate::renderer::renderer::{Align, Renderer};

pub struct Gui_Window{
    pub pos: (i32, i32),
    pub text: String,
    pub align: Align,
}

impl Gui_Window {
    pub fn new(pos: (i32, i32), text: String, align: Align) -> Self {
        Self {
            pos: pos,
            text: text,
            align: align,
        }
    }

    pub fn draw(&self, renderer: &Renderer, canvas: &mut Canvas<Window>, font: &Font) {
        renderer.draw_text(canvas, font, Color::BLACK, self.text.clone(), self.pos, self.align);
    }
}

// 0: Mouse hover tile information
pub struct Gui {
    pub draw_stack: Vec<Gui_Window>,
}

impl Gui {
    pub fn new() -> Self {
        Self {
            draw_stack: Vec::new(),
        }
    }

    pub fn draw_windows(&self, renderer: &Renderer, canvas: &mut Canvas<Window>, font: &Font) {
        for window in self.draw_stack.as_slice() {
            window.draw(renderer, canvas, font);
        }
    }
}