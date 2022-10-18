use crate::engine::app_state::AppState;

pub fn set_skin(app_state: &mut AppState)
{
    app_state.skin = {
        let label_style = root_ui()
            .style_builder()
            .font(include_bytes!("../assets/ui/MinimalPixel v2.ttf"))
            .unwrap()
            .text_color(Color::from_rgba(120, 120, 120, 255))
            .font_size(20)
            .build();

        let window_style = root_ui()
            .style_builder()
            .background(Image::from_file_with_format(
                include_bytes!("../assets/ui/window_background_2.png"),
                None,
            ))
            .background_margin(RectOffset::new(52.0, 52.0, 52.0, 52.0))
            .margin(RectOffset::new(100.0, 100.0, 100.0, 100.0))
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
            .font_size(20)
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
            .font_size(20)
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
            .font_size(20)
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
}