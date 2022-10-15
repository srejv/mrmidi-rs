use macroquad::prelude::*;

use crate::engine::mesh::MyMesh;
use crate::engine::uniform::Uniform;

pub struct Track {
    pub pipeline_params: PipelineParams,
    pub material: Material,
    pub render_target: RenderTarget,

    pub vertex_shader: String,
    pub fragment_shader: String,

    pub mesh: MyMesh,

    pub camera: Camera3D,

    pub texture0: Option<Texture2D>,

    pub uniforms: Vec<(String, Uniform)>,
}

impl Track {
    pub fn new() -> Self {
        let vertex_shader = DEFAULT_VERTEX_SHADER.to_string();
        let fragment_shader = DEFAULT_FRAGMENT_SHADER.to_string();

        let pipeline_params = PipelineParams {
            depth_write: true,
            depth_test: Comparison::LessOrEqual,
            ..Default::default()
        };

        let render_target = render_target(320, 150);
        render_target.texture.set_filter(FilterMode::Nearest);

        let material = load_material(
            &vertex_shader,
            &fragment_shader,
            MaterialParams {
                pipeline_params,
                ..Default::default()
            },
        )
        .unwrap();

        let mesh = MyMesh::Sphere;

        let camera = Camera3D {
            position: vec3(-15., 15., -5.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 5., -5.),
            render_target: Some(render_target),
            ..Default::default()
        };

        Self {
            pipeline_params,
            material,
            render_target,

            vertex_shader,
            fragment_shader,

            mesh,
            camera,

            texture0: None,

            uniforms: vec![],
        }
    }
}

pub fn render_track(track: &mut Track) {
    set_camera(&track.camera);

    clear_background(LIGHTGRAY);

    draw_grid(
        20,
        1.,
        Color::new(0.55, 0.55, 0.55, 0.75),
        Color::new(0.75, 0.75, 0.75, 0.75),
    );

    gl_use_material(track.material);
    match track.mesh {
        MyMesh::Plane => draw_plane(vec3(0., 2., 0.), vec2(5., 5.), track.texture0, WHITE),
        MyMesh::Sphere => draw_sphere(vec3(0., 6., 0.), 5., track.texture0, WHITE),
        MyMesh::Cube => {
            draw_cube(vec3(0., 5., 0.), vec3(10., 10., 10.), track.texture0, WHITE)
        }
    }
    gl_use_default_material();
}

const DEFAULT_FRAGMENT_SHADER: &'static str = "#version 100
precision lowp float;
varying vec2 uv;
uniform sampler2D Texture;
void main() {
    gl_FragColor = texture2D(Texture, uv);
}
";

const DEFAULT_VERTEX_SHADER: &'static str = "#version 100
precision lowp float;
attribute vec3 position;
attribute vec2 texcoord;
varying vec2 uv;
uniform mat4 Model;
uniform mat4 Projection;
void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    uv = texcoord;
}
";
