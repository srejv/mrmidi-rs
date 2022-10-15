use macroquad::prelude::*;

mod engine;

use engine::app_state::render_gui;
use engine::app_state::render_post_process;
use engine::app_state::AppState;
use engine::track::render_track;
use engine::track::Track;

const NUM_TRACKS : usize = 2;

#[macroquad::main("mrmidi-rs")]
async fn main() {
    let ferris = load_texture("assets/rust.png").await.unwrap();
    let mut app_state = AppState::new(NUM_TRACKS);

    for i in 0..NUM_TRACKS {
        let mut track = Track::new();
        track.texture0 = Some(ferris);
        app_state.tracks.push(track);    
    }
    
    let time_start = std::time::SystemTime::now();
    let mut time_last_frame = std::time::SystemTime::now();
    let mut time_new = std::time::SystemTime::now();

    let mut frame: i32 = 0;
    loop {
        time_new = std::time::SystemTime::now();
        let time_delta = time_new.duration_since(time_last_frame);
        let time_since_start = time_new.duration_since(time_start);

        time_last_frame = time_new;

        app_state
            .post_processing_material
            .set_uniform("iTime", time_since_start.unwrap().as_secs_f32());

        clear_background(WHITE);
        for track in &mut app_state.tracks {
            render_track(track);
        }
        render_post_process(&mut app_state);

        if is_key_pressed(KeyCode::Tab) {
            app_state.show_gui = !app_state.show_gui;
        }

        if is_key_pressed(KeyCode::Right) {
            app_state.selected_track =
                std::cmp::min(app_state.selected_track + 1, app_state.tracks.len() - 1);
        }
        if is_key_pressed(KeyCode::Left) {
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
}
