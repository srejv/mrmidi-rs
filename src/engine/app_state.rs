use macroquad::prelude::*;

use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Label, TreeNode},
};

use crate::engine::color_picker::color_picker_texture;
use crate::engine::track::Track;
use crate::engine::built_in_uniforms::BuiltInUniforms;
use crate::engine::mesh::MyMesh;
use crate::engine::uniform::Uniform;



pub struct AppState {
    pub selected_track: usize,
    pub tracks: Vec<Track>,
    pub post_processing_material: Material,

    pub error: Option<String>,
    pub colorpicker_window: bool,
    pub new_uniform_window: bool,
    pub new_uniform_name: String,
    pub color_picking_uniform: Option<String>,

    pub color_picker_texture: Texture2D,
    pub color_picker_image: Image,

    pub show_gui: bool,

    pub built_in_uniforms: BuiltInUniforms,
}

impl AppState {
    pub fn new() -> Self {
        let post_processing_material =
            load_material(CRT_VERTEX_SHADER, CRT_FRAGMENT_SHADER, Default::default()).unwrap();

        let (color_picker_texture, color_picker_image) = color_picker_texture(200, 200);

        Self {
            selected_track: 0,
            tracks: Vec::new(),
            post_processing_material,
            error: None,
            colorpicker_window: false,
            new_uniform_window: false,
            new_uniform_name: String::new(),
            color_picking_uniform: None,
            color_picker_texture,
            color_picker_image,
            show_gui: false,

            built_in_uniforms: BuiltInUniforms::new(),
        }
    }
}

