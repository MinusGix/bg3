use floem::{
    cosmic_text::Weight,
    peniko::Color,
    reactive::RwSignal,
    style::Style,
    view::View,
    view_tuple::ViewTuple,
    views::{checkbox, container, label, stack, Decorators, VirtualListVector},
};

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
            .margin_px(10.0)
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
