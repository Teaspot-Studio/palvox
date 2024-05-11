mod assets;
mod input;
mod pipeline;

use assets::PreloadedImages;
use glfw::{Context as _, SwapInterval, WindowMode};
use input::{adapt_events, LoopFeedback};
use log;
use luminance_glfw::{GlfwSurface, GlfwSurfaceError};
use pipeline::{quad::QuadPipeline, triangle::TrianglePipeline, Pipeline};
use std::time::Instant;

#[derive(Debug)]
pub enum PlatformError {
    CannotCreateWindow,
}

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .init();

    let GlfwSurface {
        events_rx,
        mut window,
        mut ctx,
    } = GlfwSurface::new_gl33(|glfw| {
        let (mut window, events) = glfw
            .create_window(960, 540, "palvox", WindowMode::Windowed)
            .ok_or_else(|| GlfwSurfaceError::UserError(PlatformError::CannotCreateWindow))?;

        window.make_current();
        window.set_all_polling(true);
        glfw.set_swap_interval(SwapInterval::Sync(1));

        Ok((window, events))
    })
    .expect("GLFW surface creation");

    let (fb_w, fb_h) = window.get_framebuffer_size();

    let mut assets = PreloadedImages::new();
    assets
        .load_file("test", "assets/test.png")
        .expect("loaded texture");

    let mut pipeline = match QuadPipeline::bootstrap([fb_w as _, fb_h as _], &assets, &mut ctx) {
        Ok(pipeline) => pipeline,
        Err(e) => {
            log::error!("cannot bootstrap pipeline: {}", e);
            return;
        }
    };

    let start_t = Instant::now();
    'app: loop {
        // handle events
        window.glfw.poll_events();
        let actions = glfw::flush_messages(&events_rx).flat_map(|(_, event)| adapt_events(event));

        let t = start_t.elapsed().as_secs_f32();
        let feedback = pipeline.render_frame(t, actions, &mut ctx);

        match feedback {
            Ok(LoopFeedback::Continue(stepped)) => {
                pipeline = stepped;
                window.swap_buffers();
            }

            Ok(LoopFeedback::Exit) => break 'app,

            Err(e) => {
                log::error!("error while rendering a frame: {}", e);
                break 'app;
            }
        }
    }
}