pub fn render_gui(app_state: &mut AppState) {
    set_default_camera();

    let track = &mut app_state.tracks[app_state.selected_track];
    let mut need_update = false;

    widgets::Window::new(hash!(), vec2(20., 20.), vec2(470., 650.))
        .label("Shader")
        .ui(&mut *root_ui(), |ui| {
            ui.label(
                None,
                &format!("Selected track: {}", &app_state.selected_track),
            );
            ui.label(None, "Camera: ");
            ui.same_line(0.0);
            if ui.button(None, "Ortho") {
                track.camera.projection = Projection::Orthographics;
            }
            ui.same_line(0.0);
            if ui.button(None, "Perspective") {
                track.camera.projection = Projection::Perspective;
            }
            ui.label(None, "Mesh: ");
            ui.same_line(0.0);
            if ui.button(None, "Sphere") {
                track.mesh = MyMesh::Sphere;
            }
            ui.same_line(0.0);
            if ui.button(None, "Cube") {
                track.mesh = MyMesh::Cube;
            }
            ui.same_line(0.0);
            if ui.button(None, "Plane") {
                track.mesh = MyMesh::Plane;
            }

            ui.label(None, "Uniforms:");
            ui.separator();

            for (i, (name, uniform)) in track.uniforms.iter_mut().enumerate() {
                ui.label(None, &format!("{}", name));
                ui.same_line(120.0);

                match uniform {
                    Uniform::Int(x) => {
                        widgets::InputText::new(hash!(hash!(), i))
                            .size(vec2(200.0, 19.0))
                            .filter_numbers()
                            .ui(ui, x);

                        if let Ok(x) = x.parse::<f32>() {
                            track.material.set_uniform(name, x);
                        }
                    }
                    Uniform::Float1(x) => {
                        widgets::InputText::new(hash!(hash!(), i))
                            .size(vec2(200.0, 19.0))
                            .filter_numbers()
                            .ui(ui, x);

                        if let Ok(x) = x.parse::<f32>() {
                            track.material.set_uniform(name, x);
                        }
                    }
                    Uniform::Float2(x, y) => {
                        widgets::InputText::new(hash!(hash!(), i))
                            .size(vec2(99.0, 19.0))
                            .filter_numbers()
                            .ui(ui, x);

                        ui.same_line(0.0);

                        widgets::InputText::new(hash!(hash!(), i))
                            .size(vec2(99.0, 19.0))
                            .filter_numbers()
                            .ui(ui, y);

                        if let (Ok(x), Ok(y)) = (x.parse::<f32>(), y.parse::<f32>()) {
                            track.material.set_uniform(name, (x, y));
                        }
                    }
                    Uniform::Float3(x, y, z) => {
                        widgets::InputText::new(hash!(hash!(), i))
                            .size(vec2(65.0, 19.0))
                            .filter_numbers()
                            .ui(ui, x);

                        ui.same_line(0.0);

                        widgets::InputText::new(hash!(hash!(), i))
                            .size(vec2(65.0, 19.0))
                            .filter_numbers()
                            .ui(ui, y);

                        ui.same_line(0.0);

                        widgets::InputText::new(hash!(hash!(), i))
                            .size(vec2(65.0, 19.0))
                            .filter_numbers()
                            .ui(ui, z);

                        if let (Ok(x), Ok(y), Ok(z)) =
                            (x.parse::<f32>(), y.parse::<f32>(), z.parse::<f32>())
                        {
                            track.material.set_uniform(name, (x, y, z));
                        }
                    }

                    Uniform::Float4(x, y, z, w) => {
                        widgets::InputText::new(hash!(hash!(), i))
                            .size(vec2(65.0, 19.0))
                            .filter_numbers()
                            .ui(ui, x);

                        ui.same_line(0.0);

                        widgets::InputText::new(hash!(hash!(), i))
                            .size(vec2(65.0, 19.0))
                            .filter_numbers()
                            .ui(ui, y);

                        ui.same_line(0.0);

                        widgets::InputText::new(hash!(hash!(), i))
                            .size(vec2(65.0, 19.0))
                            .filter_numbers()
                            .ui(ui, z);

                        ui.same_line(0.0);

                        widgets::InputText::new(hash!(hash!(), i))
                            .size(vec2(65.0, 19.0))
                            .filter_numbers()
                            .ui(ui, w);

                        if let (Ok(x), Ok(y), Ok(z), Ok(w)) = (
                            x.parse::<f32>(),
                            y.parse::<f32>(),
                            z.parse::<f32>(),
                            w.parse::<f32>(),
                        ) {
                            track.material.set_uniform(name, (x, y, z, w));
                        }
                    }

                    Uniform::Color(color) => {
                        let mut canvas = ui.canvas();

                        let cursor = canvas.cursor();

                        canvas.rect(
                            Rect::new(cursor.x + 20.0, cursor.y, 50.0, 18.0),
                            Color::new(0.2, 0.2, 0.2, 1.0),
                            Color::new(color.x, color.y, color.z, 1.0),
                        );

                        if ui.button(None, "change") {
                            app_state.colorpicker_window = true;
                            app_state.color_picking_uniform = Some(name.to_owned());
                        }
                        track
                            .material
                            .set_uniform(name, (color.x, color.y, color.z));
                    }
                }
            }
            ui.separator();
            if ui.button(None, "New uniform") {
                app_state.new_uniform_window = true;
            }
            TreeNode::new(hash!(), "Fragment shader")
                .init_unfolded()
                .ui(ui, |ui| {
                    if ui.editbox(hash!(), vec2(440., 200.), &mut track.fragment_shader) {
                        need_update = true;
                    };
                });
            ui.tree_node(hash!(), "Vertex shader", |ui| {
                if ui.editbox(hash!(), vec2(440., 300.), &mut track.vertex_shader) {
                    need_update = true;
                };
            });

            if let Some(ref error) = app_state.error {
                Label::new(error).multiline(14.0).ui(ui);
            }
        });

    if app_state.new_uniform_window {
        widgets::Window::new(hash!(), vec2(100., 100.), vec2(200., 80.))
            .label("New uniform")
            .ui(&mut *root_ui(), |ui| {
                if ui.active_window_focused() == false {
                    app_state.new_uniform_window = false;
                }
                ui.input_text(hash!(), "Name", &mut app_state.new_uniform_name);
                let uniform_type = ui.combo_box(
                    hash!(),
                    "Type",
                    &["Float1", "Float2", "Float3", "Color"],
                    None,
                );

                if ui.button(None, "Add") {
                    if app_state.new_uniform_name.is_empty() == false {
                        let uniform = match uniform_type {
                            0 => Uniform::Float1("0".to_string()),
                            1 => Uniform::Float2("0".to_string(), "0".to_string()),
                            2 => Uniform::Float3("0".to_string(), "0".to_string(), "0".to_string()),
                            3 => Uniform::Color(vec3(0.0, 0.0, 0.0)),
                            _ => unreachable!(),
                        };
                        track
                            .uniforms
                            .push((app_state.new_uniform_name.clone(), uniform));
                        app_state.new_uniform_name.clear();
                        need_update = true;
                    }
                    app_state.new_uniform_window = false;
                }

                ui.same_line(0.0);
                if ui.button(None, "Cancel") {
                    app_state.new_uniform_window = false;
                }
            });
    }

    if app_state.colorpicker_window {
        app_state.colorpicker_window &=
            widgets::Window::new(hash!(), vec2(140., 100.), vec2(210., 240.))
                .label("Colorpicker")
                .ui(&mut *root_ui(), |ui| {
                    if ui.active_window_focused() == false {
                        app_state.colorpicker_window = false;
                    }

                    let mut canvas = ui.canvas();
                    let cursor = canvas.cursor();
                    let mouse = mouse_position();
                    let x = mouse.0 as i32 - cursor.x as i32;
                    let y = mouse.1 as i32 - (cursor.y as i32 + 20);

                    let color = app_state
                        .color_picker_image
                        .get_pixel(x.max(0).min(199) as u32, y.max(0).min(199) as u32);

                    canvas.rect(
                        Rect::new(cursor.x, cursor.y, 200.0, 18.0),
                        Color::new(0.0, 0.0, 0.0, 1.0),
                        Color::new(color.r, color.g, color.b, 1.0),
                    );
                    canvas.image(
                        Rect::new(cursor.x, cursor.y + 20.0, 200.0, 200.0),
                        app_state.color_picker_texture,
                    );

                    if x >= 0 && x < 200 && y >= 0 && y < 200 {
                        canvas.rect(
                            Rect::new(mouse.0 - 3.5, mouse.1 - 3.5, 7.0, 7.0),
                            Color::new(0.3, 0.3, 0.3, 1.0),
                            Color::new(1.0, 1.0, 1.0, 1.0),
                        );

                        if is_mouse_button_down(MouseButton::Left) {
                            app_state.colorpicker_window = false;
                            let uniform_name = app_state.color_picking_uniform.take().unwrap();

                            track
                                .uniforms
                                .iter_mut()
                                .find(|(name, _)| name == &uniform_name)
                                .unwrap()
                                .1 = Uniform::Color(vec3(color.r, color.g, color.b));
                        }
                    }
                });
    }

    if need_update {
        let uniforms = track
            .uniforms
            .iter()
            .map(|(name, uniform)| (name.clone(), uniform.uniform_type()))
            .collect::<Vec<_>>();

        let pipeline_params = track.pipeline_params;

        match load_material(
            &track.vertex_shader,
            &track.fragment_shader,
            MaterialParams {
                pipeline_params,
                uniforms,
                textures: vec![],
            },
        ) {
            Ok(new_material) => {
                track.material.delete();
                track.material = new_material;
                app_state.error = None;
            }
            Err(err) => {
                app_state.error = Some(format!("{:#?}", err));
            }
        }
    }
}

