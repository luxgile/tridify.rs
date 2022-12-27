use std::path::Path;

use glium::Program;

use crate::{core::Window, LErr};

use super::{Uniform, UniformBuffer};

///Used to tell the GPU how to draw the shapes provided.
#[derive(Debug)]
pub struct Brush {
    program: Program,
    uniform_buffer: UniformBuffer,
}

impl Brush {
    pub fn from_base_unlit(wnd: &impl Window) -> Self {
        Brush::from_source(
            wnd,
            brush_templates::UNLIT_VERT.to_string(),
            brush_templates::UNLIT_FRAG.to_string(),
            None,
        )
    }

    pub fn from_path(
        wnd: &impl Window, vertex: &Path, fragment: &Path, geometry: Option<&Path>,
    ) -> Result<Self, LErr> {
        let geometry_source = if let Some(path) = geometry {
            let source = std::fs::read_to_string(path);
            Some(source.unwrap())
        } else {
            None
        };
        Ok(Self::from_source(
            wnd,
            std::fs::read_to_string(vertex.canonicalize()?)?,
            std::fs::read_to_string(fragment.canonicalize()?)?,
            geometry_source,
        ))
    }
    pub fn from_source<'a>(
        wnd: &impl Window, vertex: String, fragment: String, geometry: Option<String>,
    ) -> Self {
        let program = glium::Program::from_source(
            wnd.display(),
            vertex.as_str(),
            fragment.as_str(),
            geometry.as_deref(),
        )
        .unwrap();
        Self {
            program,
            uniform_buffer: UniformBuffer::new(Vec::new()),
        }
    }
    ///Adds a uniform to the brush. Returns error if the uniform already exists.
    pub fn add_uniform(&mut self, uniform: Uniform) -> Result<(), LErr> {
        self.uniform_buffer.add_uniform(uniform)
    }
    ///Changes or adds a uniform to the brush.
    pub fn update_uniform(&mut self, uniform: Uniform) -> Result<(), LErr> {
        if self.uniform_buffer.has_uniform(&uniform) {
            return self.uniform_buffer.change_uniform(uniform);
        } else {
            return self.uniform_buffer.add_uniform(uniform);
        }
    }
    ///Changes or adds a uniform to the brush. Returns error if the uniform does not exist.
    pub fn change_uniform(&mut self, uniform: Uniform) -> Result<(), LErr> {
        self.uniform_buffer.change_uniform(uniform)
    }
    ///Removes all uniforms from the brush.
    pub fn clear_uniforms(&mut self) { self.uniform_buffer.clear(); }

    /// Get a reference to the brush's program.
    #[must_use]
    pub fn program(&self) -> &Program { &self.program }

    /// Get a reference to the brush's uniform buffer.
    #[must_use]
    pub fn uniform_buffer(&self) -> &UniformBuffer { &self.uniform_buffer }
}

mod brush_templates {
    pub const UNLIT_FRAG: &str = r#"
        #version 330 core
        in vec4 frag_color;
        in vec2 frag_uv;

        uniform sampler2D main_tex;
        
        out vec4 out_color;
        
        void main(){
            out_color=vec4(frag_color)*texture(main_tex,frag_uv);
        }"#;

    pub const UNLIT_VERT: &str = r#"
        #version 330 core
        in vec3 pos;
        in vec4 color;
        in vec2 uv;

        uniform mat4 mvp;

        out vec4 frag_color;
        out vec2 frag_uv;

        void main(){
            frag_uv=uv;
            frag_color=color;
            gl_Position=mvp*vec4(pos,1.);
        }"#;
}
