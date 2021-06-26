use luminance_windowing::{WindowDim, WindowOpt};
use luminance_glfw::GlfwSurface;
use glfw::{WindowEvent, Key, Action, Context, SwapInterval};
use std::time::Instant;
use crate::handler::Handler;
use crate::context::Context as VXEContext;

pub mod builder;


/// Renderer structure, contains a handler and window handles
#[allow(dead_code)]
pub struct Renderer {
    surface: GlfwSurface,
    fps: u32,
}

impl Renderer {
    pub(crate) fn init(title: String, resolution: [u32; 2], vsync: bool) -> Renderer {
        let dim = WindowDim::Windowed {
            width: resolution[0],
            height: resolution[1],
        };

        let mut surface = GlfwSurface::new_gl33(title, WindowOpt::default().set_dim(dim)).unwrap();

        surface.context.window.glfw.set_swap_interval(
            if vsync { SwapInterval::Adaptive } else { SwapInterval::None }
        );

        Renderer {
            surface,
            fps: 0,
        }
    }

    pub fn run_loop<H>(&mut self, mut handler: H)
    where
        H: Handler
    {
        let start_t = Instant::now();
        let mut ctxt = &mut self.surface.context;
        let events = &mut self.surface.events_rx;

        handler.init(&mut VXEContext::new(&mut ctxt));

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

            // fps counter
            if last == start_t.elapsed().as_secs() {
                frm += 1;
            } else {
                self.fps = frm;
                frm = 0;
            }
            last = start_t.elapsed().as_secs();

            handler.draw(&mut VXEContext::new(&mut ctxt));

            // swap buffers
            ctxt.window.swap_buffers();
        }
    }
}
