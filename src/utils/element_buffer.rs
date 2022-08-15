use glow::{Context, HasContext};

pub struct ElementBuffer {
    id: <Context as HasContext>::Buffer
}

impl ElementBuffer {
    pub unsafe fn new(gl: &Context, indices: Vec<u32>, usage: u32) -> Self {
        let ebo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));

        let (_, indices_bytes, _) = indices.align_to::<u8>();
        gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, indices_bytes, usage);

        Self { id: ebo }
    }
}