use crate::view::Sprites;
use graphics::{Transformed, Viewport};
use opengl_graphics::GlGraphics;
use loong_ctrl::{LoongCtrlFullState, LoongPartVariant};

use crate::consts::{HALF_STEP, STEP};

pub fn draw_loong(
  gl: &mut GlGraphics,
  vp: Viewport,
  state: &LoongCtrlFullState,
  sprites: &Sprites,
  offset: (f64, f64),
) {
  gl.draw(vp, |c, gl| {
    for loong_part in &state.loong {
      let x = loong_part.point.0 as f64 * STEP - HALF_STEP + offset.0;
      let y = loong_part.point.1 as f64 * STEP - HALF_STEP + offset.1;
      let transform = c.transform.trans(x, y);

      let sprite = match loong_part.variant {
        LoongPartVariant::Head(dir) => sprites.head(dir),
        LoongPartVariant::Tail(dir) => sprites.tail(dir),
        LoongPartVariant::Body(is_vertical) => sprites.body(is_vertical),
        LoongPartVariant::Corner(var) => sprites.corner(var),
      };

      sprite.draw(transform, gl);
    }
  });
}
