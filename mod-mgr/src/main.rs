pub mod settings_view;
pub mod tab_view;
pub mod view_util;

use clap::Parser;
use floem::{
    id::WindowId,
    peniko::Color,
    reactive::{create_rw_signal, RwSignal},
    style::Style,
    view::View,
    views::{container, container_box, label, stack, tab, text_input, Decorators},
};
use mod_mgr_lib::config::Config;
use settings_view::settings_view;

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

// TODO: light theme

fn app_view(config: Config) -> impl View {
    let config = create_rw_signal(config);
    let preferences_cb = move || settings_view(config);

    stack(move || {
        (
            label(move || "Hello World".to_string()).style(|| Style::BASE.padding_px(10.0)),
            label(move || "Preferences".to_string()).on_click(move |_| {
                floem::new_window(WindowId::next(), preferences_cb, None);
                true
            }),
        )
    })
    .style(|| {
        Style::BASE
            .size_pct(100.0, 100.0)
            .flex_col()
            .items_center()
            .justify_center()
    })
}
