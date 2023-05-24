#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [u8; 3],
    pub block_type: u8,
}

impl From<[u8; 3]> for Vertex {
    fn from(value: [u8; 3]) -> Self {
        Self {
            position: [value[0], value[1], value[2]],
            block_type: 0,
        }
    }
}

pub fn gen_buffers(buffers: usize) -> Vec<wgpu::Buffer> {
    todo!("Generate new buffers");
}

pub fn create_buffer(vertices: &[Vertex]) {
    todo!("Create buffer");
}
