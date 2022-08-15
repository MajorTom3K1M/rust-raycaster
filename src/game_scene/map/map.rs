use glow::Context;

use crate::game_scene::map::graphics::Graphics;

use crate::utils::helpers;

pub struct Map {
    map_size: u32,
    width: u32,
    height: u32,
    map: Vec<u32>,
    map_graphics: Graphics,
}

impl Map {
    pub unsafe fn new(
        gl: &Context,
        map_size: u32,
        width: u32,
        height: u32,
        screen_width: f32,
        screen_height: f32,
        map: Vec<u32>,
    ) -> Self {
        let mut map_vertices: Vec<f32> = vec![];
        let mut map_indices: Vec<u32> = vec![];
        let mut xo: u32;
        let mut yo: u32;

        for y in 0..height {
            for x in 0..width {
                let index = y * width + x;
                if map[index as usize] == 1 {
                    xo = x * map_size;
                    yo = y * map_size;
                    let start_index = (map_vertices.len() / 3) as u32;
                    map_vertices.append(&mut helpers::covert_to_2d_catesian_coord(
                        (map_size + xo - 1) as f32,
                        (0 + yo + 1) as f32,
                        screen_width,
                        screen_height,
                    )); // top right
                    map_vertices.append(&mut helpers::covert_to_2d_catesian_coord(
                        (map_size + xo - 1) as f32,
                        (map_size + yo - 1) as f32,
                        screen_width,
                        screen_height,
                    )); // buttom right
                    map_vertices.append(&mut helpers::covert_to_2d_catesian_coord(
                        (0 + xo + 1) as f32,
                        (map_size + yo - 1) as f32,
                        screen_width,
                        screen_height,
                    )); // buttom left
                    map_vertices.append(&mut helpers::covert_to_2d_catesian_coord(
                        (0 + xo + 1) as f32,
                        (0 + yo + 1) as f32,
                        screen_width,
                        screen_height,
                    )); // top left
                    map_indices.append(&mut vec![
                        start_index + 0,
                        start_index + 1,
                        start_index + 3,
                        start_index + 1,
                        start_index + 2,
                        start_index + 3,
                    ]);
                }
            }
        }
        
        let graphics = Graphics::new(&gl, map_vertices, map_indices);
        Self {
            map_size: map_size,
            width: width,
            height: height,
            map: map,
            map_graphics: graphics,
        }
    }

    pub unsafe fn draw(&self, gl: &Context) {
        self.map_graphics.draw(&gl);
    }
}
