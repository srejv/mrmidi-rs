use macroquad::prelude::*;

mod code;
mod engine;

use engine::app_state::render_gui;
use engine::app_state::render_post_process;
use engine::app_state::AppState;
use engine::track::render_track;
use engine::track::Track;

use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Label, TreeNode},
};

const NUM_TRACKS: usize = 2;

fn render_command(
    ui: &mut macroquad::ui::Ui,
    program: &mut crate::code::commands::TreeProgram,
    node_idx: usize,
    depth: usize,
) {
    let name = format!(
        "{}{}",
        " ".repeat(depth),
        crate::code::commands::to_strang(&program.tree.arena[node_idx], &program.tree)
    );
    let command_name = format!(
        "{}{}",
        " ".repeat(depth),
        crate::code::commands::type_to_strang(&program.tree.arena[node_idx])
    );

    let num_children = program.tree.arena[node_idx].children.len();
    let n_child_str = format!("{}", &num_children);
    ui.label(None, &command_name);
    ui.label(None, &name);
    ui.label(None, &n_child_str);

    match program.tree.arena[node_idx].val.0 {
        crate::code::commands::CommandType::Value => {
            ui.label(
                None,
                "Value! Should be a leaf! We should probably separate value, variable and type.",
            );
        }
        crate::code::commands::CommandType::Multiply => {
            ui.label(None, "Multiply! Multiplies children together");
        }
        crate::code::commands::CommandType::Add => {
            ui.label(None, "Add! Adds children together");
        }
        crate::code::commands::CommandType::Divide => {
            ui.label(None, "Divide! Divides children together");
        }
        crate::code::commands::CommandType::Subtract => {
            ui.label(None, "Subtract! Subtracts children together");
        }
        crate::code::commands::CommandType::SetVariable => {
            ui.label(None, "SetVariable! First child sets the variable");
        }
        crate::code::commands::CommandType::BeginFunction => {
            ui.label(None, "BeginFunction! First child is type, second child is name, then all the arguments as separate children");
        }
        crate::code::commands::CommandType::EndFunction => {
            ui.label(None, "EndFunction! End function");
        }
        crate::code::commands::CommandType::BeginIf => {
            ui.label(None, "Begin If! First child is check expression");
        }
        crate::code::commands::CommandType::EndIf => {
            ui.label(None, "EndIf! End if");
        }
        crate::code::commands::CommandType::DeclareVariable => {
            ui.label(None, "DeclareVariable! Value is name, child is type?");
        }

        _ => {}
    }
}

struct TreeEditor {
    pub program: crate::code::commands::TreeProgram,
    pub selected_row: usize,
    pub selected_node: usize,
    pub is_open: bool,
}

fn render_tree_editor(tree_editor: &mut TreeEditor) {
    if tree_editor.is_open {
        if is_key_pressed(KeyCode::Up) {
            if tree_editor.program.tree.arena[tree_editor.selected_node]
                .children
                .len()
                == 0
            {
            } else if tree_editor.selected_row > 0 {
                tree_editor.selected_row -= 1;
            }
        }
        if is_key_pressed(KeyCode::Down) {
            if tree_editor.program.tree.arena[tree_editor.selected_node]
                .children
                .len()
                == 0
            {
            } else if tree_editor.selected_row
                < tree_editor.program.tree.arena[tree_editor.selected_node]
                    .children
                    .len()
                    - 1
            {
                tree_editor.selected_row += 1;
            }
        }

        if is_key_pressed(KeyCode::Left) {
            if let Some(parent_idx) =
                tree_editor.program.tree.arena[tree_editor.selected_node].parent
            {
                tree_editor.selected_node = parent_idx;
                tree_editor.selected_row = 0;
            }
        }
        if is_key_pressed(KeyCode::Right) {
            if tree_editor.program.tree.arena[tree_editor.selected_node]
                .children
                .len()
                == 0
            {
            } else if tree_editor.selected_row
                < tree_editor.program.tree.arena[tree_editor.selected_node]
                    .children
                    .len()
            {
                tree_editor.selected_node = tree_editor.program.tree.arena
                    [tree_editor.selected_node]
                    .children[tree_editor.selected_row];
                tree_editor.selected_row = 0;
            }
        }

        widgets::Window::new(
            hash!(),
            vec2(20., 20.),
            vec2(screen_width() - 40., screen_height() - 40.),
        )
        .label("Shader Program")
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, "Selected commands view");

            render_command(ui, &mut tree_editor.program, tree_editor.selected_node, 0);

            ui.label(None, "Commands List");

            ui.separator();

            let mut i = 0;
            let root = &tree_editor.program.tree.arena[tree_editor.selected_node];
            for child_idx in &root.children {
                let child = &tree_editor.program.tree.arena[*child_idx];
                let name = crate::code::commands::to_strang(&child, &tree_editor.program.tree);
                let command_name = crate::code::commands::type_to_strang(&child);
                let selected = if i == tree_editor.selected_row {
                    "->"
                } else {
                    ""
                };
                let label = format!("{}. {}{}: {}", i + 1, selected, command_name, name);
                ui.label(None, &label);
                i += 1;
            }
        });
    }
}

