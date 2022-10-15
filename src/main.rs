use macroquad::prelude::*;

mod engine;

use engine::app_state::render_gui;
use engine::app_state::render_post_process;
use engine::app_state::AppState;
use engine::track::render_track;
use engine::track::Track;

#[macroquad::main("mrmidi-rs")]
async fn main() {
    let ferris = load_texture("examples/rust.png").await.unwrap();
    let mut app_state = AppState::new();

    let mut track0 = Track::new();
    track0.texture0 = Some(ferris);
    app_state.tracks.push(track0);

    loop {
        clear_background(WHITE);
        for track in &mut app_state.tracks {
            render_track(track);
        }
        render_post_process(&mut app_state);

        if is_key_pressed(KeyCode::Tab) {
            app_state.show_gui = !app_state.show_gui;
        }

        if app_state.show_gui {
            render_gui(&mut app_state);
        }

        next_frame().await
    }
}