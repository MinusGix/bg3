use std::{hash::Hash, sync::Arc};

use floem::{
    peniko::Color,
    style::Style,
    view::View,
    views::{container, list, stack, Decorators},
};

use crate::{DARK1_BG, DARK2_BG, DARK3_BG};

// TODO: style structure
// TODO: let widths be percentages
// TODO: let us adjust widths. We'd have to precompute all the widths into a vec/hashmap so maybe we should just force them to give it to us in that form?
// TODO: We might want to alert the container that we modified the widths they set.. so they should be signals.

/// Create a table.  
/// `header_fn`: The list of entries in the header.  
/// `header_key_fn`: A way of identifying each entry. This may just be the entry itself.  
/// `header_view_fn`: The actual view that should be displayed. Typically just a label.  
///
/// `widths_fn`: Maps a key to the width of the table column
pub fn table<T, HF, H, WF, KHF, KH, VHF, VH, ROWSF, ROWS, U, ROWKF, ROWK, ROWVF, ROWV>(
    header_fn: HF,
    header_key_fn: KHF,
    header_view_fn: VHF,
    rows_fn: ROWSF,
    row_key_fn: ROWKF,
    row_view_fn: ROWVF,
    widths_fn: WF,
) -> impl View
where
    T: 'static,
    HF: Fn() -> H + 'static,
    H: IntoIterator<Item = T> + 'static,
    WF: Fn(&T) -> f32 + 'static,
    KHF: Fn(&T) -> KH + 'static,
    KH: Eq + Hash + 'static,
    VHF: Fn(T) -> VH + 'static,
    VH: View + 'static,
    U: 'static,
    ROWSF: Fn() -> ROWS + 'static,
    ROWS: IntoIterator<Item = U> + 'static,
    ROWKF: Fn(&U) -> ROWK + 'static,
    ROWK: Eq + Hash + 'static,
    ROWVF: Fn(&T, &U) -> ROWV + 'static + Clone,
    ROWV: View + 'static,
{
    let header_fn = Arc::new(header_fn);
    let header_key_fn = Arc::new(header_key_fn);
    let header_view_fn = Arc::new(header_view_fn);
    let widths_fn = Arc::new(widths_fn);

    stack(move || {
        let header_fn2 = header_fn.clone();
        let header_key_fn2 = header_key_fn.clone();
        let widths_fn2 = widths_fn.clone();
        (
            table_header(
                move || header_fn(),
                move |x| header_key_fn(x),
                move |x| header_view_fn(x),
                move |x| widths_fn(x),
            ),
            table_rows(
                move || header_fn2(),
                move |x| header_key_fn2(x),
                move || rows_fn(),
                move |x| row_key_fn(x),
                move |x, y| row_view_fn(x, y),
                move |x| widths_fn2(x),
            ),
        )
    })
    .base_style(|| Style::BASE.flex_col())
}

fn table_header<T, HF, H, WF, KHF, KH, VHF, VH>(
    header_fn: HF,
    header_key_fn: KHF,
    header_view_fn: VHF,
    widths_fn: WF,
) -> impl View
where
    T: 'static,
    HF: Fn() -> H + 'static,
    H: IntoIterator<Item = T>,
    WF: Fn(&T) -> f32 + 'static,
    KHF: Fn(&T) -> KH + 'static,
    KH: Eq + Hash + 'static,
    VHF: Fn(T) -> VH + 'static,
    VH: View + 'static,
{
    let header_fn = Arc::new(header_fn);
    let header_key_fn = Arc::new(header_key_fn);
    let header_view_fn = Arc::new(header_view_fn);
    let widths_fn = Arc::new(widths_fn);

    list(
        move || header_fn(),
        move |x| header_key_fn(x),
        move |x| {
            let header_view_fn = header_view_fn.clone();
            let width = widths_fn(&x);
            table_header_entry(move |x| header_view_fn(x), x, width)
        },
    )
}

fn table_header_entry<T, VHF, V>(header_view_fn: VHF, x: T, width: f32) -> impl View
where
    T: 'static,
    VHF: Fn(T) -> V + 'static,
    V: View + 'static,
{
    container(move || header_view_fn(x)).style(move || {
        Style::BASE
            .background(DARK1_BG)
            .padding_horiz_px(10.0)
            .padding_vert_px(3.0)
            .border_bottom(0.8)
            .border_right(0.8)
            .border_color(DARK3_BG)
            .width_px(width)
    })
}

fn table_rows<T, HF, H, WF, KHF, KH, ROWSF, ROWS, U, ROWKF, ROWK, ROWVF, ROWV>(
    header_fn: HF,
    header_key_fn: KHF,
    rows_fn: ROWSF,
    row_key_fn: ROWKF,
    row_view_fn: ROWVF,
    widths_fn: WF,
) -> impl View
where
    T: 'static,
    HF: Fn() -> H + 'static + Clone,
    H: IntoIterator<Item = T>,
    WF: Fn(&T) -> f32 + 'static + Clone,
    KHF: Fn(&T) -> KH + 'static + Clone,
    KH: Eq + Hash + 'static,
    U: 'static,
    ROWSF: Fn() -> ROWS + 'static,
    ROWS: IntoIterator<Item = U> + 'static,
    ROWKF: Fn(&U) -> ROWK + 'static,
    ROWK: Eq + Hash + 'static,
    ROWVF: Fn(&T, &U) -> ROWV + 'static + Clone,
    ROWV: View + 'static,
{
    // A list of lists.
    // The outer list is for each row in the table.
    // The inner list is for each column in the table.
    // This seems a bit reversed from how you'd lay it out mentally, but it
    // matches how the header works better.
    list(
        move || rows_fn(),
        move |x| row_key_fn(x),
        move |x: U| {
            let row_view_fn = row_view_fn.clone();
            let header_fn = header_fn.clone();
            let widths_fn = widths_fn.clone();
            let header_key_fn = header_key_fn.clone();
            // TODO(minor): Does this really need a container?
            container(move || {
                let row_view_fn = row_view_fn.clone();
                let widths_fn = widths_fn.clone();
                list(
                    move || header_fn(),
                    move |x: &T| header_key_fn(x),
                    move |y: T| {
                        let row_view_fn = row_view_fn.clone();
                        let widths_fn = widths_fn.clone();
                        let width = widths_fn(&y);
                        table_row_entry(move |x, y| row_view_fn(x, y), &y, &x, width)
                    },
                )
            })
        },
    )
    .base_style(|| Style::BASE.flex_col())
}

fn table_row_entry<T, U, VHF, V>(row_view_fn: VHF, x: &T, y: &U, width: f32) -> impl View
where
    T: 'static,
    U: 'static,
    VHF: Fn(&T, &U) -> V + 'static,
    V: View + 'static,
{
    container(move || row_view_fn(&x, &y)).style(move || {
        Style::BASE
            .background(DARK2_BG)
            .padding_horiz_px(10.0)
            .padding_vert_px(3.0)
            .border_bottom(0.8)
            .border_right(0.8)
            .border_color(DARK3_BG)
            .width_px(width)
    })
}
