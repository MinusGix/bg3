use std::{hash::Hash, sync::Arc};

use floem::{
    peniko::Color,
    reactive::RwSignal,
    style::{CursorStyle, Style},
    view::View,
    views::{
        container, container_box, empty, label, list, stack, tab, virtual_list, Decorators, Tab,
        VirtualListDirection, VirtualListItemSize, VirtualListVector,
    },
};

// pub struct TabContainer<V, T>
// where
//     V: View,
//     T: 'static,
// {
//     pub tab: Tab<V, T>,
// }

#[derive(Debug, Clone)]
pub struct TabSwitcherStyle {
    pub separator_background: Color,
    pub button_area_background: Color,
    pub button: TabButtonStyle,
}
impl Default for TabSwitcherStyle {
    fn default() -> Self {
        Self {
            separator_background: Color::WHITE,
            button_area_background: Color::WHITE,
            button: TabButtonStyle::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TabButtonStyle {
    pub font_size: f32,

    pub color: Color,
    pub active_color: Color,

    pub border_width: f32,
    pub border_width_focus_increase: f32,
    pub border_color: Color,

    pub background_color: Color,
    pub active_background_color: Color,

    pub active_padding_top_increase: f32,
}
impl Default for TabButtonStyle {
    fn default() -> Self {
        Self {
            font_size: 16.0,
            color: Color::BLACK,
            active_color: Color::BLACK,
            border_width: 0.8,
            border_width_focus_increase: 0.2,
            border_color: Color::LIGHT_GRAY,
            background_color: Color::WHITE,
            active_background_color: Color::LIGHT_GRAY,
            active_padding_top_increase: 0.2,
        }
    }
}

pub fn horiz_tab_switcher<T, IF, I, KF, K, VF, V>(
    active: RwSignal<usize>,
    each_fn: IF,
    key_fn: KF,
    view_fn: VF,
    TabSwitcherStyle {
        separator_background,
        button_area_background,
        button: button_style,
    }: TabSwitcherStyle,
) -> impl View
where
    T: 'static + ToString + PartialEq,
    IF: Fn() -> I + 'static,
    I: IntoIterator<Item = T> + VirtualListVector<T>,
    KF: Fn(&T) -> K + 'static,
    K: Eq + Hash + 'static,
    VF: Fn(T) -> V + 'static,
    V: View + 'static,
{
    let each_fn = Arc::new(each_fn);
    let key_fn = Arc::new(key_fn);

    stack(move || {
        // TODO: Can we get rid of these stupid clones?
        let each_fn2 = each_fn.clone();
        let each_fn3 = each_fn.clone();
        let key_fn2 = key_fn.clone();
        (
            container(move || {
                list(
                    move || each_fn(),
                    move |x| key_fn(x),
                    move |x| {
                        let each_fn = each_fn3.clone();
                        let x_string = x.to_string();
                        let idx = each_fn().into_iter().position(|y| x == y).unwrap_or(0);
                        let is_active = move || active.get() == idx;
                        tab_button(x_string, is_active, button_style).on_click(move |_| {
                            active.update(move |v| {
                                *v = idx;
                            });

                            true
                        })
                    },
                )
            })
            .style(move || Style::BASE.background(button_area_background)),
            container(|| empty()).style(move || {
                Style::BASE
                    .width_pct(100.0)
                    .height_px(3.0)
                    .background(separator_background)
            }),
            tab(
                move || active.get(),
                move || each_fn2(),
                move |x| key_fn2(x),
                view_fn,
            )
            .style(|| Style::BASE.min_height_px(500.0)),
        )
    })
}

fn tab_button(
    text: String,
    is_active: impl Fn() -> bool + 'static + Copy,
    TabButtonStyle {
        font_size,
        color,
        active_color,
        border_width,
        border_width_focus_increase,
        border_color,
        background_color,
        active_background_color,
        active_padding_top_increase,
    }: TabButtonStyle,
) -> impl View {
    // TODO: different background color for if you're hovering over it? Slight shift in border?
    // TODO: slightly curved outwards border at bottom somehow
    container(move || {
        label(move || text.clone()).style(move || {
            Style::BASE
                .font_size(font_size)
                .padding_horiz_px(10.0)
                .items_center()
                .color(color)
                .apply_if(is_active(), |s| s.color(active_color))
        })
    })
    .keyboard_navigatable()
    .style(move || {
        // TODO: intended padding logic doesn't work
        Style::BASE
            .border(border_width)
            .border_color(border_color)
            .height_px(font_size * 1.2 + active_padding_top_increase * 2.0)
            .padding_horiz_px(3.0)
            .padding_vert_px(2.0)
            .background(background_color)
            .apply_if(is_active(), |s| s.background(active_background_color))
            .apply_if(is_active(), |s| {
                s.padding_top_px(2.0 + active_padding_top_increase * 2.0)
            })
    })
    .focus_visible_style(move || {
        Style::BASE
            .border(border_width + border_width_focus_increase)
            .border_color(Color::BLUE)
            .flex_row()
    })
    .hover_style(|| {
        // TODO: change background color?
        Style::BASE.cursor(CursorStyle::Pointer)
    })
}
