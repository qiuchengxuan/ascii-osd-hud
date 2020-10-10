use std::string::String;
use std::vec::Vec;

use ascii::ToAsciiChar;

const SYMBOLS: &str = "⏉β╵⏂⍺☐";
const LINES: &str = "▔⎺⎻─⎼⎽▁▏▏|⎪⎪";
const DOTTED_NUMBERS: &str = "₀⒈⒉⒊⒋⒌⒍⒎⒏⒐";

pub fn to_utf8_string<T: AsRef<[u8]>>(screen: &[T]) -> String {
    let mut output = String::new();
    let symbols: Vec<char> = SYMBOLS.chars().collect();
    let lines: Vec<char> = LINES.chars().collect();
    let dotted_numbers: Vec<char> = DOTTED_NUMBERS.chars().collect();
    for line in screen.iter().map(|l| l.as_ref()) {
        for &byte in line.iter() {
            output.push(match byte {
                0 => ' ',
                1..=6 => *symbols.get(byte as usize - 1).unwrap(),
                128..=139 => *lines.get(byte as usize - 128).unwrap(),
                144..=154 => *dotted_numbers.get(byte as usize - 144).unwrap(),
                _ => byte.to_ascii_char().unwrap().as_char(),
            });
        }
    }
    output
}

pub fn fill_edge<T: AsMut<[u8]>>(buffer: &mut [T]) {
    buffer.iter_mut().for_each(|mutable| {
        let line = mutable.as_mut();
        if *line.last().unwrap() == 0u8 {
            line[line.len() - 1] = b'.';
        }
        if *line.first().unwrap() == 0u8 {
            line[0] = b'.';
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
