// 辿れる経路全てを返す

pub fn reachable_cells(h: usize, w: usize, x: usize, y: usize, d: usize) -> Vec<(usize, usize)> {
    for dx in 0..=d {
        let dy = d - dx;

        if let Some(nx) = x.checked_add(dx) {
        }
    }

    vec![(0, 0)]
}
