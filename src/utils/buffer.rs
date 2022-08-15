use glow::{Context, HasContext};
use std::mem::size_of;

pub struct Buffer {
    id: <Context as HasContext>::Buffer,
}

impl Buffer {
    pub unsafe fn new(gl: &Context, vertices: Vec<f32>, usage: u32, location: u32) -> Self {
        let vbo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

        let (_, vertices_bytes, _) = vertices.align_to::<u8>();
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertices_bytes, usage);
        gl.vertex_attrib_pointer_f32(
            location,
            3,
            glow::FLOAT,
            false,
            3 * size_of::<f32>() as i32,
            0,
        );
        gl.enable_vertex_attrib_array(location);

        Buffer { id: vbo }
    }

    pub unsafe fn new_u32(gl: &Context, vertices: Vec<u32>, usage: u32, normalized: bool) -> Self {
        let vbo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

        let (_, vertices_bytes, _) = vertices.align_to::<u8>();
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertices_bytes, usage);
        gl.vertex_attrib_pointer_f32(
            0,
            3,
            glow::FLOAT,
            normalized,
            3 * size_of::<f32>() as i32,
            0,
        );
        gl.enable_vertex_attrib_array(0);

        Buffer { id: vbo }
    }

    pub unsafe fn set_buffer(&self, gl: &Context, new_vertices: Vec<f32>) {
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.id));

        let (_, new_vertices_bytes, _) = new_vertices.align_to::<u8>();
        gl.buffer_sub_data_u8_slice(
            glow::ARRAY_BUFFER,
            0,
            new_vertices_bytes,
        );
    }
}
