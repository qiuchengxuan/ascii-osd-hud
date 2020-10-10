use enum_map::{enum_map, Enum, EnumMap};

pub type SymbolIndex = u8;

#[derive(Debug, Enum)]
pub enum Symbol {
    Antenna,
    Battery,
    BoxDrawningLightUp,
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
    LineLeft,
    LineLeft1,
    LineVerticalCenter,
    LineRight1,
    LineRight,
    ZeroWithTraillingDot,
}

pub fn to_number_with_dot(byte: u8, zero_with_trailling_dot: SymbolIndex) -> u8 {
    if b'0' <= byte && byte <= b'9' {
        if zero_with_trailling_dot > b'0' {
            byte + (zero_with_trailling_dot - b'0')
        } else {
            byte - (b'0' - zero_with_trailling_dot)
        }
    } else {
        zero_with_trailling_dot
    }
}

pub type SymbolTable = EnumMap<Symbol, SymbolIndex>;

pub fn default_symbol_table() -> SymbolTable {
    enum_map! {
        Symbol::Antenna => 1,
        Symbol::Battery => 2,
        Symbol::BoxDrawningLightUp => 3, // ╵ or ASCII |
        Symbol::VeclocityVector => 4,
        Symbol::Alpha => 5,
        Symbol::Square => 6,
        Symbol::LineTop => 128, // ▔
        Symbol::LineUpper1 => 129, // ⎺
        Symbol::LineUpper2 => 130, // ⎻
        Symbol::LineCenter => 131, // ⎯ or ASCII dash
        Symbol::LineLower1 => 132, // ⎼
        Symbol::LineLower2 => 133, // ⎽
        Symbol::LineBottom => 134, // ▁ or ASCII underscore
        Symbol::LineLeft => 135, // ▏
        Symbol::LineLeft1 => 136,
        Symbol::LineVerticalCenter => 137, // ⎪
        Symbol::LineRight1 => 138,
        Symbol::LineRight => 139, // ▕
        Symbol::ZeroWithTraillingDot => 144,
    }
}
