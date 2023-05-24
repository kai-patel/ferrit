#![allow(dead_code)]
use crate::engine;

const CHUNK_SIZE: usize = 16;

#[derive(Copy, Clone, Default, Debug)]
enum BlockType {
    #[default]
    EMPTY,
    SOLID,
}

#[derive(Debug)]
struct Chunk {
    blk: [[[BlockType; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    elements: usize,
    changed: bool,
    vbo: wgpu::Buffer,
}

impl Default for Chunk {
    fn default() -> Self {
        let vbo = engine::gen_buffers(1).remove(0);
        Self {
            blk: Default::default(),
            elements: 0,
            changed: false,
            vbo,
        }
    }
}

impl Chunk {
    pub fn new() -> Self {
        Chunk::default()
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> BlockType {
        self.blk[x][y][z]
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, block_type: BlockType) {
        self.blk[x][y][z] = block_type;
        self.changed = true;
    }

    pub fn update(&mut self) {
        self.changed = false;

        let mut i = 0;

        let mut vertices = &mut [engine::Vertex {
            position: [0, 0, 0],
            block_type: BlockType::EMPTY as u8,
        }; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE * 6 * 6];

        for x in 0..CHUNK_SIZE as u8 {
            for y in 0..CHUNK_SIZE as u8 {
                for z in 0..CHUNK_SIZE as u8 {
                    let block_type = self.blk[x as usize][y as usize][z as usize];
                    let b_type = block_type as u8;
                    let vs = vec![
                        [x, y, z],
                        [x, y, z + 1],
                        [x, y + 1, z],
                        [x, y + 1, z],
                        [x, y, z + 1],
                        [x, y + 1, z + 1],
                        [x + 1, y, z],
                        [x + 1, y + 1, z],
                        [x + 1, y, z + 1],
                        [x + 1, y + 1, z],
                        [x + 1, y + 1, z + 1],
                        [x + 1, y, z + 1],
                        [x, y, z],
                        [x + 1, y, z],
                        [x, y, z + 1],
                        [x + 1, y, z],
                        [x + 1, y, z + 1],
                        [x, y, z + 1],
                        [x, y + 1, z],
                        [x, y + 1, z + 1],
                        [x + 1, y + 1, z],
                        [x + 1, y + 1, z],
                        [x, y + 1, z + 1],
                        [x + 1, y + 1, z + 1],
                        [x, y, z],
                        [x, y + 1, z],
                        [x + 1, y, z],
                        [x, y + 1, z],
                        [x + 1, y + 1, z],
                        [x + 1, y, z],
                        [x, y, z + 1],
                        [x + 1, y, z + 1],
                        [x, y + 1, z + 1],
                        [x, y + 1, z + 1],
                        [x + 1, y, z + 1],
                        [x + 1, y + 1, z + 1],
                    ];
                    for j in 0..36 {
                        vertices[i].position = vs[j];
                        vertices[i].block_type = b_type;
                        i += 1;
                    }
                }
            }
        }

        self.elements = i;
        let vbo = engine::create_buffer(vertices);
    }

    pub fn render(&mut self) {
        if self.changed {
            self.update();
            todo!("Render VBO");
        }
    }
}
