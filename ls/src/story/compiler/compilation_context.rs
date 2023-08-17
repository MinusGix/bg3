/// Determines the game version we're targeting during compilation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetGame {
    DOS2,
    DOS2DE,
    BG3,
}
