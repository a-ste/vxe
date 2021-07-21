use luminance_windowing::{WindowDim, WindowOpt};
use luminance_glfw::GlfwSurface;
use glfw::{WindowEvent, Key, Action, Context, SwapInterval};
use std::time::{Instant, Duration};
use crate::handler::Handler;
use crate::context::Context as VXEContext;
use std::thread;

pub mod builder;


/// Renderer structure, contains a handler and window handles
#[allow(dead_code)]
pub struct Renderer {
    surface: GlfwSurface,
    limit: f32,
    fps: u32,
    delta: f32,
}

impl Renderer {
    pub(crate) fn init(title: String, resolution: [u32; 2], vsync: bool, limit: f32) -> Renderer {
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
            delta: 0.0,
            limit,
        }
    }

    /// Runs the renderer loop, under the hood handles window events and runs handler's functions
    pub fn run_loop<H>(&mut self, mut handler: H)
    where
        H: Handler
    {
        let start_t = Instant::now();
        let mut ctxt = &mut self.surface.context;
        let events = &mut self.surface.events_rx;

        handler.init(&mut VXEContext::new(&mut ctxt, self.fps, self.delta));

        let mut frm = 0;
        let mut last = start_t.elapsed().as_secs();

        let mut last_limit = 0.0;
        let mut last_render = 0.0;

        'app: loop {
            // handle events
            ctxt.window.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                match event {
                    WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => break 'app,
                    _ => ()
                }
            }

            // call update
            handler.update(&mut VXEContext::new(&mut ctxt, self.fps, self.delta));

            // fps counter
            if last == start_t.elapsed().as_secs() {
                frm += 1;
            } else {
                self.fps = frm;
                frm = 0;
            }
            last = start_t.elapsed().as_secs();

            // drawing frame
            handler.draw(&mut VXEContext::new(&mut ctxt, self.fps, self.delta));

            // render time measurements
            let after_render = start_t.elapsed().as_secs_f32();

            // delta time
            self.delta = after_render - last_render;
            last_render = after_render;

            // fps limiter
            let limit_delta = 1.0 / (self.limit + 1.0) - (after_render - last_limit);

            if self.limit > 0.0001 {
                if limit_delta > 0.0 {
                    thread::sleep(Duration::from_secs_f32(limit_delta));
                }
            }

            if limit_delta < -0.03 {
                last_limit = after_render - 0.03;
            } else {
                last_limit = after_render + limit_delta;
            }

            // swap buffers
            ctxt.window.swap_buffers();
        }
    }
}

