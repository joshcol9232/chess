use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings, Filter};
use piston::input::{RenderArgs};
use sprite::Sprite;

use std::collections::HashMap;
use std::rc::Rc;

use crate::consts::{self, WHITE, BLACK};
use crate::piece_kind::{PieceKind, PieceDescriptor};
use crate::board_model::BoardModel;
use crate::aux_model::{self, AuxModel};


pub struct BoardView {
    sprite_map: HashMap<PieceDescriptor, Sprite<Texture>>,
}

impl BoardView {
    pub fn new() -> Result<Self, String> {
        let mut txsettings = TextureSettings::new();
        txsettings.set_filter(Filter::Nearest);
                             
        let piece_sprites = Texture::from_path("./assets/2.png", &txsettings)?;
        let sprite_map = load_piece_sprite_map(piece_sprites);

        Ok(Self {
            sprite_map,
        })
    }

    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs, model: &BoardModel, aux_model: &AuxModel) {
        use graphics::*;

        gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(consts::color::BLACK, gl);
        });

        let square_size = args.window_size[0] as f64 / 8.0;
        let square = rectangle::square(0.0, 0.0, square_size);

        const WHITE_SQUARE_COL: [f32; 4] = [1.0, 0.8, 0.5, 1.0];
        const BLACK_SQUARE_COL: [f32; 4] = [0.4, 0.1, 0.1, 1.0];

        for x_idx in 0..8 {
            for y_idx in 0..8 {
                let (x, y) = (square_size * x_idx as f64, square_size * y_idx as f64);
                let is_white_square = ((x_idx + y_idx) % 2) != 1;
                let square_state = aux_model.get_state([x_idx, y_idx]);

                let back_col = if let Some(state) = square_state {
                    match state {
                        aux_model::SquareState::ValidMove   => [0.0, 0.7, 0.0, 1.0],
                        aux_model::SquareState::InvalidMove => [0.7, 0.0, 0.0, 1.0],
                        aux_model::SquareState::IsSelected  => [0.0, 0.7, 0.7, 1.0],
                    }
                } else if is_white_square {
                    WHITE_SQUARE_COL
                } else {
                    BLACK_SQUARE_COL
                };

                // Render background
                gl.draw(args.viewport(), |c, gl| {
                    let transform = c
                        .transform
                        .trans(x, y);
                    rectangle(back_col, square, transform, gl);
                });

                // Render piece
                let piece_kind = model.kind_at([x_idx, y_idx]);
                if piece_kind != PieceKind::Empty {
                    let piece_sprite = &self.sprite_map[ &model.descriptor_at([x_idx, y_idx]) ];
                    gl.draw(args.viewport(), |c, gl| {
                        let transform = c
                            .transform
                            .trans(x + square_size/2.0, y + square_size/2.0);
                        piece_sprite.draw(transform, gl);
                    });
                }
            }
        }
    }
}

// --------------------------

fn load_piece_sprite_map(txt: Texture) -> HashMap<PieceDescriptor, Sprite<Texture>> {
    let mut map = HashMap::new();
    // [x, y, width, height]
    
    let txt_rc = Rc::new(txt);

    // In order of the image
    // -- Bishop
    map.insert(PieceDescriptor { kind: PieceKind::Bishop, team: WHITE },
               Sprite::from_texture_rect(txt_rc.clone(), [1.0, 1.0, 11.0, 15.0]));
    map.insert(PieceDescriptor { kind: PieceKind::Bishop, team: BLACK },
               Sprite::from_texture_rect(txt_rc.clone(), [1.0, 20.0, 11.0, 15.0]));
    // -- Horse
    map.insert(PieceDescriptor { kind: PieceKind::Horse, team: WHITE },
               Sprite::from_texture_rect(txt_rc.clone(), [12.0, 1.0, 12.0, 15.0]));
    map.insert(PieceDescriptor { kind: PieceKind::Horse, team: BLACK },
               Sprite::from_texture_rect(txt_rc.clone(), [12.0, 20.0, 12.0, 15.0]));
    // -- Rook
    map.insert(PieceDescriptor { kind: PieceKind::Rook, team: WHITE },
               Sprite::from_texture_rect(txt_rc.clone(), [26.0, 1.0, 9.0, 15.0]));
    map.insert(PieceDescriptor { kind: PieceKind::Rook, team: BLACK },
               Sprite::from_texture_rect(txt_rc.clone(), [26.0, 20.0, 9.0, 15.0]));
    // -- Pawn
    map.insert(PieceDescriptor { kind: PieceKind::Pawn, team: WHITE },
               Sprite::from_texture_rect(txt_rc.clone(), [37.0, 1.0, 9.0, 15.0]));
    map.insert(PieceDescriptor { kind: PieceKind::Pawn, team: BLACK },
               Sprite::from_texture_rect(txt_rc.clone(), [37.0, 20.0, 9.0, 15.0]));
    // -- King
    map.insert(PieceDescriptor { kind: PieceKind::King, team: WHITE },
               Sprite::from_texture_rect(txt_rc.clone(), [48.0, 1.0, 13.0, 15.0]));
    map.insert(PieceDescriptor { kind: PieceKind::King, team: BLACK },
               Sprite::from_texture_rect(txt_rc.clone(), [48.0, 20.0, 13.0, 15.0]));
    // -- Queen
    map.insert(PieceDescriptor { kind: PieceKind::Queen, team: WHITE },
               Sprite::from_texture_rect(txt_rc.clone(), [63.0, 1.0, 13.0, 15.0]));
    map.insert(PieceDescriptor { kind: PieceKind::Queen, team: BLACK },
               Sprite::from_texture_rect(txt_rc.clone(), [63.0, 20.0, 13.0, 15.0]));

    // Scale
    for (_key, sprite) in map.iter_mut() {
        sprite.set_scale(5.0, 5.0);
    }

    map
}

