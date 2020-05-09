use std::string::String;
use std::vec::Vec;

use ascii::ToAsciiChar;

const UTF8_SYMBOLS: &str = " ⏉β°w⌖⍺☐▔⎺⎻⎯⎼⎽▁╵₀⒈⒉⒊⒋⒌⒍⒎⒏⒐▏▕";

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

pub fn fill_edge<T: AsMut<[u8]>>(buffer: &mut [T]) {
    buffer.iter_mut().for_each(|mutable| {
        let line = mutable.as_mut();
        if *line.last().unwrap() == 0u8 {
            line[line.len() - 1] = '.' as u8;
        }
        if *line.first().unwrap() == 0u8 {
            line[0] = '.' as u8;
        }
    });
}

pub trait ZeroSlice<T> {
    fn zero(&mut self);
}

impl ZeroSlice<u8> for [u8] {
    fn zero(&mut self) {
        for x in self {
            *x = 0
        }
    }
}
