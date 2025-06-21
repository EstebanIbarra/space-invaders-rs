#[cfg(test)]
mod tests;

pub type UnicodeMatrix = Vec<Vec<u32>>;
pub type CharMatrix = Vec<Vec<char>>;

pub struct Sprite {
    pub width: usize,
    pub height: usize,
    pub scale: f32,
    pub data: CharMatrix,
}

impl From<(UnicodeMatrix, f32)> for Sprite {
    fn from((matrix, scale): (UnicodeMatrix, f32)) -> Sprite {
        let char_matrix: CharMatrix = matrix
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|cp| char::from_u32(cp).unwrap_or('\u{FFFD}'))
                    .collect()
            })
            .collect();
        let og_height = char_matrix.len();
        let og_width = char_matrix.first().map_or(0, |r| r.len());
        let height = (((og_height as f32) * scale).max(1.0).round() as usize).min(og_height);
        let width = (((og_width as f32) * scale).max(1.0).round() as usize).min(og_width);
        let data = char_matrix
            .iter()
            .take(height)
            .map(|row| row.iter().cloned().take(width).collect::<Vec<_>>())
            .collect();
        Sprite {
            width,
            height,
            scale,
            data,
        }
    }
}

impl Sprite {
    pub fn update(&mut self, matrix: UnicodeMatrix) {
        let new: Sprite = (matrix, self.scale).into();
        self.width = new.width;
        self.height = new.height;
        self.data = new.data;
    }
}
