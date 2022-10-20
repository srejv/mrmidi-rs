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

    ui.label(None, &command_name);
    ui.label(None, &name);

    match program.tree.arena[node_idx].val.0 {
        crate::code::commands::CommandType::Multiply => {}
        crate::code::commands::CommandType::Add => {}
        crate::code::commands::CommandType::Divide => {}
        crate::code::commands::CommandType::Subtract => {}
        crate::code::commands::CommandType::SetVariable => {}

        _ => {}
    }
}

#[macroquad::main("mrmidi-r(eturn)s")]
async fn main() {
    // engine::audio::print_audio_device_status();
    crate::code::generator::test_program();
    let mut program = crate::code::generator::create_crt_fragment_shader();

    let mut selected_row = 0;
    let mut selected_command = &program.tree.arena[0].val;
    let mut selected_column = 0;

    let mut selectedNode: usize = 0;
    loop {
        set_default_camera();

        if is_key_pressed(KeyCode::Up) {
            if program.tree.arena[selectedNode].children.len() == 0 {
            } else if selected_row > 0 {
                selected_row -= 1;
            }
        }
        if is_key_pressed(KeyCode::Down) {
            if program.tree.arena[selectedNode].children.len() == 0 {
            } else if selected_row < program.tree.arena[selectedNode].children.len() - 1 {
                selected_row += 1;
            }
        }

        if is_key_pressed(KeyCode::Left) {
            if let Some(parentIdx) = program.tree.arena[selectedNode].parent {
                selectedNode = parentIdx;
                selected_row = 0;
            }
        }
        if is_key_pressed(KeyCode::Right) {
            if program.tree.arena[selectedNode].children.len() == 0 {
            } else if selected_row < program.tree.arena[selectedNode].children.len() - 1 {
                selectedNode = program.tree.arena[selectedNode].children[selected_row];
                selected_row = 0;
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

            render_command(ui, &mut program, selectedNode, 0);

            ui.label(None, "Commands List");

            ui.separator();

            let mut i = 0;
            let root = &program.tree.arena[selectedNode];
            for childIdx in &root.children {
                let child = &program.tree.arena[*childIdx];
                let name = crate::code::commands::to_strang(&child, &program.tree);
                let command_name = crate::code::commands::type_to_strang(&child);
                let selected = if i == selected_row { "->" } else { "" };
                let label = format!("{}. {}{}: {}", i + 1, selected, command_name, name);
                ui.label(None, &label);
                i += 1;
            }
        });

        next_frame().await
    }
    /*
        return;

        let ferris = load_texture("assets/rust.png").await.unwrap();

        let mut tracks = Vec::new();
        for i in 0..NUM_TRACKS {
            let mut track = Track::new(format!("iChannel{}", i));
            track.texture0 = Some(ferris);
            tracks.push(track);
        }
        let mut app_state = AppState::new(tracks);

        let time_start = std::time::SystemTime::now();
        let mut time_last_frame = std::time::SystemTime::now();
        let mut frame: i32 = 0;

        let w = macroquad::window::screen_width();
        let h = macroquad::window::screen_height();
        let aspect_ratio = w / h;
        app_state
            .post_processing_material
            .set_uniform("iResolution", (w, h, aspect_ratio));

        crate::engine::audio::begin_audio(&mut app_state.audio);

        loop {
            let time_new = std::time::SystemTime::now();
            let time_delta = time_new.duration_since(time_last_frame).unwrap();
            let time_since_start = time_new.duration_since(time_start).unwrap();
            time_last_frame = time_new;

            crate::engine::audio::update_buffer(&mut app_state.audio);

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
            for track in &mut app_state.tracks {
                track
                    .material
                    .set_uniform("iTime", time_since_start.as_secs_f32());
                track
                    .material
                    .set_uniform("iTimeDelta", time_delta.as_secs_f32());
                track.material.set_uniform("iFrame", frame);
                render_track(track);
            }
            render_post_process(&mut app_state);

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

            frame += 1;
            next_frame().await
        }
    */
}
