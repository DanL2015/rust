use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::world::world::{World, Tile};

pub struct Renderer {
    pub screen_area: Rect,  //Rect that stores screen height and width
    pub clear_color: Color, //Color on clear (set to black anyway)
    pub tile_size: u32,     //Size of tile rendering on screen
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            screen_area: Rect::new(0, 0, width, height),
            clear_color: Color::RGB(0, 0, 0),
            tile_size: 50,
        }
    }

    // Clears the canvas
    pub fn clear(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.clear_color);
        let _ = canvas.fill_rect(self.screen_area);
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, world: &mut World) {
        self.clear(canvas);

        // For now, assume the world is centered at 0, 0 (later will be centered on player)
        let sw = self.screen_area.width();
        let sh = self.screen_area.height();

        // Todo: change this when implementing player struct
        let p: (f64, f64) = (0.0, 0.0);
        // player position translated to tiles and rounded (what tile the player is currently on)
        let pt: (i32, i32) = (
            (p.0 * self.tile_size as f64).floor() as i32,
            (p.1 * self.tile_size as f64).floor() as i32,
        );
        // player relative position on the current player tile
        let prt: (f64, f64) = (
            (p.0 * self.tile_size as f64 - pt.0 as f64),
            (p.1 * self.tile_size as f64 - pt.1 as f64),
        );

        // screen offset when rendering tiles
        let so: (i32, i32) = (
            ((prt.0 * sw as f64).floor() as i32),
            ((prt.1 * sh as f64).floor() as i32),
        );

        let hx = ((sw / self.tile_size) / 2) as i32;
        let hy = ((sh / self.tile_size) / 2) as i32;

        let x = (pt.0 - hx - 1, pt.0 + hx + 2); // tuple representing range of rendering in x direction
        let y = (pt.1 - hy - 1, pt.1 + hy + 2); // tuple representing range of rendering in y direction

        // println!("{}, {}", x.0, x.1);
        // println!("{}, {}", y.0, y.1);
        // println!("{}, {}", so.0, so.1);

        for i in y.0..y.1 {
            for j in x.0..x.1 {
                let tid: i32;
                if world.world.contains_key(&(j, i)) {
                    tid = world.world.get(&(j, i)).unwrap().clone();
                } else {
                    tid = -1;
                }
                let t: &Tile = world.tiles.get(&tid).unwrap();
                // println!("x: {}, y: {}", so.0 + (j - x.0) * self.tile_size as i32, so.1 + (i - y.0) * self.tile_size as i32);
                let t_rect: Rect = Rect::new(so.0 + (j - x.0) * self.tile_size as i32, so.1 + (i - y.0) * self.tile_size as i32, self.tile_size, self.tile_size);
                canvas.set_draw_color(t.color);
                let _ = canvas.fill_rect(t_rect);
            }
        }

        canvas.present();
    }
}
