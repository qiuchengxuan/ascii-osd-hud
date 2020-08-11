use super::drawable::Align;
use crate::drawable::NumOfLine;

pub fn note<T: AsMut<[u8]>>(text: &str, align: Align, output: &mut [T]) -> NumOfLine {
    let mut index = 0;
    for line in text.split('\n') {
        let buffer = output[index].as_mut();
        let offset = match align {
            Align::Center => buffer.len() / 2 - line.len() / 2,
            Align::Right => buffer.len() - line.len(),
            _ => 0,
        };
        buffer[offset..offset + line.len()].copy_from_slice(line.as_bytes());
        index += 1;
    }
    index
}
