use floem::{
    cosmic_text::Weight,
    peniko::Color,
    reactive::RwSignal,
    style::{CursorStyle, Style},
    view::View,
    view_tuple::ViewTuple,
    views::{checkbox, container, label, stack, svg, text_input, Container, Decorators},
};

use crate::{DARK2_BG, DARK3_BG, DARK_TEXT};

pub fn save_icon() -> String {
    include_str!("../assets/document-save-light.svg").to_string()
}

pub fn save_as_icon() -> String {
    include_str!("../assets/document-save-as-light.svg").to_string()
}

pub fn forward_icon() -> String {
    include_str!("../assets/forward-light.svg").to_string()
}

pub fn hard_disk_icon() -> String {
    include_str!("../assets/drive-harddisk-light.svg").to_string()
}

pub fn settings_icon() -> String {
    include_str!("../assets/settings-light.svg").to_string()
}

/// Checkbox that automatically applies the signal on click
pub fn auto_checkbox(signal: RwSignal<bool>) -> impl View {
    checkbox(signal.read_only())
        .focus_visible_style(|| Style::BASE.border_color(Color::BLUE).border(0.5))
        .on_click(move |_| {
            signal.update(|value| *value = !*value);
            true
        })
}

pub fn form<VT: ViewTuple + 'static>(children: impl FnOnce() -> VT) -> impl View {
    stack(children).style(|| {
        Style::BASE
            .flex_col()
            .items_start()
            // .margin_px(10.0)
            .padding_px(10.0)
            .width_pct(100.0)
            .font_size(12.0)
    })
}

pub fn form_item<V: View + 'static>(
    item_label: String,
    label_width: f32,
    view_fn: impl Fn() -> V,
) -> impl View {
    container(|| {
        stack(|| {
            (
                container(|| {
                    label(move || item_label.to_string())
                        .style(|| Style::BASE.font_weight(Weight::BOLD))
                })
                .style(move || {
                    Style::BASE
                        .width_px(label_width)
                        .justify_end()
                        .margin_right_px(10.0)
                }),
                view_fn(),
            )
        })
        .style(|| Style::BASE.flex_row().items_start())
    })
    .style(|| {
        Style::BASE
            .flex_row()
            .items_center()
            .margin_bottom_px(10.0)
            .padding_px(10.0)
            .width_pct(100.0)
            .min_height_px(32.0)
    })
}

// TODO: make border nicer
// TODO: theming
// TODO: tooltip
// TODO: center text correctly
// TODO: that weird background text that I forget the name of
pub fn simple_form_input(
    text: &str,
    signal: RwSignal<String>,
    width: f32,
    height: f32,
) -> impl View {
    form_item(text.to_string(), width, move || {
        // TODO: A bit more padding might be nice.
        text_input(signal)
            .style(move || {
                Style::BASE
                    .border(0.3)
                    .height_px(height)
                    .inset_px(2.0)
                    .border_color(DARK_TEXT)
            })
            .keyboard_navigatable()
    })
}

// TODO: tooltip
/// A simple button view. Intended to look relatively native.  
/// The callback should return true if the event was handled and it should not be propagated further.
pub fn button(text: impl Into<String>, on_click: impl Fn() -> bool + 'static) -> impl View {
    let text = text.into();
    label(move || text.clone())
        .on_click(move |_| on_click())
        .base_style(|| {
            Style::BASE
                .flex_col()
                .padding_horiz_px(6.0)
                .padding_vert_px(1.0)
                .font_size(14.0)
                .background(DARK2_BG)
                .color(DARK_TEXT)
                .justify_center()
                .min_width_px(80.0)
                .border(0.6)
                .border_color(DARK3_BG)
        })
        .hover_style(|| {
            // TODO: change background color?
            Style::BASE.cursor(CursorStyle::Pointer)
        })
}

// TODO: tooltip
pub fn svg_button(svg_v: impl Into<String>, on_click: impl Fn() -> bool + 'static) -> impl View {
    let svg_v = svg_v.into();
    svg(move || svg_v.clone())
        .on_click(move |_| on_click())
        .base_style(|| Style::BASE.min_size_px(32.0, 32.0))
        .hover_style(|| {
            // TODO: change background color?
            Style::BASE.cursor(CursorStyle::Pointer)
        })
}
