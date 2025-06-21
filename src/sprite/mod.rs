#[cfg(test)]
mod tests;

pub struct Sprite {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<char>>,
}

impl Sprite {
    pub fn new(matrix: Vec<Vec<u32>>, scale: f32) -> Self {
        let char_matrix: Vec<Vec<char>> = matrix
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
            data,
        }
    }
}
