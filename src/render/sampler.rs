use wgpu::{ShaderModel, ShaderStages};

use crate::{Graphics, ToBinder};

pub struct Sampler {
    inner_sampler: wgpu::Sampler,
}
impl Sampler {
    pub fn new_default(graphics: &impl Graphics) -> Self {
        Self {
            inner_sampler: graphics
                .get_device()
                .create_sampler(&wgpu::SamplerDescriptor {
                    address_mode_u: wgpu::AddressMode::ClampToEdge,
                    address_mode_v: wgpu::AddressMode::ClampToEdge,
                    address_mode_w: wgpu::AddressMode::ClampToEdge,
                    mag_filter: wgpu::FilterMode::Linear,
                    min_filter: wgpu::FilterMode::Nearest,
                    mipmap_filter: wgpu::FilterMode::Nearest,
                    ..Default::default()
                }),
        }
    }
}
impl ToBinder for Sampler {
    fn get_layout(&self, index: u32) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: index,
            visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        }
    }

    fn get_group(&self, index: u32) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding: index,
            resource: wgpu::BindingResource::Sampler(&self.inner_sampler),
        }
    }
}