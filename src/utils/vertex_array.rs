use glow::{Context, HasContext};

pub struct VertexArray {
    id: <Context as HasContext>::VertexArray
}

impl VertexArray {
    pub unsafe fn new(gl: &Context) -> Self {
        let vao = gl.create_vertex_array().unwrap();
        gl.bind_vertex_array(Some(vao));

        VertexArray { id: vao }
    }

    pub unsafe fn bind(&self, gl: &Context) {
        gl.bind_vertex_array(Some(self.id));
    }
}