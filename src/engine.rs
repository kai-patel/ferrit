use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

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

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            window_id,
            ref event,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        _ => {}
    });
}
