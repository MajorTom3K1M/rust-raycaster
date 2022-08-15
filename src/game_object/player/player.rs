use super::graphics::Graphics;
use glow::{Context, HasContext};
use glutin::event::ElementState;
use winit::event::{Event, WindowEvent, VirtualKeyCode as Key};

pub struct Player {
    player: Graphics,
    speed: f32,
    position: [f32; 3],
    angle: [f32; 3],
}

impl Player {
    pub unsafe fn new(gl: &Context, position: [f32; 3], angle: [f32; 3]) -> Self {
        let graphics = Graphics::new(&gl, position, angle);
        
        Self {
            player: graphics,
            speed: 1.0,
            position: position,
            angle: angle
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
                        match keycode {
                            Key::Right => {
                                self.position[0] += self.speed * 0.01;
                                self.player.set_move(&gl, self.position.to_vec(), self.angle.to_vec());
                            }
                            Key::Left => {
                                self.position[0] -= self.speed * 0.01;
                                self.player.set_move(&gl, self.position.to_vec(), self.angle.to_vec());
                            }
                            Key::Up => {
                                self.position[1] += self.speed * 0.01;
                                self.player.set_move(&gl, self.position.to_vec(), self.angle.to_vec());
                            }
                            Key::Down => {
                                self.position[1] -= self.speed * 0.01;
                                self.player.set_move(&gl, self.position.to_vec(), self.angle.to_vec());
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
