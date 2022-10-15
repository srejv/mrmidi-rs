use macroquad::prelude::*;

mod engine;

use engine::app_state::render_gui;
use engine::app_state::render_post_process;
use engine::app_state::AppState;
use engine::track::render_track;
use engine::track::Track;

const NUM_TRACKS: usize = 2;

use macroquad::ui::{hash, root_ui, widgets, Skin};

#[macroquad::main("mrmidi-rs")]
async fn main() {
    let ferris = load_texture("assets/rust.png").await.unwrap();
    let mut app_state = AppState::new(NUM_TRACKS);

    for i in 0..NUM_TRACKS {
        let mut track = Track::new(format!("iChannel{}", i));
        track.texture0 = Some(ferris);
        app_state.tracks.push(track);
    }

    let time_start = std::time::SystemTime::now();
    let mut time_last_frame = std::time::SystemTime::now();
    let mut frame: i32 = 0;

    let w = macroquad::window::screen_width();
    let h = macroquad::window::screen_height();
    let aspectRatio = w / h;
    app_state
        .post_processing_material
        .set_uniform("iResolution", (w, h, aspectRatio));

    app_state.skin = {
        let label_style = root_ui()
            .style_builder()
            .font(include_bytes!("../assets/ui/MinimalPixel v2.ttf"))
            .unwrap()
            .text_color(Color::from_rgba(120, 120, 120, 255))
            .font_size(25)
            .build();

        let window_style = root_ui()
            .style_builder()
            .background(Image::from_file_with_format(
                include_bytes!("../assets/ui/window_background_2.png"),
                None,
            ))
            .background_margin(RectOffset::new(52.0, 52.0, 52.0, 52.0))
            .margin(RectOffset::new(-30.0, 0.0, -30.0, 0.0))
            .build();

        let button_style = root_ui()
            .style_builder()
            .background(Image::from_file_with_format(
                include_bytes!("../assets/ui/button_background_2.png"),
                None,
            ))
            .background_margin(RectOffset::new(8.0, 8.0, 8.0, 8.0))
            .background_hovered(Image::from_file_with_format(
                include_bytes!("../assets/ui/button_hovered_background_2.png"),
                None,
            ))
            .background_clicked(Image::from_file_with_format(
                include_bytes!("../assets/ui/button_clicked_background_2.png"),
                None,
            ))
            .font(include_bytes!("../assets/ui/MinimalPixel v2.ttf"))
            .unwrap()
            .text_color(Color::from_rgba(180, 180, 100, 255))
            .font_size(40)
            .build();

        let checkbox_style = root_ui()
            .style_builder()
            .background(Image::from_file_with_format(
                include_bytes!("../assets/ui/checkbox_background.png"),
                None,
            ))
            .background_hovered(Image::from_file_with_format(
                include_bytes!("../assets/ui/checkbox_hovered_background.png"),
                None,
            ))
            .background_clicked(Image::from_file_with_format(
                include_bytes!("../assets/ui/checkbox_clicked_background.png"),
                None,
            ))
            .build();

        let editbox_style = root_ui()
            .style_builder()
            .background(Image::from_file_with_format(
                include_bytes!("../assets/ui/editbox_background.png"),
                None,
            ))
            .background_margin(RectOffset::new(2., 2., 2., 2.))
            .font(include_bytes!("../assets/ui/MinimalPixel v2.ttf"))
            .unwrap()
            .text_color(Color::from_rgba(120, 120, 120, 255))
            .font_size(25)
            .build();

        let combobox_style = root_ui()
            .style_builder()
            .background(Image::from_file_with_format(
                include_bytes!("../assets/ui/combobox_background.png"),
                None,
            ))
            .background_margin(RectOffset::new(4., 25., 6., 6.))
            .font(include_bytes!("../assets/ui/MinimalPixel v2.ttf"))
            .unwrap()
            .text_color(Color::from_rgba(120, 120, 120, 255))
            .color(Color::from_rgba(210, 210, 210, 255))
            .font_size(25)
            .build();

        Skin {
            window_style,
            button_style,
            label_style,
            checkbox_style,
            editbox_style,
            combobox_style,
            ..root_ui().default_skin()
        }
    };

    loop {
        let time_new = std::time::SystemTime::now();
        let time_delta = time_new.duration_since(time_last_frame).unwrap();
        let time_since_start = time_new.duration_since(time_start).unwrap();
        time_last_frame = time_new;

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
