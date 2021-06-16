use glutin::{GlRequest, WindowedContext, EventsLoop};
use glutin::Api::OpenGl;
use glutin::dpi::LogicalSize;
use gfx::handle::{RenderTargetView, DepthStencilView};
use gfx_device_gl::{Factory, Resources};
use gfx::Device;

pub mod builder;

#[allow(dead_code)]
pub struct Instance {
    window: WindowedContext,
    device: gfx_device_gl::Device,
    factory: Factory,
    color_view: RenderTargetView<Resources, ColorFormat>,
    depth_view: DepthStencilView<Resources, DepthFormat>,
    events_loop: EventsLoop,
}

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

impl Instance {
    pub(crate) fn init(title: String, resolution: [u32; 2], vsync: bool) -> Instance {
        let events_loop = glutin::EventsLoop::new();
        let window_builder = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(LogicalSize::from((resolution[0], resolution[1])));
        let context_builder = glutin::ContextBuilder::new()
            .with_gl(GlRequest::Specific(OpenGl,(3,2)))
            .with_vsync(vsync);
        let (window, device, factory, color_view, depth_view) =
            gfx_window_glutin::init::<ColorFormat, DepthFormat>(window_builder, context_builder, &events_loop).unwrap();

        Instance {
            window,
            device,
            factory,
            color_view,
            depth_view,
            events_loop
        }
    }

    pub fn run_loop(&mut self) {
        let mut running = true;

        while running {
            self.events_loop.poll_events(|event| {
                if let glutin::Event::WindowEvent {event, .. } = event {
                    match event {
                        glutin::WindowEvent::CloseRequested |
                        glutin::WindowEvent::KeyboardInput {
                            input: glutin::KeyboardInput {
                                virtual_keycode: Some(glutin::VirtualKeyCode::Escape), ..
                            }, ..
                        } => running = false,
                        _ => {}
                    }
                }
            });

            self.window.swap_buffers().unwrap();
            self.device.cleanup();
        }
    }
}

