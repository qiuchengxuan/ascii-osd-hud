use enum_map::{enum_map, Enum, EnumMap};

pub type SymbolIndex = u8;

#[derive(Debug, Enum)]
pub enum Symbol {
    Space,
    Antenna,
    Battery,
    BoxDrawningLightUp,
    Degree,
    CrossHair,
    VeclocityVector,
    Alpha,
    Square,
    LineTop,
    LineUpper1,
    LineUpper2,
    LineCenter,
    LineLower1,
    LineLower2,
    LineBottom,
    ZeroWithTraillingDot,
    LeftOneEighthBlock,
    RightOneEighthBlock,
}

pub type SymbolTable = EnumMap<Symbol, SymbolIndex>;

pub fn default_symbol_table() -> SymbolTable {
    enum_map! {
        Symbol::Space => 0, // duplicate of ASCII 32 space
        Symbol::Antenna => 1,
        Symbol::Battery => 2,
        Symbol::Degree => 3,
        Symbol::CrossHair => 4,
        Symbol::VeclocityVector => 5,
        Symbol::Alpha => 6,
        Symbol::Square => 7,
        Symbol::LineTop => 8, // ▔
        Symbol::LineUpper1 => 9, // ⎺
        Symbol::LineUpper2 => 10, // ⎻
        Symbol::LineCenter => 11, // ⎯ or ASCII dash
        Symbol::LineLower1 => 12, // ⎼
        Symbol::LineLower2 => 13, // ⎽
        Symbol::LineBottom => 14, // ▁ or ASCII underscore
        Symbol::BoxDrawningLightUp => 15, // ╵ or ASCII |
        Symbol::ZeroWithTraillingDot => 16,
        // 17~26 number with trailling dot
        Symbol::LeftOneEighthBlock => 27, // ▏
        Symbol::RightOneEighthBlock => 28, // ▕
        // 32-33 ASCII
        // 40-62 ASCII 48-57 0-9
        // 64-95 ASCII
    }
}
