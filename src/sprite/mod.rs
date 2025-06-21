#[cfg(test)]
mod tests;

use std::iter::repeat_n;

pub type UnicodeMatrix = [[u32; 4]; 2];
pub type CharMatrix = Vec<Vec<char>>;

pub struct Sprite {
    pub width: usize,
    pub height: usize,
    pub scale: f32,
    pub data: CharMatrix,
}

impl From<(UnicodeMatrix, f32)> for Sprite {
    fn from((matrix, scale): (UnicodeMatrix, f32)) -> Sprite {
        let char_rows: [[char; 4]; 2] = matrix.map(|row| {
            let mut arr = ['\u{FFFD}'; 4];
            for (i, &cp) in row.iter().enumerate() {
                arr[i] = char::from_u32(cp).unwrap_or('\u{FFFD}');
            }
            arr
        });
        let mut width = (3.0 * scale).round().max(3.0) as usize;
        if width % 2 == 0 {
            width += 1;
        }
        let fill_count = (width - 3) / 2;
        let mut data: CharMatrix = Vec::with_capacity(2);
        for row in &char_rows {
            let filler = row[3];
            let mut new_row = Vec::with_capacity(width);
            new_row.push(row[0]);
            new_row.extend(repeat_n(filler, fill_count));
            new_row.push(row[1]);
            new_row.extend(repeat_n(filler, fill_count));
            new_row.push(row[2]);
            data.push(new_row);
        }

        Sprite {
            width,
            height: 2,
            scale,
            data,
        }
    }
}
