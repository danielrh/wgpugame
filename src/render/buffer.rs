use crate::util::size_of_slice;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

pub const U32_SIZE: wgpu::BufferAddress = std::mem::size_of::<u32>() as wgpu::BufferAddress;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vertex {
    #[allow(dead_code)]
    position: cgmath::Vector4<f32>,
    color: Color,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Color {
    color: cgmath::Vector4<f32>,
}
impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            color: (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0).into(),
        }
    }
    pub fn to_vec4(&self) -> cgmath::Vector4<f32> {
        self.color
    }
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

impl Vertex {
    pub const SIZE: wgpu::BufferAddress = std::mem::size_of::<Self>() as wgpu::BufferAddress;
    pub const DESC: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: Self::SIZE,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![
            0 => Float32x4,
            1 => Float32x4,
        ],
    };
}

pub struct QuadBufferBuilder {
    vertex_data: Vec<Vertex>,
    index_data: Vec<u32>,
}

impl QuadBufferBuilder {
    pub fn new() -> Self {
        Self {
            vertex_data: Vec::new(),
            index_data: Vec::new(),
        }
    }

    pub fn push_player(self, player: &crate::game::Player) -> Self {
        if player.visible {
            self.push_quad(
                player.position.x - player.size.x * 0.5,
                player.position.y - player.size.y * 0.5,
                player.position.x + player.size.x * 0.5,
                player.position.y + player.size.y * 0.5,
            )
        } else {
            self
        }
    }
    pub fn push_quad2d(
        mut self,
        center: cgmath::Vector2<f32>,
        size: cgmath::Vector2<f32>,
        color: Color,
    ) -> Self {
        let min_x = center.x - size.x * 0.5;
        let min_y = center.y - size.y * 0.5;
        let max_x = center.x + size.x * 0.5;
        let max_y = center.y + size.y * 0.5;

        self.vertex_data.extend(&[
            Vertex {
                position: (min_x, min_y, 0.0, 1.0).into(),
                color,
            },
            Vertex {
                position: (max_x, min_y, 0.0, 1.0).into(),
                color,
            },
            Vertex {
                position: (max_x, max_y, 0.0, 1.0).into(),
                color,
            },
            Vertex {
                position: (min_x, max_y, 0.0, 1.0).into(),
                color,
            },
        ]);
        let vertex_len = self.vertex_data.len() as u32;
        self.index_data.extend(&[
            vertex_len - 4,
            vertex_len - 3,
            vertex_len - 2,
            vertex_len - 4,
            vertex_len - 2,
            vertex_len - 1,
        ]);
        self
    }
    pub fn push_circle2d(
        mut self,
        center: cgmath::Vector2<f32>,
        size: cgmath::Vector2<f32>,
        color: Color,
    ) -> Self {
        let divisions = (1024 * size.x.max(size.y) as usize).min(256).max(64);
        let mut last_vertex = Vertex {
            position: (center.x + size.x * 0.5, center.y, 0.0, 1.0).into(),
            color,
        };

        for i in 0..divisions {
            let angle = 2.0 * std::f32::consts::PI * (i + 1) as f32 / divisions as f32;
            let vertex = Vertex {
                position: (
                    center.x + size.x * 0.5 * f32::cos(angle),
                    center.y + size.y * 0.5 * f32::sin(angle),
                    0.0,
                    1.0,
                )
                    .into(),
                color,
            };
            self.vertex_data.extend(&[
                Vertex {
                    position: (center.x, center.y, 0.0, 1.0).into(),
                    color,
                },
                last_vertex,
                vertex,
            ]);
            let vertex_len = self.vertex_data.len() as u32;
            self.index_data
                .extend(&[vertex_len - 3, vertex_len - 2, vertex_len - 1]);
            last_vertex = vertex;
        }
        self
    }

    pub fn push_tri2d(
        mut self,
        a: cgmath::Vector2<f32>,
        b: cgmath::Vector2<f32>,
        c: cgmath::Vector2<f32>,
        color: Color,
    ) -> Self {
        self.vertex_data.extend(&[
            Vertex {
                position: (a.x, a.y, 0.0, 1.0).into(),
                color,
            },
            Vertex {
                position: (b.x, b.y, 0.0, 1.0).into(),
                color,
            },
            Vertex {
                position: (c.x, c.y, 0.0, 1.0).into(),
                color,
            },
        ]);
        let vertex_len = self.vertex_data.len() as u32;
        self.index_data
            .extend(&[vertex_len - 3, vertex_len - 2, vertex_len - 1]);
        self
    }

    pub fn push_quad(mut self, min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        self.vertex_data.extend(&[
            Vertex {
                position: (min_x, min_y, 0.0, 1.0).into(),
                color: Color::new(255, 255, 255),
            },
            Vertex {
                position: (max_x, min_y, 0.0, 1.0).into(),
                color: Color::new(255, 255, 255),
            },
            Vertex {
                position: (max_x, max_y, 0.0, 1.0).into(),
                color: Color::new(255, 255, 255),
            },
            Vertex {
                position: (min_x, max_y, 0.0, 1.0).into(),
                color: Color::new(255, 255, 255),
            },
        ]);
        let vertex_len = self.vertex_data.len() as u32;
        self.index_data.extend(&[
            vertex_len - 4,
            vertex_len - 3,
            vertex_len - 2,
            vertex_len - 4,
            vertex_len - 2,
            vertex_len - 1,
        ]);
        self
    }

    pub fn build(self, device: &wgpu::Device) -> (StagingBuffer, StagingBuffer, u32) {
        (
            StagingBuffer::new(device, &self.vertex_data, false),
            StagingBuffer::new(device, &self.index_data, true),
            self.index_data.len() as u32,
        )
    }
}

pub struct StagingBuffer {
    buffer: wgpu::Buffer,
    size: wgpu::BufferAddress,
}

impl StagingBuffer {
    pub fn new<T: bytemuck::Pod + Sized>(
        device: &wgpu::Device,
        data: &[T],
        is_index_buffer: bool,
    ) -> StagingBuffer {
        StagingBuffer {
            buffer: device.create_buffer_init(&BufferInitDescriptor {
                contents: bytemuck::cast_slice(data),
                usage: wgpu::BufferUsages::COPY_SRC
                    | if is_index_buffer {
                        wgpu::BufferUsages::INDEX
                    } else {
                        wgpu::BufferUsages::empty()
                    },
                label: Some("Staging Buffer"),
            }),
            size: size_of_slice(data) as wgpu::BufferAddress,
        }
    }

    pub fn copy_to_buffer(&self, encoder: &mut wgpu::CommandEncoder, other: &wgpu::Buffer) {
        encoder.copy_buffer_to_buffer(&self.buffer, 0, other, 0, self.size)
    }
}
