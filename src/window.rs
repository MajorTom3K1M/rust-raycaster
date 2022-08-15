use glow::*;
use glutin::event_loop;
use winit::{
    event_loop::EventLoop,
    window::{self as winit_window, WindowBuilder},
};

pub struct Window(glutin::ContextWrapper<glutin::PossiblyCurrent, winit_window::Window>);

impl Window {
    pub fn build(window_builder: WindowBuilder, event_loop: &EventLoop<()>) -> (Self, Context) {
        unsafe {
            let windowed_context = glutin::ContextBuilder::new()
                .with_vsync(true)
                .build_windowed(window_builder, event_loop)
                .unwrap();
            let windowed_context = windowed_context.make_current().unwrap();
            let gl = glow::Context::from_loader_function(|s| windowed_context.get_proc_address(s));

            (Window(windowed_context), gl)
        }
    }

    pub fn winit(&self) -> &winit_window::Window {
        self.0.window()
    }

    pub fn swap_buffer(&self) {
        self.0.swap_buffers().unwrap();
    }
}
