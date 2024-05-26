use eframe::egui::style::{HandleShape, NumericColorSpace, Selection, Widgets};
use eframe::egui::*;
use eframe::epaint::Shadow;

pub fn dark() -> Visuals {
    Visuals {
        dark_mode: true,
        override_text_color: None,
        widgets: Widgets::default(),
        selection: Selection::default(),
        hyperlink_color: Color32::from_rgb(90, 170, 255),
        faint_bg_color: Color32::from_additive_luminance(5), // visible, but barely so
        extreme_bg_color: Color32::from_gray(10), // e.g. TextEdit background
        code_bg_color: Color32::from_gray(64),
        warn_fg_color: Color32::from_rgb(255, 143, 0), // orange
        error_fg_color: Color32::from_rgb(255, 0, 0),  // red

        window_rounding: Rounding::same(6.0),
        window_shadow: Shadow {
            offset: vec2(10.0, 20.0),
            blur: 15.0,
            spread: 0.0,
            color: Color32::from_black_alpha(96),
        },
        window_fill: Color32::from_gray(27),
        window_stroke: Stroke::new(1.0, Color32::from_gray(60)),
        window_highlight_topmost: true,

        menu_rounding: Rounding::same(6.0),

        panel_fill: Color32::from_gray(27),

        popup_shadow: Shadow {
            offset: vec2(6.0, 10.0),
            blur: 8.0,
            spread: 0.0,
            color: Color32::from_black_alpha(96),
        },

        resize_corner_size: 12.0,

        text_cursor: Stroke::new(2.0, Color32::from_rgb(192, 222, 255)),
        text_cursor_preview: false,

        clip_rect_margin: 3.0, // should be at least half the size of the widest frame stroke + max WidgetVisuals::expansion
        button_frame: true,
        collapsing_header_frame: false,
        indent_has_left_vline: true,

        striped: false,

        slider_trailing_fill: false,
        handle_shape: HandleShape::Circle,

        interact_cursor: None,

        image_loading_spinners: true,

        numeric_color_space: NumericColorSpace::GammaByte,
    }
}
