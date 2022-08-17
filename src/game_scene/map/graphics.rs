use crate::utils::{
    buffer::Buffer, element_buffer::ElementBuffer, shader::Shader, vertex_array::VertexArray,
};
use glow::{Context, HasContext};

pub struct Graphics {
    map_vao: VertexArray,
    map_vbo: Buffer,
    map_ebo: ElementBuffer,
    shader_black: Shader,
    shader_white: Shader,
    indices: usize
}

impl Graphics {
    pub unsafe fn new(gl: &Context, vertices: Vec<f32>, indices: Vec<u32>) -> Self {
        let vertex_source = "
            #version 330 core
            layout (location = 0) in vec3 bPos;

            void main() {
                gl_Position = vec4(bPos.x, bPos.y, bPos.z, 1.0f);
            }
        ";
        let fragment_white_source = "
            #version 330 core
            out vec4 FragColor;

            void main() {
                FragColor = vec4(1.0f, 1.0f, 1.0f, 1.0f);
            }
        ";
        let fragment_black_source = "
            #version 330 core
            out vec4 FragColor;

            void main() {
                FragColor = vec4(0.0f, 0.0f, 0.0f, 1.0f);
            }
        ";
        let indices_len = indices.len();
        let map_vao = VertexArray::new(&gl);
        let map_vbo = Buffer::new(&gl, vertices, glow::STATIC_DRAW,0);
        let map_ebo = ElementBuffer::new(&gl, indices, glow::STATIC_DRAW);
        let shader_black = Shader::new(&gl, vertex_source, fragment_black_source);
        let shader_white = Shader::new(&gl, vertex_source, fragment_white_source);

        Self {
            map_vao,
            map_vbo,
            map_ebo,
            shader_black: shader_black,
            shader_white: shader_white,
            indices: indices_len
        }
    }

    pub unsafe fn draw(&self, gl: &Context) {
        self.shader_white.activate(&gl);
        self.map_vao.bind(&gl);
        gl.draw_elements(glow::TRIANGLES, self.indices as i32, glow::UNSIGNED_INT, 0);
        // self.shader_black.activate(&gl);
        // gl.draw_arrays(glow::TRIANGLES, 0, self.indices as i32 * 2);
    }
}
