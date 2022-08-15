use glow::{Context, HasContext};

pub struct Shader {
    id: <Context as HasContext>::Program,
}

impl Shader {
    pub unsafe fn new(gl: &Context, vertex_source: &str, fragment_source: &str) -> Self {
        // Compile individual shaders into OpenGL objects
        let vertex_shader = Self::build_shader(gl, glow::VERTEX_SHADER, vertex_source);
        let fragment_shader = Self::build_shader(gl, glow::FRAGMENT_SHADER, fragment_source);

        let shader_program = gl.create_program().unwrap();
        gl.attach_shader(shader_program, vertex_shader);
        gl.attach_shader(shader_program, fragment_shader);

        gl.link_program(shader_program);
        if !gl.get_program_link_status(shader_program) {
            panic!(
                "Shader program failed to link with error: {}",
                gl.get_program_info_log(shader_program)
            );
        }

        Shader { id: shader_program }
    }

    unsafe fn build_shader(
        gl: &Context,
        shader_type: u32,
        source: &str,
    ) -> <Context as HasContext>::Shader {
        // Create a new OpenGL shader object
        let shader = gl.create_shader(shader_type).unwrap();

        // Pass source to OpenGL
        gl.shader_source(shader, source);

        // Call the OpenGL shader compiler
        gl.compile_shader(shader);
        if !gl.get_shader_compile_status(shader) {
            panic!(
                "Shader failed to compile with error: {}",
                gl.get_shader_info_log(shader)
            );
        }

        return shader;
    }

    pub unsafe fn activate(&self, gl: &Context) {
        // println!("id : {:?}", self.id);
        gl.use_program(Some(self.id));
    }
}
