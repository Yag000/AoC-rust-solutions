pub fn adjacents(i: usize, j: usize, size_x: usize, size_y: usize) -> Vec<(usize, usize)> {
    let mut adjacents = Vec::new();

    if i > 0 {
        adjacents.push((i - 1, j));
    }
    if i < size_x - 1 {
        adjacents.push((i + 1, j));
    }
    if j > 0 {
        adjacents.push((i, j - 1));
    }
    if j < size_y - 1 {
        adjacents.push((i, j + 1));
    }
    if i > 0 && j > 0 {
        adjacents.push((i - 1, j - 1));
    }
    if i > 0 && j < size_y - 1 {
        adjacents.push((i - 1, j + 1));
    }
    if i < size_x - 1 && j > 0 {
        adjacents.push((i + 1, j - 1));
    }
    if i < size_x - 1 && j < size_y - 1 {
        adjacents.push((i + 1, j + 1));
    }

    adjacents
}
