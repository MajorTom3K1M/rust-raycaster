use crate::utils::{buffer::Buffer, shader::Shader, vertex_array::VertexArray};
use glow::{Context, HasContext};
pub struct Graphics {
    body_vao: VertexArray,
    body_vbo: Buffer,
    angle_vao: VertexArray,
    angle_vbo: Buffer,
    body_shader: Shader,
}

impl Graphics {
    pub unsafe fn new(gl: &Context, position: [f32; 3]) -> Self {
        let body_position = position;
        let angle: Vec<f32> = [body_position.clone(), [0.0, 0.1, 0.0]].concat();

        let vertex_source = "
            #version 330 core
            layout (location = 0) in vec3 aPos;

            void main() {
                gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0f);
                gl_PointSize = 8.0;
            }
        ";
        let fragment_source = "
            #version 330 core
            out vec4 FragColor;

            void main() {
                FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
            }
        ";

        let body_vao = VertexArray::new(&gl);
        let body_vbo = Buffer::new(&gl, body_position.to_vec(), glow::DYNAMIC_DRAW, 0);
        gl.enable(glow::PROGRAM_POINT_SIZE);

        let angle_vao = VertexArray::new(&gl);
        let angle_vbo = Buffer::new(&gl, angle, glow::DYNAMIC_DRAW, 0);
        gl.enable(glow::LINE_SMOOTH);
        gl.enable(glow::LINE_STRIP);
        gl.enable(glow::LINE_WIDTH);
        gl.line_width(1.0);

        let shader_program = Shader::new(&gl, vertex_source, fragment_source);


        Self {
            body_vao: body_vao,
            body_vbo: body_vbo,
            angle_vao: angle_vao,
            angle_vbo: angle_vbo,
            body_shader: shader_program,
        }
    }

    pub unsafe fn set_move(
        &self,
        gl: &Context,
        new_body_vertices: Vec<f32>,
        new_angle_vertices: Vec<f32>,
    ) {
        let line_angle_vertices = vec![new_angle_vertices[0] + new_body_vertices[0], new_angle_vertices[1] + new_body_vertices[1]];
        let new_angle = [new_body_vertices.clone(), line_angle_vertices].concat();
        self.body_vbo.set_buffer(&gl, new_body_vertices);
        self.angle_vbo.set_buffer(&gl, new_angle);
    }

    pub unsafe fn draw(&self, gl: &Context) {
        self.body_shader.activate(&gl);
        self.body_vao.bind(&gl);
        gl.draw_arrays(glow::POINTS, 0, 1);

        self.angle_vao.bind(&gl);
        gl.draw_arrays(glow::LINES, 0, 2);
    }
}
