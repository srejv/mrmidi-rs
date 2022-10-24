use macroquad::prelude::*;

use crate::engine::mesh::MyMesh;
use crate::engine::uniform::Uniform;

use crate::gif::GifAnimation;
pub struct MuhGif {
    animation: GifAnimation,
    position: Vec2,
}
impl MuhGif {
    pub fn new(animation: GifAnimation) -> Self {
        Self {
            position: vec2(animation.pos_x(), animation.pos_y()),
            animation,
        }
    }

    pub fn draw(&mut self) {
        self.animation.draw_at(self.position.x, self.position.y);
    }

    pub fn tick(&mut self) {
        self.animation.tick();
    }
}

struct MuhMesh {
    mesh_type: MyMesh,
    position: Vec3,
    size: Vec3,
    texture: Option<Texture2D>,
}
impl MuhMesh {
    pub fn new(mesh_type: MyMesh, position: Vec3, size: Vec3, texture: Option<Texture2D>) -> Self {
        Self {
            mesh_type,
            position,
            size,
            texture,
        }
    }

    pub fn draw(&self) {
        match self.mesh_type {
            MyMesh::Plane => draw_plane(
                self.position,
                vec2(self.size.x, self.size.z),
                self.texture,
                WHITE,
            ),
            MyMesh::Sphere => draw_sphere(self.position, self.size.x, self.texture, WHITE),
            MyMesh::Cube => draw_cube(self.position, self.size, self.texture, WHITE),
        }
    }
}

pub struct Track {
    pub pipeline_params: PipelineParams,
    pub material: Material,
    pub render_target: RenderTarget,

    pub vertex_shader: String,
    pub fragment_shader: String,

    pub mesh: MyMesh,

    pub camera: Camera3D,
    pub camera_2d: Camera2D,

    pub texture0: Option<Texture2D>,

    pub uniforms: Vec<(String, Uniform)>,

    pub output_name: String,

    pub clear_color: Option<Color>,

    pub vertex_shader_tree: crate::code::commands::TreeProgram,
    pub fragment_shader_tree: crate::code::commands::TreeProgram,

    pub animation: Option<GifAnimation>,
    pub animations: Vec<MuhGif>,
}

impl Track {
    pub fn new(name: String) -> Self {
        //let vertex_shader = std::fs::read_to_string("assets/shaders/default.vert").unwrap();
        //let fragment_shader = std::fs::read_to_string("assets/shaders/default.frag").unwrap();

        let mut vertex_shader_tree = crate::code::generator::create_default_vertex_shader();
        let mut fragment_shader_tree = crate::code::generator::create_default_fragment_shader();

        println!("Vert Treeprint:");
        vertex_shader_tree.print();

        println!("Frag Treeprint:");
        fragment_shader_tree.print();

        let vertex_shader = vertex_shader_tree.to_string();
        let fragment_shader = fragment_shader_tree.to_string();

        println!("Vertshader to_string:");
        println!("{}", vertex_shader);
        println!("Fragshader to_string:");
        println!("{}", fragment_shader);

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

        let mut camera_2d =
            Camera2D::from_display_rect(Rect::new(0., 0., screen_width(), screen_height()));
        camera_2d.render_target = Some(render_target);

        Self {
            pipeline_params,
            material,
            render_target,

            vertex_shader,
            fragment_shader,

            mesh,
            camera,
            camera_2d,

            texture0: None,

            uniforms: vec![],

            output_name: name,

            clear_color: None,

            fragment_shader_tree: fragment_shader_tree,
            vertex_shader_tree: vertex_shader_tree,

            animation: None,
            animations: Vec::new(),
        }
    }

    pub fn render(&mut self) {
        set_camera(&self.camera);

        if let Some(clear_color) = self.clear_color {
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

        gl_use_material(self.material);
        match self.mesh {
            MyMesh::Plane => draw_plane(vec3(0., 2., 0.), vec2(5., 5.), self.texture0, WHITE),
            MyMesh::Sphere => draw_sphere(vec3(0., 6., 0.), 5., self.texture0, WHITE),
            MyMesh::Cube => draw_cube(vec3(0., 5., 0.), vec3(10., 10., 10.), self.texture0, WHITE),
        }

        set_camera(&self.camera_2d);
        for animation in &mut self.animations {
            animation.draw();
        }

        if let Some(anim) = &self.animation {
            anim.draw();
        }

        gl_use_default_material();
    }
}
