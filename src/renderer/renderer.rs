use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::world::world::{World, Tile};

pub struct Renderer {
    pub screen_area: Rect,  //Rect that stores screen height and width
    pub clear_color: Color, //Color on clear (set to black anyway)
    pub tile_size: i32,     //Size of tile rendering on screen
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

    // Note: Idk if this works right or not, did the calculations in my head
    pub fn render(&self, canvas: &mut Canvas<Window>, world: &mut World) {
        self.clear(canvas);

        // For now, assume the world is centered at 0, 0 (later will be centered on player)
        let sw = self.screen_area.width();
        let sh = self.screen_area.height();

        // Todo: change this when implementing player struct
        // Absolute position of player on the map
        let p: (f64, f64) = (-200.0, -200.0);

        // player position translated to tiles and rounded (what tile the player is currently on)
        let pt: (i32, i32) = (
            (p.0 as f64 / self.tile_size as f64).ceil() as i32,
            (p.1 as f64 / self.tile_size as f64).ceil() as i32,
        );

        // player relative position on the current player tile
        let prt: (f64, f64) = (
            ((pt.0 * self.tile_size) as f64 - p.0 as f64),
            ((pt.1 * self.tile_size) as f64 - p.1 as f64),
        );

        // screen offset when rendering tiles
        let so: (i32, i32) = (
            (prt.0.floor() as i32),
            (prt.1.floor() as i32),
        );

        let hx = (sw as i32 / self.tile_size) / 2;
        let hy = (sh as i32 / self.tile_size) / 2;

        let x = (pt.0 - hx - 1, pt.0 + hx + 1); // tuple representing range of rendering in x direction
        let y = (pt.1 - hy - 1, pt.1 + hy + 1); // tuple representing range of rendering in y direction

        for i in y.0..y.1 {
            for j in x.0..x.1 {
                let tid: i32;
                if world.world.contains_key(&(j, i)) {
                    tid = world.world.get(&(j, i)).unwrap().clone();
                } else {
                    tid = -1;
                }
                let t: &Tile = world.tiles.get(&tid).unwrap();
                let t_rect: Rect = Rect::new(so.0 + (j - x.0 - 1) * self.tile_size as i32, so.1 + (i - y.0 - 1) * self.tile_size as i32, self.tile_size as u32, self.tile_size as u32);
                canvas.set_draw_color(t.color);
                let _ = canvas.fill_rect(t_rect);
            }
        }

        //temporary draw player at 0, 0
        let p_rect: Rect = Rect::new((sw / 2 - 25) as i32, (sh / 2 - 25) as i32, 50, 50);
        canvas.set_draw_color(Color::BLACK);
        let _ = canvas.fill_rect(p_rect);

        canvas.present();
    }
}
