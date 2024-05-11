use luminance::{
    backend::Backend,
    context::Context,
    dim::{Dim2, Size2},
    framebuffer::{Back, Framebuffer},
    pipeline::PipelineState,
    pixel::{NormRGB8UI, NormUnsigned},
    primitive::Triangle,
    render_state::RenderState,
    shader::{Program, ProgramBuilder, Uni},
    texture::{InUseTexture, MagFilter, MinFilter, Mipmaps, Texture, TextureSampling},
    vertex_entity::{VertexEntity, VertexEntityBuilder, View},
    Uniforms,
};

use crate::{assets::AssetLoader, input::{InputAction, LoopFeedback}};

use super::{FragSlot, Pipeline};

const VS: &'static str = include_str!("../shaders/screenquad.vs");
const FS: &'static str = include_str!("../shaders/screenquad.fs");

#[derive(Uniforms)]
struct ShaderInterface {
    image: Uni<InUseTexture<Dim2, NormUnsigned>>,
}

pub struct QuadPipeline {
    img_tex: Texture<Dim2, NormRGB8UI>,
    program: Program<(), (), Triangle, FragSlot, ()>,
    attributeless: VertexEntity<(), Triangle, ()>,
    back_buffer: Framebuffer<Dim2, Back<FragSlot>, Back<()>>,
}

impl Pipeline for QuadPipeline {
    type Error = luminance::backend::Error;

    fn bootstrap<A: AssetLoader>(
        [width, height]: [u32; 2],
        assets: &A,
        ctx: &mut Context<impl Backend>,
    ) -> Result<Self, Self::Error> {
        let (img_size, img) = load_img(assets, "test").expect("test texture is loaded");
        let img_tex = ctx.new_texture(
            img_size,
            Mipmaps::No,
            &TextureSampling {
                min_filter: MinFilter::Nearest,
                mag_filter: MagFilter::Nearest,
                ..TextureSampling::default()
            },
            &img[..],
        )?;

        // we don’t use a Vertex type anymore (i.e. attributeless, so we use the unit () type)
        let program = ctx.new_program(
            ProgramBuilder::new()
                .add_vertex_stage(VS)
                .no_primitive_stage()
                .add_shading_stage(FS),
        )?;

        // yet, we still need to tell luminance to render a certain number of vertices (even if we send no
        // attributes / data); in our case, we’ll just render a triangle, which has three vertices
        let attributeless = ctx.new_vertex_entity(VertexEntityBuilder::new())?;

        let back_buffer = ctx.back_buffer(Size2::new(width, height))?;

        Ok(Self {
            img_tex,
            program,
            attributeless,
            back_buffer,
        })
    }

    fn render_frame(
        mut self,
        _time: f32,
        actions: impl Iterator<Item = InputAction>,
        ctx: &mut Context<impl Backend>,
    ) -> Result<LoopFeedback<Self>, Self::Error> {
        for action in actions {
            match action {
                InputAction::Quit => return Ok(LoopFeedback::Exit),
                _ => (),
            }
        }

        let program = &mut self.program;
        let attributeles = &self.attributeless;

        ctx.with_framebuffer(&self.back_buffer, &PipelineState::default(), |mut frame| {
            frame.with_program(program, |mut frame| {
                frame.with_render_state(&RenderState::default(), |mut frame| {
                    frame.render_vertex_entity(attributeles.view(..6))
                })
            })
        })?;

        Ok(LoopFeedback::Continue(self))
    }
}

pub fn load_img<A: AssetLoader>(assets: &A, name: &str) -> Option<(Size2, Vec<u8>)> {
  let img = assets
    .get_texture(name)
    .map_err(|e| log::error!("error while loading image: {}", e))
    .ok()?;
  let (width, height) = img.dimensions();
  let texels = img.clone().into_raw();
  log::info!("loaded texture with width={} height={}", width, height);

  Some((Size2::new(width, height), texels))
}