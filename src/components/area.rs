use super::*;

#[derive(Clone, Debug)]
pub struct AreaComponent {
    pub r#type: AreaType,
    pub position: vec2<f32>,
}

#[derive(Clone, Debug)]
pub enum AreaType {
    Circle { radius: f32 },
    Rectangle { size: vec2<f32> },
}

impl VarsProvider for AreaComponent {
    fn extend_vars(&self, vars: &mut Vars, _resources: &Resources) {
        vars.insert(VarName::Position, Var::Vec2(self.position));
        match self.r#type {
            AreaType::Circle { radius } => vars.insert(VarName::Radius, Var::Float(radius)),
            AreaType::Rectangle { size } => vars.insert(VarName::Box, Var::Vec2(size)),
        }
    }
}

impl AreaComponent {
    pub fn new(r#type: AreaType, position: vec2<f32>) -> Self {
        Self { r#type, position }
    }

    pub fn contains(&self, pos: vec2<f32>) -> bool {
        let pos = pos - self.position;
        match self.r#type {
            AreaType::Circle { radius } => pos.len() < radius,
            AreaType::Rectangle { size } => pos.x.abs() < size.x && pos.y.abs() < size.y,
        }
    }

    pub fn from_shader(shader: &Shader) -> Option<Self> {
        if !shader.is_enabled() {
            return None;
        }
        let uniforms = &shader.parameters.uniforms;
        let scale = uniforms
            .try_get_float(&VarName::Scale.uniform())
            .unwrap_or(1.0);
        uniforms
            .try_get_vec2(&VarName::Position.uniform())
            .and_then(|position| {
                let offset = uniforms.try_get_vec2("u_offset").unwrap_or(vec2::ZERO);
                let position = position + offset;
                if let Some(radius) = shader
                    .parameters
                    .uniforms
                    .try_get_float(&VarName::Radius.uniform())
                {
                    Some(Self {
                        r#type: AreaType::Circle {
                            radius: radius * scale,
                        },
                        position,
                    })
                } else if let Some(size) = shader
                    .parameters
                    .uniforms
                    .try_get_vec2(&VarName::Box.uniform())
                {
                    Some(Self {
                        r#type: AreaType::Rectangle { size: size * scale },
                        position,
                    })
                } else {
                    None
                }
            })
    }
}
