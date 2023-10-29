use glam::{Affine3A, Mat4, Quat, Vec3};

use crate::input_layout::{GpuDataLayout, InputLayoutGroup, InputType};

/// Representation for position, rotation and scale.
#[derive(Clone, Copy)]
pub struct Transform {
    affine: Affine3A,
}

impl Transform {
    pub fn new(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            affine: Affine3A::from_scale_rotation_translation(scale, rotation, position),
        }
    }

    pub fn get_pos(&self) -> Vec3 { self.affine.translation.into() }
    pub fn set_pos(&mut self, pos: Vec3) { self.affine.translation = pos.into() }

    /// Create transform based only on position. Rotation and scale will have default values.
    pub fn from_pos(position: Vec3) -> Self { Transform::new(position, Quat::IDENTITY, Vec3::ONE) }

    /// Create transform based on position and view direction.
    pub fn from_look_to(eye: Vec3, forward: Vec3, up: Vec3) -> Self {
        Self {
            affine: Affine3A::look_to_lh(eye, forward, up),
        }
    }

    /// Create transform based on position and view point.
    pub fn from_look_at(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        Self {
            affine: Affine3A::look_at_lh(eye, center, up),
        }
    }

    pub fn build_matrix(&self) -> Mat4 { Mat4::from(self.affine) }
}
impl Default for Transform {
    fn default() -> Self { Transform::new(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE) }
}
impl GpuDataLayout for Transform {
    fn get_layout() -> crate::input_layout::InputLayoutGroup {
        let mut instance_layout_group = InputLayoutGroup::new_instance();
        instance_layout_group
            .add_input(InputType::Vec4)
            .add_input(InputType::Vec4)
            .add_input(InputType::Vec4)
            .add_input(InputType::Vec4);
        instance_layout_group
    }
}
