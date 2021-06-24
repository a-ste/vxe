use luminance_windowing::{WindowDim, WindowOpt};
use luminance_glfw::GlfwSurface;
use glfw::{WindowEvent, Key, Action, Context, SwapInterval};
use std::time::Instant;
use luminance::context::GraphicsContext;
use luminance::pipeline::PipelineState;

pub mod builder;


#[allow(dead_code)]
pub struct Instance {
    surface: GlfwSurface,
    fps: u32,
}

impl Instance {
    pub(crate) fn init(title: String, resolution: [u32; 2], vsync: bool) -> Instance {
        let dim = WindowDim::Windowed {
            width: resolution[0],
            height: resolution[1],
        };

        let mut surface = GlfwSurface::new_gl33(title, WindowOpt::default().set_dim(dim)).unwrap();

        surface.context.window.glfw.set_swap_interval(
            if vsync { SwapInterval::Adaptive } else { SwapInterval::None }
        );

        Instance {
            surface,
            fps: 0,
        }
    }

    pub fn run_loop(&mut self) {
        let start_t = Instant::now();
        let mut ctxt = &mut self.surface.context;
        let events = &mut self.surface.events_rx;
        let back_buffer = ctxt.back_buffer().expect("back buffer");

        let mut frm = 0;
        let mut last = start_t.elapsed().as_secs();

        'app: loop {
            // handle events
            ctxt.window.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                match event {
                    WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => break 'app,
                    _ => ()
                }
            }

            // rendering code goes here
            let t = start_t.elapsed().as_secs_f32();
            let color = [t.cos(), t.sin(), 0.5, 1.];

            let render = ctxt
                .new_pipeline_gate()
                .pipeline(
                    &back_buffer,
                    &PipelineState::default().set_clear_color(color),
                    |_, _| Ok(())
                )
                .assume();

            // fps counter
            if last == start_t.elapsed().as_secs() {
                frm += 1;
            } else {
                self.fps = frm;
                frm = 0;
            }
            last = start_t.elapsed().as_secs();

            // swap buffer chain
            if render.is_ok() {
                ctxt.window.swap_buffers();
            } else {
                break 'app;
            }
        }
    }
}

