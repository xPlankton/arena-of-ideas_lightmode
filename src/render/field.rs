use super::*;

impl Render {
    pub fn draw_field(
        &self,
        shader_program: &ShaderProgram,
        game_time: f64,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        let mut instances_arr: ugli::VertexBuffer<Instance> =
            ugli::VertexBuffer::new_dynamic(self.geng.ugli(), Vec::new());
        instances_arr.resize(shader_program.instances, Instance {});
        let quad = shader_program.get_vertices(&self.geng);
        let framebuffer_size = framebuffer.size();
        let window_size = self.geng.window().size();

        ugli::draw(
            framebuffer,
            &shader_program.program,
            ugli::DrawMode::TriangleFan,
            ugli::instanced(&quad, &instances_arr),
            (
                ugli::uniforms! {
                    u_time: game_time,
                    u_unit_position: vec2(0.0,0.0),
                    u_window_size: window_size,
                },
                geng::camera2d_uniforms(&self.camera, framebuffer_size.map(|x| x as f32)),
                &shader_program.parameters,
            ),
            ugli::DrawParameters {
                blend_mode: Some(default()),
                ..default()
            },
        );
    }
}
