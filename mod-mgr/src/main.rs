mod mod_table_view;
pub mod settings_view;
pub mod tab_view;
pub mod table;
pub mod view_util;

use clap::Parser;
use floem::{
    id::WindowId,
    peniko::Color,
    reactive::{create_rw_signal, RwSignal},
    style::Style,
    view::View,
    views::{container, container_box, empty, label, stack, tab, text_input, Decorators},
};
use mod_mgr_lib::config::Config;
use mod_table_view::{active_mods, inactive_mods};
use settings_view::settings_view;
use table::table;
use view_util::{save_as_icon, save_icon, simple_form_input, svg_button};

use crate::view_util::{auto_checkbox, form, form_item};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    // TODO:
}

fn main() {
    let args = Args::parse();

    // TODO: load config from a config file
    let config = Config::default();
    let root_view = move || app_view(config.clone());

    floem::launch(root_view)
}

/// Headers/footers
pub const DARK0_BG: Color = Color::rgb8(42, 43, 52);
/// Inputs
pub const DARK1_BG: Color = Color::rgb8(51, 51, 64);
/// Main background
pub const DARK2_BG: Color = Color::rgb8(61, 61, 76);
/// Selected option background
pub const DARK3_BG: Color = Color::rgb8(76, 79, 98);
/// Text in the dark theme
pub const DARK_TEXT: Color = Color::rgb8(209, 209, 212);

fn app_view(config: Config) -> impl View {
    let config = create_rw_signal(config);
    let preferences_cb = move || settings_view(config);

    stack(move || {
        (
            // TODO: menu bar
            // label(move || "Hello World".to_string()).style(|| Style::BASE.padding_px(10.0)),
            // label(move || "Preferences".to_string()).on_click(move |_| {
            //     floem::new_window(WindowId::next(), preferences_cb, None);
            //     true
            // }),
            operation_bar(config.clone()),
            stack(move || {
                (
                    // TODO: Currently if we resize the window small enough then the inactive mods will intersect with the active mods. We should use the response feature or something like it to check for if the screen is small and then just put the inactive mods below.
                    container(move || active_mods(config.clone()))
                        .style(|| Style::BASE.width_pct(50.0)),
                    inactive_mods(config.clone())
                        .style(|| Style::BASE.width_pct(50.0).items_end().justify_end()),
                )
            })
            .style(|| Style::BASE.width_pct(100.0).margin_top_pct(1.0).flex_row()),
        )
    })
    .style(|| {
        Style::BASE
            .size_pct(100.0, 100.0)
            .flex_col()
            .items_start()
            .background(DARK0_BG)
    })
}

/// Top bar with 'profile' and such
fn operation_bar(config: RwSignal<Config>) -> impl View {
    stack(move || {
        // TODO: profile dropdown
        // TODO: mod order dropdown
        (
            svg_button(save_icon(), || todo!()).style(|| Style::BASE.flex_row()),
            svg_button(save_as_icon(), || todo!()).style(|| Style::BASE.flex_row()),
        )
    })
    .style(|| {
        Style::BASE
            .size_pct(100.0, 5.0)
            .min_height_px(32.0)
            .flex_row()
            .background(DARK0_BG)
            .border_bottom(0.8)
            .border_color(Color::SILVER)
    })
}
