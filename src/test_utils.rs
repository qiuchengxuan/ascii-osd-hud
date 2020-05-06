use std::string::String;
use std::vec::Vec;

use ascii::ToAsciiChar;

const UTF8_SYMBOLS: &str = " ⏉β°w⌖⍺☐▔⎺⎻⎯⎼⎽▁╵₀₁₂₃₄₅₆₇₈₉▏▕";

pub fn to_utf8_string<T: AsRef<[u8]>>(lines: &[T]) -> String {
    let mut output = String::new();
    let symbols: Vec<char> = UTF8_SYMBOLS.chars().collect();
    for line in lines.iter().map(|l| l.as_ref()) {
        for &byte in line.iter() {
            let ch = byte.to_ascii_char().unwrap().as_char();
            output.push(*symbols.get(byte as usize).unwrap_or(&ch))
        }
    }
    output
}
