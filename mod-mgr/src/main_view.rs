//! https://github.com/LaughingLeader/BG3ModManager/blob/master/GUI/ViewModels/MainWindowViewModel.cs

use std::path::PathBuf;

use floem::{
    cosmic_text::Weight,
    glazier::FileDialogOptions,
    id::WindowId,
    peniko::Color,
    reactive::{create_effect, create_rw_signal, RwSignal},
    style::Style,
    view::View,
    views::{container, container_box, label, stack, Decorators},
    ViewContext,
};
use mod_mgr_lib::settings::Settings;

use crate::{
    mod_table_view::{active_mods, inactive_mods},
    settings_view::settings_view,
    view_util::{
        forward_icon, hard_disk_icon, save_as_icon, save_icon, settings_icon, simple_form_input,
        svg_button,
    },
    GameDataInfo, MainData, DARK0_BG, DARK2_BG, DARK_TEXT,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartupStage {
    Loading,
    AskGamePath,
    Ready,
}

pub fn app_view(main_data: MainData) -> impl View {
    let startup_stage = main_data.startup_stage.clone();

    // TODO: container_box is the more natural method of implementing this,
    // however there's an open floem issue that means it doesn't update on signals properly.
    // container_box(move || {
    //     let startup_stage = startup_stage.get();
    //     println!("ContainerBox: {startup_stage:?}");
    //     match startup_stage {
    //         StartupStage::Loading => Box::new(loading_view()),
    //         StartupStage::AskGamePath => Box::new(ask_game_path_view(main_data)),
    //         StartupStage::Ready => Box::new(main_view(main_data)),
    //     }
    // })
    // .style(|| Style::BASE.size_pct(100.0, 100.0))

    // TODO: we could animate transitions between the views, like a sliding pane.
    // Instead of container_box we hide the elements that aren't in the appropriate stage since
    // their style callbacks will still work.
    stack(move || {
        (
            loading_view().style(move || {
                Style::BASE.apply_if(startup_stage.get() != StartupStage::Loading, |s| s.hide())
            }),
            ask_game_path_view(main_data.clone()).style(move || {
                Style::BASE
                    .size_pct(100., 100.)
                    .apply_if(startup_stage.get() != StartupStage::AskGamePath, |s| {
                        s.hide()
                    })
            }),
            main_view(main_data.clone()).style(move || {
                Style::BASE
                    .size_pct(100., 100.)
                    .apply_if(startup_stage.get() != StartupStage::Ready, |s| s.hide())
            }),
        )
    })
    .style(|| Style::BASE.size_pct(100.0, 100.0))
}

fn loading_view() -> impl View {
    container(|| {
        label(|| "Loading...".to_string()).style(|| {
            Style::BASE
                .font_size(20.0)
                .font_weight(Weight::BOLD)
                .justify_center()
        })
    })
    .base_style(|| {
        Style::BASE
            .size_pct(100.0, 100.0)
            .flex_col()
            .items_center()
            .justify_center()
            .background(DARK0_BG)
    })
}

fn ask_game_path_view(main_data: MainData) -> impl View {
    let dry = main_data.dry;
    let game_path = create_rw_signal(String::new());
    let startup_stage = main_data.startup_stage.clone();
    let game_data_info: RwSignal<Option<GameDataInfo>> = create_rw_signal(None);
    let settings = main_data.settings.clone();
    let pathway = main_data.pathway.clone();

    // Whenever the game path changes, update game data info.
    create_effect(move |prev_game_path| {
        let game_path = game_path.get();
        if Some(&game_path) != prev_game_path.as_ref() {
            // TODO: it would be preferable to do this on a separate thread or something.
            // Because this could delay typing.
            let path = PathBuf::from(&game_path);
            game_data_info.set(Some(GameDataInfo::from_path(&path)));
        }

        game_path
    });

    // TODO: provide guidance about where this is typically located.
    // TODO: provide an example path to ensure they know what we mean
    stack(move || {
        (
            label(|| "Set the path to your Baldur's Gate 3 root installation folder".to_string())
                .style(|| {
                    Style::BASE
                        .font_size(20.0)
                        .font_weight(Weight::BOLD)
                        .color(DARK_TEXT)
                }),
            // Path input and file open button
            stack(|| {
                (
                    simple_form_input("Path:", game_path, 300.0, 24.0)
                        .style(|| Style::BASE.font_size(20.0).color(DARK_TEXT)),
                    svg_button(hard_disk_icon(), move || {
                        // TODO(minor): Set starting path to the current settings path / current guessed path?
                        let options = FileDialogOptions::new()
                            // just in case their steam folder is hidden, like it is commonly named
                            // .steam on Linux
                            .show_hidden()
                            .select_directories();

                        floem::action::open_file(options, move |file_info| {
                            if let Some(file_info) = file_info {
                                let path = file_info.path;

                                game_path.set(path.to_string_lossy().to_string());
                            } else {
                                eprintln!("User did not choose file.");
                            }
                        });

                        true
                    })
                    .style(|| Style::BASE),
                )
            })
            .style(|| {
                Style::BASE
                    // Shift it so that the input box is closer to the center of the top label
                    .margin_right_px(160.0)
            }),
            // Move forward arrow
            container(move || {
                svg_button(forward_icon(), move || {
                    startup_stage.set(StartupStage::Ready);

                    if let Some(game_data_info) = game_data_info.get_untracked() {
                        if dry {
                            eprintln!("GameDataInfo: {game_data_info:#?}");
                            return true;
                        }

                        // TODO: This triggers an effect run regardless of whether we've changed anything..
                        settings.update(|settings| {
                            if let Some(game_data_path) = game_data_info.game_data_path {
                                settings.game_data_path = game_data_path;
                            }

                            if let Some(game_executable_path) = game_data_info.game_exe_path {
                                settings.game_executable_path = game_executable_path;
                            }
                        });

                        pathway.update(|pathway| {
                            // TODO: none of pathways other fields get modified?
                            pathway.install_path = game_data_info.install_path;
                        });
                    } else {
                        // TODO: should we ask the user if they're sure?
                        eprintln!("User moved to next stage without providing game data path");
                    }

                    true
                })
            })
            .style(|| {
                Style::BASE
                    .items_end()
                    .justify_end()
                    .margin_right_px(32.0)
                    .width_pct(100.0)
            }),
        )
    })
    .base_style(|| {
        Style::BASE
            .size_pct(100.0, 100.0)
            .flex_col()
            .items_center()
            .justify_center()
            .background(DARK0_BG)
            .border(1.0)
            .border_color(Color::GREEN)
    })
}

fn main_view(main_data: MainData) -> impl View {
    let settings = main_data.settings.clone();

    stack(move || {
        (
            operation_bar(main_data.clone()),
            stack(move || {
                (
                    // TODO: Currently if we resize the window small enough then the inactive mods will intersect with the active mods. We should use the response feature or something like it to check for if the screen is small and then just put the inactive mods below.
                    active_mods(settings.clone()).style(|| Style::BASE.width_pct(50.0)),
                    inactive_mods(settings.clone())
                        .style(|| Style::BASE.width_pct(50.0).items_end().justify_end()),
                )
            })
            .style(|| Style::BASE.width_pct(100.0).margin_top_pct(1.0).flex_row()),
        )
    })
    .base_style(|| {
        Style::BASE
            .size_pct(100.0, 100.0)
            .flex_col()
            .items_start()
            .background(DARK0_BG)
    })
}

/// Top bar with 'profile' and such
fn operation_bar(main_data: MainData) -> impl View {
    stack(move || {
        // TODO: profile dropdown
        // TODO: mod order dropdown
        (
            // left side
            stack(|| {
                (
                    svg_button(save_icon(), || todo!()).style(|| Style::BASE.flex_row()),
                    svg_button(save_as_icon(), || todo!()).style(|| Style::BASE.flex_row()),
                )
            })
            .style(|| {
                Style::BASE
                    .size_pct(50.0, 100.0)
                    .items_start()
                    .justify_start()
            }),
            stack(|| {
                (
                    // Settings button. Opens settings window.
                    svg_button(settings_icon(), move || {
                        // TODO: should we pause actions in the main display while settings is open?
                        // TODO: if the main window is closed, close the settings window
                        // TODO: should be floating window
                        let main_data = main_data.clone();
                        floem::new_window(
                            WindowId::next(),
                            move || settings_view(main_data.clone()),
                            None,
                        );

                        true
                    }),
                )
            })
            .style(|| {
                Style::BASE
                    .size_pct(50.0, 100.0)
                    .flex_row()
                    .items_end()
                    .justify_end()
            }),
        )
    })
    .base_style(|| {
        Style::BASE
            .size_pct(100.0, 5.0)
            .min_height_px(32.0)
            .flex_row()
            .background(DARK0_BG)
            .border_bottom(0.8)
            .border_color(Color::SILVER)
    })
}
