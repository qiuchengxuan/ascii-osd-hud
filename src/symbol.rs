use enum_map::{enum_map, Enum, EnumMap};

pub type SymbolIndex = u8;

#[derive(Debug, Enum)]
pub enum Symbol {
    Antenna,
    Battery,
    BoxDrawningLightUp,
    Degree,
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
        Symbol::Antenna => 128,
        Symbol::Battery => 129,
        Symbol::Degree => 130,
        Symbol::VeclocityVector => 131,
        Symbol::Alpha => 132,
        Symbol::Square => 133,
        Symbol::LineTop => 134, // ▔
        Symbol::LineUpper1 => 135, // ⎺
        Symbol::LineUpper2 => 136, // ⎻
        Symbol::LineCenter => 137, // ⎯ or ASCII dash
        Symbol::LineLower1 => 138, // ⎼
        Symbol::LineLower2 => 139, // ⎽
        Symbol::LineBottom => 140, // ▁ or ASCII underscore
        Symbol::BoxDrawningLightUp => 141, // ╵ or ASCII |
        Symbol::ZeroWithTraillingDot => 142,
        Symbol::LineLeft => 152, // ▏
        Symbol::LineLeft1 => 153,
        Symbol::LineVerticalCenter => 154, // ⎪
        Symbol::LineRight1 => 155,
        Symbol::LineRight => 156, // ▕
    }
}
