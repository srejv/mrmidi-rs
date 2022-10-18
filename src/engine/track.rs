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

    pub output_name: String,

    pub clear_color: Option<Color>,
}

impl Track {
    pub fn new(name: String) -> Self {
        let vertex_shader = std::fs::read_to_string("assets/shaders/default.vert").unwrap();
        let fragment_shader = std::fs::read_to_string("assets/shaders/default.frag").unwrap();

        let pipeline_params = PipelineParams {
            depth_write: true,
            depth_test: Comparison::LessOrEqual,
            ..Default::default()
        };

        let render_target = render_target(320, 150);
        render_target.texture.set_filter(FilterMode::Nearest);

        let mut material_params = MaterialParams {
            pipeline_params,
            ..Default::default()
        };
        material_params
            .uniforms
            .push(("iTime".to_owned(), UniformType::Float1));
        material_params
            .uniforms
            .push(("iTimeDelta".to_owned(), UniformType::Float1));
        material_params
            .uniforms
            .push(("iFrame".to_owned(), UniformType::Int1));

        let material = load_material(&vertex_shader, &fragment_shader, material_params).unwrap();

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

            output_name: name,

            clear_color: None,
        }
    }
}

pub fn render_track(track: &mut Track) {
    set_camera(&track.camera);

    if let Some(clear_color) = track.clear_color {
        clear_background(clear_color);
    } else {
        clear_background(WHITE);
    }

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
        MyMesh::Cube => draw_cube(vec3(0., 5., 0.), vec3(10., 10., 10.), track.texture0, WHITE),
    }
    gl_use_default_material();
}
