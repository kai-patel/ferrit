#![allow(dead_code)]
use crate::engine;

const CHUNK_SIZE: usize = 2;

const CUBE_VERTICES: [[f32; 3]; 24] = [
    [-1.0, -1.0, 1.0],
    [1.0, -1.0, 1.0],
    [1.0, 1.0, 1.0],
    [-1.0, 1.0, 1.0],
    [-1.0, 1.0, -1.0],
    [1.0, 1.0, -1.0],
    [1.0, -1.0, -1.0],
    [-1.0, -1.0, -1.0],
    [1.0, -1.0, -1.0],
    [1.0, 1.0, -1.0],
    [1.0, 1.0, 1.0],
    [1.0, -1.0, 1.0],
    [-1.0, -1.0, 1.0],
    [-1.0, 1.0, 1.0],
    [-1.0, 1.0, -1.0],
    [-1.0, -1.0, -1.0],
    [1.0, 1.0, -1.0],
    [-1.0, 1.0, -1.0],
    [-1.0, 1.0, 1.0],
    [1.0, 1.0, 1.0],
    [1.0, -1.0, 1.0],
    [-1.0, -1.0, 1.0],
    [-1.0, -1.0, -1.0],
    [1.0, -1.0, -1.0],
];

const CUBE_INDICES: &[u16] = &[
    0, 1, 2, 2, 3, 0, // top
    4, 5, 6, 6, 7, 4, // bottom
    8, 9, 10, 10, 11, 8, // right
    12, 13, 14, 14, 15, 12, // left
    16, 17, 18, 18, 19, 16, // front
    20, 21, 22, 22, 23, 20, // back
];

#[derive(Copy, Clone, Default, Debug)]
pub enum BlockType {
    #[default]
    EMPTY,
    SOLID,
}

#[derive(Debug)]
pub struct Chunk {
    blk: [[[BlockType; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    elements: usize,
    changed: bool,
    pub vertices: [engine::Vertex; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE * 24],
    pub indices: [u16; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE * 36],
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            blk: Default::default(),
            elements: 0,
            changed: false,
            vertices: [engine::Vertex::default(); CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE * 24],
            indices: [0u16; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE * 36],
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

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let _block_type = self.blk[x as usize][y as usize][z as usize];

                    let xf = x as f32;
                    let yf = y as f32;
                    let zf = z as f32;

                    for v in CUBE_VERTICES {
                        self.vertices[i].position[0] = xf + v[0];
                        self.vertices[i].position[1] = yf + v[1];
                        self.vertices[i].position[2] = zf + v[2];
                        self.vertices[i].color = self.vertices[i].position;
                        i += 1;
                    }
                }
            }
        }

        // For each cube in the chunk, add the indices
        for a in 0..CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE * 36 {
            self.indices[a] = {
                let n = a / 36;
                CUBE_INDICES[a % 36] + (n * 36) as u16
            };
        }

        self.elements = i;
    }

    pub fn render(&mut self) {
        if self.changed {
            self.update();
            todo!("Render VBO");
        }
    }
}