pub fn update_builtin_uniforms(app_state: &mut AppState) {
    let year = 2022.0;
    let month = 10.0;
    let day = 15.0;
    let second = 0.0;
    app_state.built_in_uniforms.update_date(year, month, day, second);
    
    app_state.post_processing_material.set_uniform("iDate", &app_state.built_in_uniforms.i_date);
    app_state.post_processing_material.set_uniform("iFrame", &app_state.built_in_uniforms.i_frame);
    app_state.post_processing_material.set_uniform("iFrameRate", &app_state.built_in_uniforms.i_frame_rate);
    app_state.post_processing_material.set_uniform("iMouse", &app_state.built_in_uniforms.i_mouse);
    app_state.post_processing_material.set_uniform("iResolution", &app_state.built_in_uniforms.i_resolution);
}

pub fn render_post_process(app_state: &mut AppState) {
    // drawing render targets to the screen
    set_default_camera();
    clear_background(WHITE);

    for track in &mut app_state.tracks {
        gl_use_material(app_state.post_processing_material);
        draw_texture_ex(
            track.render_target.texture,
            0.,
            0.,
            Color::new(1.0, 0.0, 0.0, 1.0), // WHITE
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        gl_use_default_material();
    }
}

const CRT_FRAGMENT_SHADER: &'static str = r#"#version 100
precision lowp float;
varying vec4 color;
varying vec2 uv;


uniform sampler2D Texture;
// https://www.shadertoy.com/view/XtlSD7
vec2 CRTCurveUV(vec2 uv)
{
    uv = uv * 2.0 - 1.0;
    vec2 offset = abs( uv.yx ) / vec2( 6.0, 4.0 );
    uv = uv + uv * offset * offset;
    uv = uv * 0.5 + 0.5;
    return uv;
}
void DrawVignette( inout vec3 color, vec2 uv )
{    
    float vignette = uv.x * uv.y * ( 1.0 - uv.x ) * ( 1.0 - uv.y );
    vignette = clamp( pow( 16.0 * vignette, 0.3 ), 0.0, 1.0 );
    color *= vignette;
}
void DrawScanline( inout vec3 color, vec2 uv )
{
    float iTime = 0.1;
    float scanline 	= clamp( 0.95 + 0.05 * cos( 3.14 * ( uv.y + 0.008 * iTime ) * 240.0 * 1.0 ), 0.0, 1.0 );
    float grille 	= 0.85 + 0.15 * clamp( 1.5 * cos( 3.14 * uv.x * 640.0 * 1.0 ), 0.0, 1.0 );    
    color *= scanline * grille * 1.2;
}
void main() {
    
    vec2 crtUV = CRTCurveUV(uv);
    
    vec3 res = texture2D(Texture, uv).rgb * color.rgb;
 	
    if (crtUV.x < 0.0 || crtUV.x > 1.0 || crtUV.y < 0.0 || crtUV.y > 1.0)
    {
        res = vec3(0.0, 0.0, 0.0);
    } 
    DrawVignette(res, crtUV);
    DrawScanline(res, uv);
    gl_FragColor = vec4(res, 1.0);
}
"#;

const CRT_VERTEX_SHADER: &'static str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
varying lowp vec2 uv;
varying lowp vec4 color;
uniform mat4 Model;
uniform mat4 Projection;
void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    color = color0 / 255.0;
    uv = texcoord;
}
";
