use macroquad::prelude::*;

pub enum InternalUniform {
    Int(i32),
    Float1(f32),
    Float2(f32, f32),
    Float3(f32, f32, f32),
    Float4(f32, f32, f32, f32),
    Color(Vec3),
}

impl InternalUniform {
    pub fn uniform_type(&self) -> UniformType {
        match self {
            InternalUniform::Float1(_) => UniformType::Float1,
            InternalUniform::Float2(_, _) => UniformType::Float2,
            InternalUniform::Float3(_, _, _) => UniformType::Float3,
            InternalUniform::Float4(_, _, _, _) => UniformType::Float4,
            InternalUniform::Int(_) => UniformType::Int1,
            InternalUniform::Color(_) => UniformType::Float3,
        }
    }
}
