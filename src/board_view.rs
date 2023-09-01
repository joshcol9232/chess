use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::input::{RenderArgs};
use sprite::Sprite;

use std::collections::HashMap;

use crate::{consts, piece_kind::PieceKind};

fn load_piece_sprite_map(txt: Texture) -> HashMap<PieceKind, Sprite<Texture>> {
    let mut map = HashMap::new();
    map.insert(PieceKind::Bishop, Sprite::from_texture_rect(txt.into(), [0.0, 0.0, 12.0, 15.0]));
    map
}

pub struct BoardView {
    sprite_map: HashMap<PieceKind, Sprite<Texture>>,
}

impl BoardView {
    pub fn new() -> Result<Self, String> {
        let piece_sprites = Texture::from_path("./assets/pieces.png", &TextureSettings::new())?;
        let sprite_map = load_piece_sprite_map(piece_sprites);

        Ok(Self {
            sprite_map,
        })
    }

    fn render_board(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(consts::color::BLACK, gl);
        });

        let square_size = args.window_size[0] as f64 / 8.0;
        let square = rectangle::square(0.0, 0.0, square_size);

        // Render board background
        for x_idx in 0..8 {
            for y_idx in 0..8 {
                let (x, y) = (square_size * x_idx as f64, square_size * y_idx as f64);
                let mul: bool = ((x_idx + y_idx) % 2) != 1;
                let color = [0.2 + mul as u8 as f32 * 0.8, 0.2 + mul as u8 as f32 * 0.6, 0.2, 1.0];

                gl.draw(args.viewport(), |c, gl| {
                    let transform = c
                        .transform
                        .trans(x, y);
                    rectangle(color, square, transform, gl);
                });
            }
        }
    }

    pub fn render_pieces(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        // TODO
    }

    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        self.render_board(gl, args);
        self.render_pieces(gl, args);
    }
}

