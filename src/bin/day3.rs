#[derive(Debug, Clone, Copy)]
enum Point {
    Number {
        digit: u8,
        number: usize,
        start_position: usize,
    },
    Symbol,
    Dot,
}
fn main() {
    let map_string = include_str!("ex3.txt");
    let map = map_string
        .lines()
        .map(|line| {
            let mut point_row = Vec::new();
            let mut number_buf = String::new();
            for current in line.chars() {
                match current {
                    '.' => {
                        let number_start_index = point_row.len();
                        if !number_buf.is_empty() {
                            let number = number_buf.parse::<usize>().unwrap();
                            point_row.extend(number_buf.chars().map(|digit| Point::Number {
                                number,
                                digit: digit.to_digit(10).unwrap() as u8,
                                start_position: number_start_index,
                            }));
                            number_buf.clear();
                        }
                        point_row.push(Point::Dot);
                    }
                    digit if digit.is_ascii_digit() => {
                        number_buf.push(digit);
                    }
                    _ => {
                        let number_start_index = point_row.len();
                        if !number_buf.is_empty() {
                            let number = number_buf.parse::<usize>().unwrap();
                            point_row.extend(number_buf.chars().map(|digit| Point::Number {
                                number,
                                digit: digit.to_digit(10).unwrap() as u8,
                                start_position: number_start_index,
                            }));
                            number_buf.clear();
                        }
                        point_row.push(Point::Symbol);
                    }
                }
            }
            point_row
        })
        .collect::<Vec<Vec<Point>>>();

    let mut result = 0;
    let mut last_part_number_position: Option<usize> = None;
    for (y, row) in map.iter().enumerate() {
        for (x, point) in row.iter().enumerate() {
            if let Point::Number {
                digit,
                number,
                start_position,
            } = point
            {
                if last_part_number_position.is_none()
                    && (
                        // Check right
                        matches!(row.get(x+1), Some(Point::Symbol)) ||
                        // Check left
                        matches!(if x == 0 {None} else {row.get(x-1)}, Some(Point::Symbol)) ||
                        // check upwards
                        matches!(if y == 0 {None} else {map.get(y-1).and_then(|row| row.get(x))}, Some(Point::Symbol)) ||
                        // check downwards
                        matches!(map.get(y+1).and_then(|row| row.get(x)), Some(Point::Symbol)) ||
                        // check up left
                        matches!(if y == 0 || x == 0 { None } else {map.get(y-1).and_then(|row| row.get(x-1))}, Some(Point::Symbol)) ||
                        // check up right
                        matches!(if y == 0 {None} else {map.get(y-1).and_then(|row| row.get(x+1))}, Some(Point::Symbol)) ||
                        // check down left
                        matches!(if x == 0 {None} else {map.get(y+1).and_then(|row| row.get(x-1))}, Some(Point::Symbol)) || 
                        // check down right
                        matches!(map.get(y+1).and_then(|row| row.get(x+1)), Some(Point::Symbol))
                    )
                {
                    last_part_number_position = Some(*start_position);
                    result += number;
                }
            } else {
                last_part_number_position = None;
            }
        }
    }
    println!("{result}");
}
