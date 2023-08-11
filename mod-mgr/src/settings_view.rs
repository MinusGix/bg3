use std::fmt::Display;

use floem::{
    reactive::{create_rw_signal, RwSignal},
    style::{AlignContent, Style},
    view::View,
    views::{container, container_box, label, tab, text_input, Decorators, VirtualListVector},
};
use mod_mgr_lib::config::Config;

use crate::{
    tab_view,
    view_util::{auto_checkbox, form, form_item},
};

// TODO: do we need to store the field stuff on this so that we can restore it when you swap around the tabs? We could also make structs of rw signals to avoid that bother
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
enum SettingTab {
    General = 0,
    KeyboardShortcuts = 1,
    // Advanced = 2,
}
impl SettingTab {
    fn title(&self) -> String {
        match self {
            SettingTab::General => "General".to_string(),
            SettingTab::KeyboardShortcuts => "Keyboard Shortcuts".to_string(),
            // SettingTab::Advanced => "Advanced".to_string(),
        }
    }
}
impl Display for SettingTab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.title().fmt(f)
    }
}

pub fn settings_view(config: RwSignal<Config>) -> impl View {
    let tabs = im::Vector::from_iter(
        [
            SettingTab::General,
            SettingTab::KeyboardShortcuts,
            // SettingTab::Advanced,
        ]
        .into_iter(),
    );
    let active_tab = create_rw_signal(0);
    let general = config.with(GeneralSettingData::from_config);
    let keyboard = config.with(KeyboardSettingData::from_config);
    tab_view::horiz_tab_switcher(
        active_tab,
        move || tabs.clone(),
        |it| *it,
        move |it| match it {
            SettingTab::General => {
                container_box(|| Box::new(general_settings_view(general.clone())))
            }
            SettingTab::KeyboardShortcuts => {
                container_box(|| Box::new(keyboard_settings_view(keyboard.clone())))
            } // SettingTab::Advanced => todo!(),
        },
        Default::default(),
    )

    // TODO: save button in footer
    // TODO: close button at top?
}

/// Data for the 'General' setting tab.  
#[derive(Debug, Clone)]
struct GeneralSettingData {
    game_data_path: RwSignal<String>,
    game_executable_path: RwSignal<String>,
    saved_load_orders_path: RwSignal<String>,
    enable_story_log: RwSignal<bool>,
    auto_add_missing_deps: RwSignal<bool>,
    disable_missing_mod_warnings: RwSignal<bool>,
    shift_focus_on_swap: RwSignal<bool>,
    save_window_location: RwSignal<bool>,
    enable_dx11_mode: RwSignal<bool>,
    skip_launcher: RwSignal<bool>,
}
impl GeneralSettingData {
    fn from_config(config: &Config) -> GeneralSettingData {
        GeneralSettingData {
            game_data_path: create_rw_signal(config.game_data_path_str().to_string()),
            game_executable_path: create_rw_signal(config.game_executable_path_str().to_string()),
            saved_load_orders_path: create_rw_signal(
                config.saved_load_orders_path_str().to_string(),
            ),
            enable_story_log: create_rw_signal(config.enable_story_log),
            auto_add_missing_deps: create_rw_signal(config.auto_add_missing_dependencies_on_export),
            disable_missing_mod_warnings: create_rw_signal(!config.show_missing_mod_warnings),
            shift_focus_on_swap: create_rw_signal(config.shift_focus_on_swap),
            save_window_location: create_rw_signal(config.save_window_location),
            enable_dx11_mode: create_rw_signal(config.enable_dx11_mode),
            skip_launcher: create_rw_signal(config.skip_launcher),
        }
    }
}

fn general_settings_view(g: GeneralSettingData) -> impl View {
    const LABEL_WIDTH: f32 = 400.0;
    fn chk(text: &str, signal: RwSignal<bool>) -> impl View {
        form_item(text.to_string(), LABEL_WIDTH, move || auto_checkbox(signal))
    }

    fn inp(text: &str, signal: RwSignal<String>) -> impl View {
        form_item(text.to_string(), LABEL_WIDTH, move || {
            text_input(signal)
                .style(|| Style::BASE.border(0.5).height_px(24.0))
                .keyboard_navigatable()
        })
    }

    form(move || {
        (
            // TODO: let the user open a file dialog for quickly choosing a folder.
            // TODO: show an icon next to the path marking whether it actually exists
            // TODO: tooltips
            inp("Game Data Path", g.game_data_path),
            inp("Game Executable Path", g.game_executable_path),
            inp("Saved Load Orders Path", g.saved_load_orders_path),
            chk("Enable Story Log", g.enable_story_log),
            chk(
                "Auto Add Missing Dependencies When Exporting",
                g.auto_add_missing_deps,
            ),
            chk(
                "Disable Missing Mod Warnings",
                g.disable_missing_mod_warnings,
            ),
            chk("Shift Focus on Swap", g.shift_focus_on_swap),
            chk("Save Window Location", g.save_window_location),
            chk("Enable DirectX 11 Mode", g.enable_dx11_mode),
            chk("Skip Launcher", g.skip_launcher),
        )
    })
}

#[derive(Debug, Clone)]
struct KeyboardSettingData {}
impl KeyboardSettingData {
    fn from_config(config: &Config) -> KeyboardSettingData {
        KeyboardSettingData {}
    }
}
fn keyboard_settings_view(g: KeyboardSettingData) -> impl View {
    container(|| label(|| "TODO: Nothing here yet :)".to_string()))
        .style(|| Style::BASE.padding_px(10.0).items_center().justify_center())
}
