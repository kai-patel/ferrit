#![allow(dead_code)]
use rand::{
    distributions::{Standard, Uniform},
    prelude::Distribution,
};

use crate::engine;

pub const CHUNK_SIZE: usize = 4;
const SCALE: f32 = 0.25;

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

impl BlockType {
    fn get_color(&self) -> [f32; 3] {
        match self {
            BlockType::RED => [1.0, 0.0, 0.0],
            BlockType::BLUE => [0.0, 1.0, 0.0],
            BlockType::GREEN => [0.0, 0.0, 1.0],
            BlockType::YELLOW => [1.0, 1.0, 0.0],
            BlockType::SOLID => [1.0, 0.0, 1.0],
            BlockType::EMPTY => [0.0, 0.0, 0.0],
        }
    }
}

impl Distribution<BlockType> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> BlockType {
        match rng.gen_range(0..=5) {
            0 => BlockType::EMPTY,
            1 => BlockType::SOLID,
            2 => BlockType::RED,
            3 => BlockType::GREEN,
            4 => BlockType::BLUE,
            5 => BlockType::YELLOW,
            _ => BlockType::EMPTY,
        }
    }
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
        for i in 0..CUBE_VERTICES.len() {
            let vertex = {
                let vertex = CUBE_VERTICES[i];
                [vertex[0] * SCALE, vertex[1] * SCALE, vertex[2] * SCALE]
            };


            self.vertices[i] = engine::Vertex {
                position: vertex,
                color: [1.0, 0.0, 0.0],
            }
        }

        for i in 0..CUBE_INDICES.len() {
            self.indices[i] = CUBE_INDICES[i];
        }

        self.changed = false;

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let block_type = self.blk[x][y][z];

                    let xf = x as f32;
                    let yf = y as f32;
                    let zf = z as f32;

                    self.instances[x + CHUNK_SIZE * (y + CHUNK_SIZE * z)] = engine::Instance {
                        position: cgmath::Vector3 {
                            x: xf,
                            y: yf,
                            z: zf,
                        },
                        color: block_type.get_color(),
                        ..Default::default()
                    };
                }
            }
        }

        // println!("{:?}", self.instances);
    }

    pub fn render(&mut self) {
        if self.changed {
            self.update();
            todo!("Render VBO");
        }
    }
}
