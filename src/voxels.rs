#![allow(dead_code)]
use rand::Rng;

use crate::engine;

pub const CHUNK_SIZE: usize = 2;

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
    RED,
    GREEN,
    BLUE,
    YELLOW,
}

#[derive(Debug)]
pub struct Chunk {
    blk: [[[BlockType; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    elements: usize,
    changed: bool,
    pub vertices: Box<[engine::Vertex; CUBE_VERTICES.len()]>,
    pub indices: Box<[u16; CUBE_INDICES.len()]>,
    pub instances: Box<[engine::Instance; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE]>,
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            blk: Default::default(),
            elements: 0,
            changed: false,
            vertices: Box::new([engine::Vertex::default(); CUBE_VERTICES.len()]),
            indices: Box::new([0u16; CUBE_INDICES.len()]),
            instances: Box::new(
                [engine::Instance::default(); CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
            ),
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
        let mut rng = rand::thread_rng();

        for i in 0..CUBE_VERTICES.len() {
            self.vertices[i] = engine::Vertex {
                position: CUBE_VERTICES[i],
                color: [rng.gen(), rng.gen(), rng.gen()],
            }
        }

        println!("{:?}", self.vertices);

        // self.vertices = Box::new(CUBE_VERTICES.map(|v| engine::Vertex {
        //     position: v,
        //     color: [rng.gen(), rng.gen(), rng.gen()],
        // }));

        for i in 0..CUBE_INDICES.len() {
            self.indices[i] = CUBE_INDICES[i];
        }

        self.changed = false;

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let _block_type = self.blk[x][y][z];

                    let xf = x as f32;
                    let yf = y as f32;
                    let zf = z as f32;

                    self.instances[x + CHUNK_SIZE * (y + CHUNK_SIZE * z)] = engine::Instance {
                        position: cgmath::Vector3 {
                            x: xf,
                            y: yf,
                            z: zf,
                        },
                        ..Default::default()
                    };
                }
            }
        }
    }

    pub fn render(&mut self) {
        if self.changed {
            self.update();
            todo!("Render VBO");
        }
    }
}
