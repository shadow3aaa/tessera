use bytemuck::{Pod, Zeroable};
use earcutr::earcut;
use log::error;
use wgpu::{include_wgsl, util::DeviceExt};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable, PartialEq)]
pub struct ShapeUniforms {
    // vec4f: size.x, size.y, corner_radius, border_width
    pub size_cr_border_width: [f32; 4],
    // vec4f: r, g, b, a (fill_color or border_color)
    pub primary_color: [f32; 4],
    // vec4f: r, g, b, a (shadow color)
    pub shadow_color: [f32; 4],
    // vec4f: shadow_offset.x, shadow_offset.y, shadow_smoothness, render_mode
    // render_mode: 0.0 = fill, 1.0 = outline, 2.0 = shadow
    pub render_params: [f32; 4],
}

/// Vertex for any shapes
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable, PartialEq)]
pub struct Vertex {
    /// Position of the vertex(x, y, z)
    pub position: [f32; 3],
    /// Color of the vertex
    pub color: [f32; 3],
    /// Normalized local position relative to rect center
    pub local_pos: [f32; 2],
}

impl Vertex {
    /// Describe the vertex attributes
    /// 0: position (x, y, z)
    /// 1: color (r, g, b)
    /// 2: local_pos (u, v)
    /// The vertex attribute array is used to describe the vertex buffer layout
    const ATTR: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2];

    /// Create a new vertex
    fn new(pos: [f32; 2], color: [f32; 3], local_pos: [f32; 2]) -> Self {
        Self {
            position: [pos[0], pos[1], 0.0],
            color,
            local_pos,
        }
    }

    /// Describe the vertex buffer layout
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: core::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTR,
        }
    }
}

pub struct ShapeVertexData<'a> {
    pub polygon_vertices: &'a [[f32; 2]],
    pub vertex_colors: &'a [[f32; 3]],
    pub vertex_local_pos: &'a [[f32; 2]],
}

pub struct ShapePipeline {
    pipeline: wgpu::RenderPipeline,
    uniform_buffer: wgpu::Buffer,
    #[allow(unused)]
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
}

// Define MAX_CONCURRENT_SHAPES, can be adjusted later
pub const MAX_CONCURRENT_SHAPES: wgpu::BufferAddress = 256;

impl ShapePipeline {
    pub fn new(gpu: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let shader = gpu.create_shader_module(include_wgsl!("shaders/shape.wgsl"));

        let uniform_alignment =
            gpu.limits().min_uniform_buffer_offset_alignment as wgpu::BufferAddress;
        let size_of_shape_uniforms = std::mem::size_of::<ShapeUniforms>() as wgpu::BufferAddress;
        let aligned_size_of_shape_uniforms =
            wgpu::util::align_to(size_of_shape_uniforms, uniform_alignment);

        let uniform_buffer = gpu.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Shape Uniform Buffer"),
            size: MAX_CONCURRENT_SHAPES * aligned_size_of_shape_uniforms,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = gpu.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: true, // Set to true for dynamic offsets
                    min_binding_size: wgpu::BufferSize::new(
                        std::mem::size_of::<ShapeUniforms>() as _
                    ),
                },
                count: None,
            }],
            label: Some("shape_bind_group_layout"),
        });

        let bind_group = gpu.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &uniform_buffer,
                    offset: 0, // Initial offset, will be overridden by dynamic offset
                    size: wgpu::BufferSize::new(std::mem::size_of::<ShapeUniforms>() as _),
                }),
            }],
            label: Some("shape_bind_group"),
        });

        let pipeline_layout = gpu.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Shape Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = gpu.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Shape Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
            cache: None,
        });

        Self {
            pipeline,
            uniform_buffer,
            bind_group_layout,
            bind_group,
        }
    }

    pub fn draw(
        &self,
        gpu: &wgpu::Device,
        gpu_queue: &wgpu::Queue,
        render_pass: &mut wgpu::RenderPass<'_>,
        vertex_data_in: &ShapeVertexData,
        uniforms: &ShapeUniforms,
        dynamic_offset: wgpu::DynamicOffset,
    ) {
        let flat_polygon_vertices: Vec<f64> = vertex_data_in
            .polygon_vertices
            .iter()
            .flat_map(|[x, y]| vec![*x as f64, *y as f64])
            .collect();

        let indices = earcut(&flat_polygon_vertices, &[], 2).unwrap_or_else(|e| {
            error!("Earcut error: {e:?}");
            Vec::new()
        });

        if indices.is_empty() && !vertex_data_in.polygon_vertices.is_empty() {
            return;
        }

        let vertex_data: Vec<Vertex> = indices
            .iter()
            .map(|&i| {
                if i < vertex_data_in.polygon_vertices.len()
                    && i < vertex_data_in.vertex_colors.len()
                    && i < vertex_data_in.vertex_local_pos.len()
                {
                    Vertex::new(
                        vertex_data_in.polygon_vertices[i],
                        vertex_data_in.vertex_colors[i],
                        vertex_data_in.vertex_local_pos[i],
                    )
                } else {
                    error!("Warning: Earcut index {i} out of bounds for input arrays.");
                    // Fallback to the first vertex if index is out of bounds
                    if !vertex_data_in.polygon_vertices.is_empty()
                        && !vertex_data_in.vertex_colors.is_empty()
                        && !vertex_data_in.vertex_local_pos.is_empty()
                    {
                        Vertex::new(
                            vertex_data_in.polygon_vertices[0],
                            vertex_data_in.vertex_colors[0],
                            vertex_data_in.vertex_local_pos[0],
                        )
                    } else {
                        // This case should ideally not happen if inputs are validated
                        // Or handle it by returning early / logging a more severe error
                        Vertex::new([0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0]) // Placeholder
                    }
                }
            })
            .collect();

        if vertex_data.is_empty() {
            return;
        }

        let vertex_buffer = gpu.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Triangulated Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        gpu_queue.write_buffer(
            &self.uniform_buffer,
            dynamic_offset as wgpu::BufferAddress,
            bytemuck::bytes_of(uniforms),
        );

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[dynamic_offset]);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw(0..vertex_data.len() as u32, 0..1);
    }
}
