pub struct Map {
    map: [[u8; 9]; 9],
}

pub struct Point {
    row: usize,
    column: usize,
}

pub enum Line {
    Row(usize),
    Column(usize),
}

