use std::{fmt::Display, path::PathBuf};

use floem::{
    peniko::Color,
    reactive::{create_rw_signal, RwSignal},
    style::{AlignContent, CursorStyle, Style},
    view::View,
    views::{
        container, container_box, label, scroll, stack, svg, tab, text_input, Decorators,
        VirtualListVector,
    },
};
use mod_mgr_lib::config::Config;

use crate::{
    tab_view::{self, TabButtonStyle, TabSwitcherStyle},
    view_util::{auto_checkbox, button, form, form_item, save_icon, simple_form_input, svg_button},
    MainData, DARK0_BG, DARK2_BG, DARK3_BG, DARK_TEXT,
};

fn save_config(
    config: RwSignal<Config>,
    general: GeneralSettingData,
    keyboard: KeyboardSettingData,
) {
    config.update(|config| {
        // TODO: can path buf conversion panic if it is bad?
        config.game_data_path = PathBuf::from(general.game_data_path.get_untracked());
        config.game_executable_path = PathBuf::from(general.game_executable_path.get_untracked());
        config.saved_load_orders_path =
            PathBuf::from(general.saved_load_orders_path.get_untracked());
        config.enable_story_log = general.enable_story_log.get_untracked();
        config.auto_add_missing_dependencies_on_export =
            general.auto_add_missing_deps.get_untracked();
        config.show_missing_mod_warnings = !general.disable_missing_mod_warnings.get_untracked();
        config.shift_focus_on_swap = general.shift_focus_on_swap.get_untracked();
        config.save_window_location = general.save_window_location.get_untracked();
        config.enable_dx11_mode = general.enable_dx11_mode.get_untracked();
        config.skip_launcher = general.skip_launcher.get_untracked();

        // TODO: keyboard shortcuts

        if let Err(err) = config.save() {
            eprintln!("Failed to save the config: {:?}", err);
        }
    });
}

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

pub fn settings_view(main_data: MainData) -> impl View {
    let config = main_data.config.clone();

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

    stack(move || {
        let header = settings_view_header(config, general.clone(), keyboard.clone());
        let footer = settings_view_footer(config, general.clone(), keyboard.clone());
        (
            header,
            tab_view::horiz_tab_switcher(
                active_tab,
                move || tabs.clone(),
                |it| *it,
                move |it| match it {
                    SettingTab::General => {
                        container_box(|| Box::new(general_settings_view(general.clone())))
                            .style(|| Style::BASE.size_pct(100.0, 100.0))
                    }
                    SettingTab::KeyboardShortcuts => {
                        container_box(|| Box::new(keyboard_settings_view(keyboard.clone())))
                    } // SettingTab::Advanced => todo!(),
                },
                TabSwitcherStyle {
                    separator_background: DARK3_BG,
                    button_area_background: DARK0_BG,
                    button: TabButtonStyle {
                        background_color: DARK2_BG,
                        active_background_color: DARK3_BG,
                        color: DARK_TEXT,
                        active_color: DARK_TEXT,
                        border_color: DARK3_BG,
                        ..Default::default()
                    },
                },
            )
            .style(|| {
                Style::BASE
                    .flex_col()
                    .border(1.0)
                    .border_color(DARK3_BG)
                    .background(DARK2_BG)
                    .color(DARK_TEXT)
                    .width_pct(100.0)
                    .height_pct(80.0)
                // .max_height_pct(95.0)
                // .min_height_pct(80.0)
            }),
            footer,
        )
    })
    .style(|| Style::BASE.size_pct(100.0, 100.0).flex_col())
    // .style(|| Style::BASE.width_pct(100.0).height_pct(100.0))
    // TODO: save button in footer
    // TODO: close button at top?
}

fn settings_view_header(
    config: RwSignal<Config>,
    general: GeneralSettingData,
    keyboard: KeyboardSettingData,
) -> impl View {
    stack(|| {
        // TODO: There's another button in the view. What does it do?
        (svg_button(save_icon(), move || {
            // TODO: show that it has been saving by flashing the button or showing a checkmark or something.
            save_config(config, general.clone(), keyboard.clone());

            true
        }),)
    })
    .style(|| {
        Style::BASE
            .size_pct(100.0, 10.0)
            // .min_height_px(32.0)
            // .max_height_px(60.0)
            .width_pct(100.0)
            .background(DARK0_BG)
            .items_start()
            .flex_col()
    })
}

fn settings_view_footer(
    config: RwSignal<Config>,
    general: GeneralSettingData,
    keyboard: KeyboardSettingData,
) -> impl View {
    container(|| {
        // TODO: decent button view?
        button("Save", move || {
            save_config(config, general.clone(), keyboard.clone());

            true
        })
        .style(|| Style::BASE.margin_right_pct(20.0).margin_top_pct(2.0))
    })
    .style(|| {
        Style::BASE
            .size_pct(100.0, 10.0)
            // .min_height_px(40.0)
            // .max_height_px(60.0)
            .background(DARK0_BG)
            .items_end()
            .flex_col()
    })
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

    let inp = |x, y| simple_form_input(x, y, LABEL_WIDTH, 24.0);

    container(|| {
        scroll(|| {
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
        })
        .style(|| Style::BASE.size_pct(100.0, 100.0))
    })
    .style(|| Style::BASE.size_pct(100.0, 100.0))
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
