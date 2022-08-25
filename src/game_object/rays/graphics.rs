use glow::{Context, HasContext};
use crate::Player;
use crate::game_object::player;
use crate::utils::{buffer::Buffer, shader::Shader, vertex_array::VertexArray, element_buffer::ElementBuffer};

pub struct Graphics {
    rays_vao: VertexArray,
    rays_vbo: Buffer,
    rays_ebo: ElementBuffer,
    rays_shader: Shader,
    indices: usize
}

impl Graphics {
    pub unsafe fn new(gl: &Context, vertex: Vec<f32>) -> Self {
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
        let mut indices: Vec<u32> = vec![];
        for index in 1..(vertex.len() / 3) {
            indices.push(0);
            indices.push(index as u32);
        }
        let indices_len = indices.len();
        let rays_vao = VertexArray::new(&gl);
        let rays_vbo = Buffer::new(&gl, vertex, glow::DYNAMIC_DRAW, 0);
        let rays_ebo = ElementBuffer::new(&gl, indices, glow::DYNAMIC_DRAW);
        gl.enable(glow::LINE_SMOOTH);
        gl.enable(glow::LINE_STRIP);
        gl.enable(glow::LINE_WIDTH);
        gl.line_width(1.0);

        let shader_program = Shader::new(&gl, vertex_source, fragment_source);

        Self { rays_vao, rays_vbo, rays_ebo, rays_shader: shader_program, indices: indices_len }
    }

    pub unsafe fn set_move(&self, gl: &Context, vertex: Vec<f32>) {
        self.rays_vbo.set_buffer(&gl, vertex);
    }

    pub unsafe fn draw(&self, gl: &Context) {
        self.rays_shader.activate(&gl);
        self.rays_vao.bind(&gl);
        gl.draw_elements(glow::LINES, self.indices as i32, glow::UNSIGNED_INT, 0);
    }
}
