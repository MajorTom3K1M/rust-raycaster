use super::graphics::Graphics;
use glow::{Context, HasContext};
use glutin::event::ElementState;
use winit::event::{Event, WindowEvent, VirtualKeyCode as Key};

const PI: f32 = 3.14159;

pub struct Player {
    player: Graphics,
    speed: f32,
    angle: f32,
    position: [f32; 3],
    delta: [f32; 3],
    aspect_ratio: [f32; 2]
}

impl Player {
    pub unsafe fn new(gl: &Context, position: [f32; 3], aspect_ratio: [f32; 2]) -> Self {
        let graphics = Graphics::new(&gl, position);
        let default_angle = -PI/2.0;
        let default_delta_x = (0.1/aspect_ratio[0])*(-default_angle.cos());
        let default_delta_y = (0.1/aspect_ratio[1])*(-default_angle.sin());
        Self {
            player: graphics,
            speed: 1.0,
            position: position,
            angle: default_angle,
            aspect_ratio,
            delta: [default_delta_x,default_delta_y,0.0]
        }
    }

    pub unsafe fn draw(&self, gl: &Context) {
        self.player.draw(&gl);
    }

    pub unsafe fn update(&mut self, gl: &Context, event: Event<()>) {
        match event {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(keycode) = input.virtual_keycode {
                        // match input.state {
                        //     ElementState::Pressed => {
                        //         println!("key pressed {:?}",keycode);
                        //     }
                        //     _ => {}
                        // }
                        println!("x: {:?}, y: {:?}", self.delta[0], self.delta[1]);
                        match keycode {
                            Key::Right => {
                                self.angle += 0.05;
                                self.delta[0] = (0.1/self.aspect_ratio[0])*(self.angle).cos();
                                self.delta[1] = (0.1/self.aspect_ratio[1])*(-self.angle).sin();
                                self.player.set_move(&gl, self.position.to_vec(), self.delta.to_vec());
                            }
                            Key::Left => {
                                self.angle -= 0.05;
                                self.delta[0] = (0.1/self.aspect_ratio[0])*(self.angle).cos();
                                self.delta[1] = (0.1/self.aspect_ratio[1])*(-self.angle).sin();
                                self.player.set_move(&gl, self.position.to_vec(), self.delta.to_vec());
                            }
                            Key::Up => {
                                self.position[0] += self.delta[0] * 0.5;
                                self.position[1] += self.delta[1] * 0.5;
                                self.player.set_move(&gl, self.position.to_vec(), self.delta.to_vec());
                            }
                            Key::Down => {
                                self.position[0] -= self.delta[0] * 0.5;
                                self.position[1] -= self.delta[1] * 0.5;
                                self.player.set_move(&gl, self.position.to_vec(), self.delta.to_vec());
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
            _ => ()
        }
    }
}
