use enum_map::{enum_map, Enum, EnumMap};

pub type SymbolIndex = u8;

#[derive(Debug, Enum)]
pub enum Symbol {
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
    SmallBlackSquare,
    VerticalLine,
    ZeroWithTraillingDot,
}

pub fn to_number_with_dot(byte: u8, zero_with_trailling_dot: SymbolIndex) -> u8 {
    if '0' as u8 <= byte && byte <= '9' as u8 {
        if zero_with_trailling_dot > '0' as u8 {
            byte + (zero_with_trailling_dot - '0' as u8)
        } else {
            byte - ('0' as u8 - zero_with_trailling_dot)
        }
    } else if byte == 0 {
        zero_with_trailling_dot
    } else {
        byte
    }
}

pub type SymbolTable = EnumMap<Symbol, SymbolIndex>;

pub fn default_symbol_table() -> SymbolTable {
    enum_map! {
        Symbol::Antenna => 128,
        Symbol::Battery => 129,
        Symbol::Degree => 130,
        Symbol::CrossHair => 131,
        Symbol::VeclocityVector => 132,
        Symbol::Alpha => 133,
        Symbol::Square => 134,
        Symbol::LineTop => 135, // ▔
        Symbol::LineUpper1 => 136, // ⎺
        Symbol::LineUpper2 => 137, // ⎻
        Symbol::LineCenter => 138, // ⎯ or ASCII dash
        Symbol::LineLower1 => 139, // ⎼
        Symbol::LineLower2 => 140, // ⎽
        Symbol::LineBottom => 141, // ▁ or ASCII underscore
        Symbol::BoxDrawningLightUp => 142, // ╵ or ASCII |
        Symbol::ZeroWithTraillingDot => 143,
        Symbol::SmallBlackSquare => 153, // ▪
        Symbol::VerticalLine => 154, // ⎪
    }
}
