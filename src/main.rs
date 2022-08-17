mod game_object;
mod game_scene;
mod utils;
mod window;

use game_object::player::player::Player;
use game_scene::map::map::Map;
use glow::{Context, HasContext};
use window::Window;
use winit::{
    dpi,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

struct GameObject {
    player: Player,
}

impl GameObject {
    fn create_game_object(gl: &Context, w: &Window) -> Self {
        unsafe {
            const PI: f32 = 3.14159;
            let position = [0.0, 0.0, 0.0];
            // let angle = [0.0, 0.1, 0.0];
            let aspect_ratio = w.aspect_ratio();
            let player = Player::new(&gl, position, aspect_ratio);

            Self { player }
        }
    }
}

fn main() {
    unsafe {
        let event_loop = EventLoop::new();
        let window_builder = WindowBuilder::new()
            .with_title("Raycaster")
            .with_inner_size(dpi::LogicalSize::new(1024, 512));

        let (window, gl) = window::Window::build(window_builder, &event_loop);

        
        let window_size = window.winit().inner_size();
        // let aspect_ratio = window.aspect_ratio();
        gl.viewport(0, 0, window_size.width as i32, window_size.height as i32);

        let game_object = GameObject::create_game_object(&gl, &window);
        let map = [
            1,1,1,1,1,1,1,1,
            1,0,1,0,0,0,0,1,
            1,0,1,0,0,0,0,1,
            1,0,1,0,0,0,0,1,
            1,0,0,0,0,0,0,1,
            1,0,0,0,0,1,0,1,
            1,0,0,0,0,0,0,1,
            1,1,1,1,1,1,1,1,
        ];

        let map_object = Map::new(
            &gl,
            64,
            8,
            8,
            window_size.width as f32,
            window_size.height as f32,
            map.to_vec(),
        );

        let draw = move |gl: &Context, game_object: &GameObject| {
            gl.clear_color(0.2, 0.3, 0.3, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
            map_object.draw(&gl);
            game_object.player.draw(&gl);
        };

        let update = move |gl: &Context, event: Event<()>, game_object: &mut GameObject| {
            game_object.player.update(&gl, event);
        };

        run_event_loop(gl, event_loop, window, game_object, draw, update);
    }
}

fn run_event_loop(
    gl: Context,
    event_loop: EventLoop<()>,
    window: Window,
    mut game_object: GameObject,
    draw: impl Fn(&Context, &GameObject) + 'static,
    update: impl Fn(&Context, Event<()>, &mut GameObject) + 'static,
) {
    unsafe {
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::LoopDestroyed => {
                    return;
                }
                Event::MainEventsCleared => {
                    window.winit().request_redraw();
                }
                Event::RedrawRequested(_) => {
                    draw(&gl, &game_object);
                    window.swap_buffer();
                }
                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::Resized(size) => {
                        gl.viewport(0, 0, size.width as i32, size.height as i32);
                    }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                _ => (),
            }

            update(&gl, event, &mut game_object);
        });
    }
}
