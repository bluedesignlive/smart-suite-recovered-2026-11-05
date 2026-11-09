//! Squarified treemap for the Space Lens view.
#[derive(Clone, Debug)]
pub struct Cell {
    pub idx: usize,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

/// `sizes` must be sorted descending. Output rects fill `(0,0,w,h)`.
pub fn layout(sizes: &[u64], w: f32, h: f32) -> Vec<Cell> {
    let total: f64 = sizes.iter().map(|s| (*s).max(1) as f64).sum();
    let mut out = vec![];
    if sizes.is_empty() || total <= 0.0 {
        return out;
    }
    let scale = (w * h) as f64 / total;
    let areas: Vec<f64> = sizes.iter().map(|s| (*s).max(1) as f64 * scale).collect();
    let (mut x, mut y, mut rw, mut rh) = (0.0f64, 0.0f64, w as f64, h as f64);
    let mut i = 0;
    while i < areas.len() {
        let short = rw.min(rh);
        let mut j = i + 1;
        let worst = |row: &[f64]| -> f64 {
            let s: f64 = row.iter().sum();
            let (mx, mn) = (row.iter().cloned().fold(0.0, f64::max), row.iter().cloned().fold(f64::MAX, f64::min));
            ((short * short * mx) / (s * s)).max((s * s) / (short * short * mn))
        };
        while j < areas.len() && worst(&areas[i..=j]) <= worst(&areas[i..j]) {
            j += 1;
        }
        let row = &areas[i..j];
        let s: f64 = row.iter().sum();
        if rw >= rh {
            let cw = s / rh;
            let mut cy = y;
            for (k, a) in row.iter().enumerate() {
                let ch = a / cw;
                out.push(Cell { idx: i + k, x: x as f32, y: cy as f32, w: cw as f32, h: ch as f32 });
                cy += ch;
            }
            x += cw;
            rw -= cw;
        } else {
            let ch = s / rw;
            let mut cx = x;
            for (k, a) in row.iter().enumerate() {
                let cw = a / ch;
                out.push(Cell { idx: i + k, x: cx as f32, y: y as f32, w: cw as f32, h: ch as f32 });
                cx += cw;
            }
            y += ch;
            rh -= ch;
        }
        i = j;
    }
    out
}