mod gif;

#[macroquad::main("mrmidi-r(eturn)s")]
async fn main() {
    // engine::audio::print_audio_device_status();
    // crate::code::generator::test_program();

    let mut tree_editor = TreeEditor {
        program: crate::code::generator::create_crt_fragment_shader(),
        selected_row: 0,
        selected_node: 0,
        is_open: false,
    };

    let ferris = load_texture("assets/textures/rust.png").await.unwrap();

    let mut tracks = Vec::new();
    for i in 0..NUM_TRACKS {
        let mut track = Track::new(format!("iChannel{}", i));
        track.texture0 = Some(ferris);
        tracks.push(track);
    }
    let mut app_state = AppState::new(tracks);

    app_state.tracks[0].animation =
        Some(crate::gif::GifAnimation::load("assets/gifs/animation.gif".to_string()).await);
    app_state.tracks[1].animation = Some(
        crate::gif::GifAnimation::load(
            "assets/gifs/stolencantuse/Jitter-Pink-perfect-loop-cubes.gif".to_string(),
        )
        .await,
    );

    let time_start = std::time::SystemTime::now();
    let mut time_last_frame = std::time::SystemTime::now();
    let mut frame: i32 = 0;

    let w = macroquad::window::screen_width();
    let h = macroquad::window::screen_height();
    let aspect_ratio = w / h;
    app_state
        .post_processing_material
        .set_uniform("iResolution", (w, h, aspect_ratio));

    app_state.audio.play();
    app_state.midi.start();

    app_state
        .post_processing_material
        .set_texture("iAudio", app_state.audio.audio_tex);

    app_state
        .post_processing_material
        .set_texture("iMidi", app_state.midi.tex);

    loop {
        // Update states
        let time_new = std::time::SystemTime::now();
        let time_delta = time_new.duration_since(time_last_frame).unwrap();
        let time_since_start = time_new.duration_since(time_start).unwrap();
        time_last_frame = time_new;

        app_state.audio.update();
        app_state.audio.audio_tex.update(&app_state.audio.buffer);

        app_state.midi.update();
        app_state.midi.tex.update(&app_state.midi.buffer);

        app_state
            .post_processing_material
            .set_uniform("iTime", time_since_start.as_secs_f32());
        app_state
            .post_processing_material
            .set_uniform("iTimeDelta", time_delta.as_secs_f32());
        app_state
            .post_processing_material
            .set_uniform("iFrame", frame);

        clear_background(WHITE);

        // Tracks
        for track in &mut app_state.tracks {
            if let Some(anim) = &mut track.animation {
                anim.tick();
            }
            track
                .material
                .set_uniform("iTime", time_since_start.as_secs_f32());
            track
                .material
                .set_uniform("iTimeDelta", time_delta.as_secs_f32());
            track.material.set_uniform("iFrame", frame);
            render_track(track);
        }

        // Post process
        render_post_process(&mut app_state);

        // Tree editor
        if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::Tab) {
            app_state.show_gui = !app_state.show_gui;
        }
        if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::Right) {
            app_state.selected_track =
                std::cmp::min(app_state.selected_track + 1, app_state.tracks.len() - 1);
        }
        if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::Left) {
            if app_state.selected_track != 0 {
                app_state.selected_track = std::cmp::max(app_state.selected_track - 1, 0);
            }
        }
        if app_state.show_gui {
            render_gui(&mut app_state);
        }

        if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::Key1) {
            tree_editor.is_open = !tree_editor.is_open;
        }
        render_tree_editor(&mut tree_editor);

        frame += 1;

        next_frame().await
    }
}
