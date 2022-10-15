use macroquad::prelude::*;

pub enum Uniform {
    Int(String),
    Float1(String),
    Float2(String, String),
    Float3(String, String, String),
    Float4(String, String, String, String),
    Color(Vec3),
}

impl Uniform {
    pub fn uniform_type(&self) -> UniformType {
        match self {
            Uniform::Float1(_) => UniformType::Float1,
            Uniform::Float2(_, _) => UniformType::Float2,
            Uniform::Float3(_, _, _) => UniformType::Float3,
            Uniform::Float4(_, _, _, _) => UniformType::Float4,
            Uniform::Int(_) => UniformType::Int1,
            Uniform::Color(_) => UniformType::Float3,
        }
    }
}
