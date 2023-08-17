use lsf::{lsx::LSXVersion, LSFVersion};

use crate::pak::PackageVersion;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Game {
    DivinityOriginalSin = 0,
    DivinityOriginalSinEE,
    DivinityOriginalSin2,
    DivinityOriginalSin2DE,
    BaldursGate3,
}
impl Game {
    pub fn is_fw3(self) -> bool {
        self != Game::DivinityOriginalSin && self != Game::DivinityOriginalSinEE
    }

    pub fn pak_version(self) -> PackageVersion {
        match self {
            Game::DivinityOriginalSin => PackageVersion::V7,
            Game::DivinityOriginalSinEE => PackageVersion::V9,
            Game::DivinityOriginalSin2 => PackageVersion::V10,
            Game::DivinityOriginalSin2DE => PackageVersion::V13,
            Game::BaldursGate3 => PackageVersion::V18,
        }
    }

    pub fn lsf_version(self) -> LSFVersion {
        match self {
            Game::DivinityOriginalSin => LSFVersion::ChunkedCompress,
            Game::DivinityOriginalSinEE => LSFVersion::ChunkedCompress,
            Game::DivinityOriginalSin2 => LSFVersion::ExtendedNodes,
            Game::DivinityOriginalSin2DE => LSFVersion::ExtendedNodes,
            Game::BaldursGate3 => LSFVersion::BG3AdditionalBlob,
        }
    }

    pub fn lsx_version(self) -> LSXVersion {
        match self {
            Game::DivinityOriginalSin
            | Game::DivinityOriginalSinEE
            | Game::DivinityOriginalSin2
            | Game::DivinityOriginalSin2DE => LSXVersion::V3,
            Game::BaldursGate3 => LSXVersion::V4,
        }
    }
}
