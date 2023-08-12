// TODO: light theme, it would basically just be a matter of threading the style info around

use floem::{
    reactive::{create_rw_signal, RwSignal},
    style::Style,
    view::View,
    views::{container, empty, label, stack, Decorators},
};
use mod_mgr_lib::config::Config;

use crate::{table::table, view_util::simple_form_input, DARK0_BG, DARK_TEXT};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ModTableEntry {
    /// Load order index
    Index,
    Name,
    Version,
    Author,
    LastUpdated,
    // TODO: Should we have some inbuilt way in table to have a blank space? Eh.
    Blank,
}
impl ModTableEntry {
    fn title(&self) -> &'static str {
        match self {
            Self::Index => "#",
            Self::Name => "Name",
            Self::Version => "Version",
            Self::Author => "Author",
            Self::LastUpdated => "Last Updated",
            Self::Blank => "",
        }
    }
}

const ACTIVE_MOD_TABLE_ENTRIES: [ModTableEntry; 6] = [
    ModTableEntry::Index,
    ModTableEntry::Name,
    ModTableEntry::Version,
    ModTableEntry::Author,
    ModTableEntry::LastUpdated,
    ModTableEntry::Blank,
];

#[derive(Debug, Clone)]
struct ModRow {
    pub name: String,
    pub version: String,
    pub author: String,
    pub last_updated: String,
}
impl ModRow {
    fn value(&self, idx: usize, entry: ModTableEntry) -> String {
        match entry {
            ModTableEntry::Index => idx.to_string(),
            ModTableEntry::Name => self.name.clone(),
            ModTableEntry::Version => self.version.clone(),
            ModTableEntry::Author => self.author.clone(),
            ModTableEntry::LastUpdated => self.last_updated.clone(),
            ModTableEntry::Blank => String::new(),
        }
    }
}

// TODO: searching
pub fn active_mods(config: RwSignal<Config>) -> impl View {
    let mods_filter = create_rw_signal(String::new());
    // TODO: I think it optionally has more fields you can show
    let rows = im::Vector::from_iter(
        [
            ModRow {
                name: "DndRebalancing".to_string(),
                version: "1.0.0.0".to_string(),
                author: "Zerd".to_string(),
                last_updated: "11/5/2020".to_string(),
            },
            ModRow {
                name: "8 More Short Rests".to_string(),
                version: "-1.15".to_string(),
                author: "Logos".to_string(),
                last_updated: "11/5/2020".to_string(),
            },
            ModRow {
                name: "Customizer".to_string(),
                version: "1.0.0.0".to_string(),
                author: "AlanaSP".to_string(),
                last_updated: "12/1/2020".to_string(),
            },
        ]
        .into_iter()
        .enumerate(),
    );
    stack(|| {
        (
            container(|| {
                // TODO: background text
                simple_form_input("Active Mods", mods_filter, 400.0, 14.0)
                    .style(|| Style::BASE.font_size(12.0).flex_col())
            })
            .style(|| {
                Style::BASE
                    .width_pct(100.0)
                    .flex_col()
                    .margin_bottom_px(10.0)
                    // .items_start()
                    .background(DARK0_BG)
                    .color(DARK_TEXT)
            }),
            table(
                move || ACTIVE_MOD_TABLE_ENTRIES,
                Clone::clone,
                mod_table_text,
                move || rows.clone(),
                |(idx, _)| *idx,
                mod_entry_text,
                mod_table_entry_sizes,
            )
            .style(|| Style::BASE.width_pct(100.0)),
        )
    })
    .style(|| Style::BASE.flex_col())
}

const INACTIVE_MOD_TABLE_ENTRIES: [ModTableEntry; 5] = [
    ModTableEntry::Name,
    ModTableEntry::Version,
    ModTableEntry::Author,
    ModTableEntry::LastUpdated,
    ModTableEntry::Blank,
];

pub fn inactive_mods(config: RwSignal<Config>) -> impl View {
    // TODO: searching
    // table(
    //     move || INACTIVE_MOD_TABLE_ENTRIES,
    //     Clone::clone,
    //     mod_table_text,
    //     mod_table_entry_sizes,
    // )
    empty()
}

fn mod_table_text(x: ModTableEntry) -> impl View {
    label(move || x.title().to_string()).style(|| Style::BASE.color(DARK_TEXT).font_size(14.0))
}

fn mod_entry_text(x: &ModTableEntry, (idx, row): &(usize, ModRow)) -> impl View {
    let row_value = row.value(*idx, *x);
    label(move || row_value.clone()).style(|| Style::BASE.color(DARK_TEXT).font_size(14.0))
}

fn mod_table_entry_sizes(x: &ModTableEntry) -> f32 {
    let base = 24.0;
    match x {
        ModTableEntry::Index => base * 2.,
        ModTableEntry::Name => base * 6.,
        ModTableEntry::Version => base * 6.,
        ModTableEntry::Author => base * 6.,
        ModTableEntry::LastUpdated => base * 6.,
        ModTableEntry::Blank => base * 8.,
    }
}
