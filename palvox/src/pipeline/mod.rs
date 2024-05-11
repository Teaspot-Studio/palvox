pub mod quad; 
pub mod triangle;

use std::fmt::Display;

use luminance::pixel::NormRGB8UI;
use luminance::{context::Context, RenderSlots};
use luminance::backend::Backend;

use crate::assets::AssetLoader;
use crate::input::{InputAction, LoopFeedback};

pub trait Pipeline: Sized {
  type Error: Display; 

  fn bootstrap<A: AssetLoader>(
    frame_size: [u32; 2],
    assets: &A,
    context: &mut Context<impl Backend>,
  ) -> Result<Self, Self::Error>;

  fn render_frame(
    self,
    time: f32,
    actions: impl Iterator<Item = InputAction>,
    ctx: &mut Context<impl Backend>,
  ) -> Result<LoopFeedback<Self>, Self::Error>;
}

// Render slots.
//
// A render slot represents the channels the end stage of a shader program is going to end up writing to. In our case,
// since we are only interested in rendering the color of each pixel, we will just have one single channel for the
// color.
#[derive(Clone, Copy, Debug, PartialEq, RenderSlots)]
pub struct FragSlot {
  frag: NormRGB8UI,
}