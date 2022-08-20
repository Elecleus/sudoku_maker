use rand::prelude::*;

pub struct SudokuMap {
    map: [[u8; 9]; 9],
}
// The 9x9 map, as an array.

pub struct MapPoint {
    row: usize,
    column: usize,
}

pub enum MapLine {
    Row(usize),
    Column(usize),
}

impl MapLine {
    pub fn new_in_group(group_order: usize, order_in_group: usize, line_type: bool) -> MapLine {
        if group_order > 2 || order_in_group > 2 {
            panic!();
        }
        let order: usize = group_order * 3 + order_in_group;
        match line_type {
            // "true" for row, and "false" for column.
            true => MapLine::Row(order),
            false => MapLine::Column(order),
        }
    }
}

impl MapPoint {
    pub fn new(row: usize, column: usize) -> MapPoint {
        MapPoint { row, column }
    }
}

impl SudokuMap {
    pub fn new() -> SudokuMap {
        SudokuMap {
            map: [
                [1, 2, 3, 4, 5, 6, 7, 8, 9],
                [7, 8, 9, 1, 2, 3, 4, 5, 6],
                [4, 5, 6, 7, 8, 9, 1, 2, 3],
                [9, 1, 2, 3, 4, 5, 6, 7, 8],
                [6, 7, 8, 9, 1, 2, 3, 4, 5],
                [3, 4, 5, 6, 7, 8, 9, 1, 2],
                [8, 9, 1, 2, 3, 4, 5, 6, 7],
                [5, 6, 7, 8, 9, 1, 2, 3, 4],
                [2, 3, 4, 5, 6, 7, 8, 9, 1],
            ],
        }
    }

    pub fn print(&self) {
        for row in 0..9 {
            for column in 0..9 {
                self.print_point(MapPoint::new(row, column));
                print!(" ");
            }
            println!();
        }
        println!();
    }

    pub fn print_point(&self, point: MapPoint) {
        print!("{}", self.map[point.row][point.column]);
    }

    pub fn exchange_point(&mut self, point_a: MapPoint, point_b: MapPoint) {
        let temp_a = self.map[point_a.row][point_a.column];
        self.map[point_a.row][point_a.column] = self.map[point_b.row][point_b.column];
        self.map[point_b.row][point_b.column] = temp_a;
    }

    pub fn get_line(&self, line: MapLine) -> Vec<MapPoint> {
        let mut result = vec![];
        match line {
            MapLine::Row(row) => {
                for column in 0..9 {
                    result.push(MapPoint::new(row, column));
                }
            }
            MapLine::Column(column) => {
                for row in 0..9 {
                    result.push(MapPoint::new(row, column));
                }
            }
        };
        result
    }

    pub fn print_line(&self, line: MapLine) {
        let line = self.get_line(line);
        for point in line.iter() {
            self.print_point(*point);
            print!(" ");
        }
        println!();
    }

    pub fn exchange_line(&mut self, two_lines: (MapLine, MapLine)) {
        let (line_a, line_b) = two_lines;

        match (line_a, line_b) {
            (MapLine::Row(a), MapLine::Row(b)) => {
                if !is_in_the_same_group(a, b) {
                    panic!("The two lines to exchange are in differet groups.");
                }
            }
            (MapLine::Column(a), MapLine::Column(b)) => {
                if !is_in_the_same_group(a, b) {
                    panic!("The two lines to exchange are in differet groups.");
                }
            }
            (MapLine::Row(_), MapLine::Column(_)) => {
                panic!("The two lines to exchange are in differet types.");
            }
            (MapLine::Column(_), MapLine::Row(_)) => {
                panic!("The two lines to exchange are in differet types.");
            }
        }

        let line_a = self.get_line(line_a);
        let line_b = self.get_line(line_b);
        for n in 0..9 {
            self.exchange_point(line_a[n], line_b[n]);
        }

        fn is_in_the_same_group(a: usize, b: usize) -> bool {
            a / 3 == b / 3
        }
    }
}

impl Copy for MapPoint {}
impl Clone for MapPoint {
    fn clone(&self) -> MapPoint {
        *self
    }
}

impl Copy for MapLine {}
impl Clone for MapLine {
    fn clone(&self) -> MapLine {
        *self
    }
}

pub fn mk_two_lines() -> (MapLine, MapLine) {
    let mut rng = rand::thread_rng();
    let line_type = rng.gen_bool(0.5);
    let group_order = rng.gen_range(0..3);
    let mut order_in_group_1 = 0;
    let mut order_in_group_2 = 0;
    loop {
        if &order_in_group_1 == &order_in_group_2 {
            order_in_group_1 = rng.gen_range(0..3);
            order_in_group_2 = rng.gen_range(0..3);
            continue;
        }
        break;
    }
    let result_1 = MapLine::new_in_group(group_order, order_in_group_1, line_type);
    let result_2 = MapLine::new_in_group(group_order, order_in_group_2, line_type);
    (result_1, result_2)
}

 pub fn run() {
    let mut map = SudokuMap::new();
    for _n in 0..=100 {
        map.exchange_line(mk_two_lines());
    }
    map.print();
}

pub fn origin() {
    SudokuMap::new().print();
}
