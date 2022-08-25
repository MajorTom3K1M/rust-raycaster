use crate::{
    game_object::{player::player::Player, rays::graphics::Graphics},
    game_scene::map::map::Map,
    utils::helpers::{convert_to_2d_pixel_coord, degree_to_radian, convert_to_2d_catesian_coord, fixed_angle},
};
use glow::Context;

pub struct Rays {
    rays: Graphics,
    rays_amount: u32
}

impl Rays {
    pub unsafe fn new(gl: &Context, player: &Player, map: &Map, rays_amount: u32) -> Self {
        let rays_indices: Vec<f32> = vec![];
        let line_end = Self::finding_walls(player, map, rays_amount);
        let mut vertex = player.position.clone().to_vec();
        vertex.extend_from_slice(&line_end);
        let graphics = Graphics::new(&gl, vertex);

        Self { rays: graphics, rays_amount: rays_amount }
    }

    pub unsafe fn draw(&self, gl: &Context) {
        self.rays.draw(&gl);
    }

    pub unsafe fn update(&self, gl: &Context, player: &Player, map: &Map) {
        let line_end = Self::finding_walls(player, map, self.rays_amount);
        let mut vertex = player.position.clone().to_vec();
        vertex.extend_from_slice(&line_end);
        self.rays.set_move(&gl, vertex);
    }

    unsafe fn finding_walls(player: &Player, map: &Map, rays_amount: u32) -> Vec<f32> {
        let map_size = map.map_size as f32;
        let map_width = map.width as f32;
        let map_heigth = map.height as f32;
        let player_position =
            convert_to_2d_pixel_coord(player.position[0], player.position[1], 1024.0, 512.0);

        let mut vertex: Vec<f32> = vec![];
        let mut angle = player.angle + 30.0;
        fixed_angle(&mut angle);

        for r in 0..rays_amount {
            //-- Check for Horizontal Line --//
            let hxa: f32;
            let hya: f32;
            let dist: f32;
            let mut ay: f32;
            if angle > 0.0 && angle < 180.0 {
                // facing up
                ay = ((player_position[1] / map_size).floor() * map_size) - 1.0;
                hya = -(map_size);
                hxa = - hya / degree_to_radian(angle).tan();
            } else {
                // facing down
                ay = ((player_position[1] / map_size).floor() * map_size) + map_size;
                hya = map_size;
                hxa = - hya / degree_to_radian(angle).tan();
            }
            let mut ax = ((player_position[1] - ay) / degree_to_radian(angle).tan()) + player_position[0];
            
            loop {
                let mx = (ax / map_size).floor();
                let my = (ay / map_size).floor();
    
                if mx >= map_width || my >= map_heigth || mx < 0.0 || my < 0.0 {
                    dist = f32::MAX;
                    break;
                } else if map.map[(my*map_width+mx) as usize] != 0 {
                    dist = (ax - player_position[0]) / degree_to_radian(angle).cos();
                    break;
                } else {
                    ax += hxa;
                    ay += hya;
                }
            }
    
            //-- Check for Vertical Line --//
            let vxa: f32;
            let vya: f32;
            let vdist: f32;
            let mut bx: f32;
            if angle > 90.0 && angle < 270.0 {
                // facing left
                bx = ((player_position[0] / map_size).floor() * map_size) - 1.0;
                vxa = -(map_size);
                vya = - vxa * degree_to_radian(angle).tan();
            } else {
                // facing right
                bx = ((player_position[0] / map_size).floor() * map_size) + map_size;
                vxa = map_size;
                vya = - vxa * degree_to_radian(angle).tan();
            }
            let mut by = ((player_position[0] - bx) * degree_to_radian(angle).tan()) + player_position[1];
    
            loop {
                let mx = (bx / map_size).floor();
                let my = (by / map_size).floor();
    
                if mx >= map_width || my >= map_heigth || mx < 0.0 || my < 0.0 {
                    vdist = f32::MAX;
                    break;
                } else if map.map[(my*map_width+mx) as usize] != 0 {
                    vdist = (bx - player_position[0]) / degree_to_radian(angle).cos();
                    break;
                } else {
                    bx += vxa;
                    by += vya;
                }
            }
    
            if vdist < dist {
                vertex.extend_from_slice(&convert_to_2d_catesian_coord(bx, by, 1024.0, 512.0));
                // convert_to_2d_catesian_coord(bx, by, 1024.0, 512.0);
            } else {
                vertex.extend_from_slice(&convert_to_2d_catesian_coord(ax, ay, 1024.0, 512.0));
                // convert_to_2d_catesian_coord(ax, ay, 1024.0, 512.0);
            }

            angle -= 1.0;
            fixed_angle(&mut angle);
        }

        return vertex;
    }
}
