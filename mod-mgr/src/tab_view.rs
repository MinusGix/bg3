use std::{hash::Hash, sync::Arc};

use floem::{
    peniko::Color,
    reactive::RwSignal,
    style::{CursorStyle, Style},
    view::View,
    views::{
        container, container_box, label, stack, tab, virtual_list, Decorators, Tab,
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
    pub button: TabButtonStyle,
}
impl Default for TabSwitcherStyle {
    fn default() -> Self {
        Self {
            button: TabButtonStyle::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TabButtonStyle {
    pub border_width: f32,
    pub border_width_focus_increase: f32,
    pub border_color: Color,
}
impl Default for TabButtonStyle {
    fn default() -> Self {
        Self {
            border_width: 1.0,
            border_width_focus_increase: 0.2,
            border_color: Color::LIGHT_GRAY,
        }
    }
}

pub fn horiz_tab_switcher<T, IF, I, KF, K, VF, V>(
    active: RwSignal<usize>,
    each_fn: IF,
    key_fn: KF,
    view_fn: VF,
    TabSwitcherStyle {
        button: button_style,
    }: TabSwitcherStyle,
) -> impl View
where
    T: 'static + ToString + PartialEq + std::fmt::Debug,
    IF: Fn() -> I + 'static,
    I: IntoIterator<Item = T> + VirtualListVector<T>,
    KF: Fn(&T) -> K + 'static,
    K: Eq + Hash + 'static,
    VF: Fn(T) -> V + 'static,
    V: View + 'static,
{
    let each_fn = Arc::new(each_fn);
    let key_fn = Arc::new(key_fn);
    let values = each_fn().into_iter().collect::<Vec<_>>();
    println!("Values: {values:?}");

    stack(move || {
        // TODO: Can we get rid of these stupid clones?
        let each_fn2 = each_fn.clone();
        let each_fn3 = each_fn.clone();
        let key_fn2 = key_fn.clone();
        (
            // container(move || {
            virtual_list(
                VirtualListDirection::Horizontal,
                // TODO
                VirtualListItemSize::Fn(Box::new(|x| 12.0)),
                move || each_fn(),
                move |x| key_fn(x),
                move |x| {
                    let each_fn = each_fn3.clone();
                    tab_button(button_style, x.to_string()).on_click(move |_| {
                        let x = &x;
                        let each_fn = each_fn.clone();

                        // TODO: should we just have a separate function to map idx <-> T??
                        active.update(move |v| {
                            *v = each_fn().into_iter().position(|y| x == &y).unwrap_or(0);
                        });

                        true
                    })
                },
            )
            .style(|| Style::BASE.flex_row()),
            // }),
            tab(
                move || active.get(),
                move || each_fn2(),
                move |x| key_fn2(x),
                view_fn,
            ),
        )
    })
    .style(|| Style::BASE.flex_col())
}

fn tab_button(
    TabButtonStyle {
        border_width,
        border_width_focus_increase,
        border_color,
    }: TabButtonStyle,
    text: String,
) -> impl View {
    // TODO: different background color for active tab?
    container(move || {
        label(move || text.clone()).style(move || {
            Style::BASE
                .font_size(14.0)
                // .flex_col()
                // .width_pct(32.0)
                // .height_pct(32.0)
                .border_color(border_color)
        })
    })
    .keyboard_navigatable()
    .style(move || Style::BASE.border(border_width))
    .focus_visible_style(move || {
        Style::BASE
            .border(border_width + border_width_focus_increase)
            .border_color(Color::BLUE)
    })
    .hover_style(|| {
        // TODO: change background color?
        Style::BASE.cursor(CursorStyle::Pointer)
    })
}
