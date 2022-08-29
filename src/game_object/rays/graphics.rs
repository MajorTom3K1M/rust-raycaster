use crate::game_object::player;
use crate::utils::helpers::convert_to_2d_catesian_coord;
use crate::utils::{
    buffer::Buffer, element_buffer::ElementBuffer, shader::Shader, vertex_array::VertexArray,
};
use crate::Player;
use glow::{Context, HasContext};

pub struct Graphics {
    rays_vao: VertexArray,
    rays_vbo: Buffer,
    rays_ebo: ElementBuffer,
    rays_shader: Shader,
    walls_horizontal_vao: VertexArray,
    walls_horizontal_vbo: Buffer,
    walls_horizontal_shader: Shader,
    walls_vertical_vao: VertexArray,
    walls_vertical_vbo: Buffer,
    walls_vertical_shader: Shader,
    walls_horizontal_slice: usize,
    walls_vertical_slice: usize,
    indices: usize,
}

impl Graphics {
    pub unsafe fn new(
        gl: &Context,
        vertex: Vec<f32>,
        wall_horizontal: Vec<f32>,
        wall_vertical: Vec<f32>,
    ) -> Self {
        let vertex_source = "
            #version 330 core
            layout (location = 0) in vec3 aPos;

            void main() {
                gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0f);
            }
        ";
        let fragment_source = "
            #version 330 core
            out vec4 FragColor;

            void main() {
                FragColor = vec4(1.0f, 0.0f, 0.0f, 1.0f);
            }
        ";
        let fragment_horizontal_source = "
            #version 330 core
            out vec4 FragColor;

            void main() {
                FragColor = vec4(0.7f, 0.0f, 0.0f, 1.0f);
            }
        ";
        let fragment_vertical_source = "
            #version 330 core
            out vec4 FragColor;

            void main() {
                FragColor = vec4(0.9f, 0.0f, 0.0f, 1.0f);
            }
        ";
        let mut indices: Vec<u32> = vec![];
        for index in 1..(vertex.len() / 3) {
            indices.push(0);
            indices.push(index as u32);
        }
        let indices_len = indices.len();
        let walls_horizontal_slice = wall_horizontal.len() / 3;
        let walls_vertical_slice = wall_vertical.len() / 3;
        
        let rays_vao = VertexArray::new(&gl);
        let rays_vbo = Buffer::new(&gl, vertex, glow::DYNAMIC_DRAW, 0);
        let rays_ebo = ElementBuffer::new(&gl, indices, glow::DYNAMIC_DRAW);

        let walls_horizontal_vao = VertexArray::new(&gl);
        let walls_horizontal_vbo = Buffer::new(&gl, wall_horizontal, glow::DYNAMIC_DRAW, 0);

        let walls_vertical_vao = VertexArray::new(&gl);
        let walls_vertical_vbo = Buffer::new(&gl, wall_vertical, glow::DYNAMIC_DRAW, 0);

        let shader_program = Shader::new(&gl, vertex_source, fragment_source);
        let walls_horizontal_program = Shader::new(&gl, vertex_source, fragment_horizontal_source);
        let walls_vertical_program = Shader::new(&gl, vertex_source, fragment_vertical_source);

        Self {
            rays_vao,
            rays_vbo,
            rays_ebo,
            rays_shader: shader_program,
            indices: indices_len,
            walls_horizontal_vao,
            walls_horizontal_vbo,
            walls_horizontal_shader: walls_horizontal_program,
            walls_vertical_vao,
            walls_vertical_vbo,
            walls_vertical_shader: walls_vertical_program,
            walls_horizontal_slice,
            walls_vertical_slice,
        }
    }

    pub unsafe fn set_move(&self, gl: &Context, vertex: Vec<f32>, walls_horizontal: Vec<f32>, walls_vertical: Vec<f32>) {
        self.rays_vbo.set_buffer(&gl, vertex);
        self.walls_vertical_vbo.set_buffer(&gl, walls_vertical);
        self.walls_horizontal_vbo.set_buffer(&gl, walls_horizontal);
    }

    pub unsafe fn draw(&self, gl: &Context) {
        self.rays_shader.activate(&gl);
        self.rays_vao.bind(&gl);
        gl.line_width(1.0);
        gl.draw_elements(glow::LINES, self.indices as i32, glow::UNSIGNED_INT, 0);

        self.walls_vertical_shader.activate(&gl);
        self.walls_vertical_vao.bind(&gl);
        gl.line_width(8.0);
        gl.draw_arrays(glow::LINES, 0,  self.walls_vertical_slice as i32);

        self.walls_horizontal_shader.activate(&gl);
        self.walls_horizontal_vao.bind(&gl);
        gl.line_width(8.0);
        gl.draw_arrays(glow::LINES, 0, self.walls_horizontal_slice as i32);
    }
}
