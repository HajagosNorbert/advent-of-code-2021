use std::fs;

pub fn solve2(input: &str) -> String {
    let input = input.split_whitespace();

    let mut aim: i32 = 0;
    let mut input = input;
    let mut pos = Position1 {
        horizontal: 0,
        depth: 0,
    };
    //go through the commands line by line!
    while let Some(cmd) = input.next() {
        let ammount: i32 = input.next().unwrap().parse().unwrap();
        match cmd {
            //if command concerns it, modify the aim
            "down" => aim += ammount,
            "up" => aim -= ammount,
            //move forward and depth based on aim and input
            "forward" => {
                pos.horizontal += ammount;
                pos.depth += aim * ammount;
            }
            _ => unreachable!(),
        };
    }
    let result = pos.horizontal * pos.depth;
    result.to_string()
}

//create one position struct where the current location is stored
struct Position1 {
    horizontal: i32,
    depth: i32,
}
pub fn solve(input: &str) -> String {
    let movements = format_input(input);
    let pos = calculate_position(&movements);
    let result = pos.forward * pos.depth;
    result.to_string()
}

fn calculate_position(movements: &[Movement]) -> Position {
    let mut pos = Position {
        depth: 0,
        forward: 0,
    };
    for m in movements {
        match m.dir {
            Direction::Forward => pos.forward += m.ammount,
            Direction::Depth => pos.depth += m.ammount,
        }
    }

    pos
}

fn format_input(input: &str) -> Vec<Movement> {
    let input = input.split_ascii_whitespace();
    let mut movements: Vec<Movement> = vec![];
    let mut straight_or_opposit: i32;
    let mut movement: Movement;
    let mut input = input;
    while let Some(i) = input.next() {
        straight_or_opposit = 1;
        let dir = match i {
            "forward" => Direction::Forward,
            "down" => Direction::Depth,
            "up" => {
                straight_or_opposit = -1;
                Direction::Depth
            }
            _ => unreachable!(),
        };
        movement = Movement {
            dir,
            ammount: input.next().unwrap().parse::<i32>().unwrap() * straight_or_opposit,
        };
        movements.push(movement);
    }
    movements
}

#[derive(Eq, PartialEq, Debug)]
enum Direction {
    Forward,
    Depth,
}
#[derive(Eq, PartialEq, Debug)]
struct Movement {
    ammount: i32,
    dir: Direction,
}

#[derive(Eq, PartialEq, Debug)]
struct Position {
    depth: i32,
    forward: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1_format_input_works() {
        let input = "forward 5\ndown 5\nup 3";
        let calculated_movements = format_input(input);

        let correct_movements = &vec![
            Movement {
                ammount: 5,
                dir: Direction::Forward,
            },
            Movement {
                ammount: 5,
                dir: Direction::Depth,
            },
            Movement {
                ammount: -3,
                dir: Direction::Depth,
            },
        ];
        assert!(calculated_movements
            .iter()
            .zip(correct_movements)
            .all(|(a, b)| a == b));
    }

    #[test]
    fn task1_calculate_position_works() {
        let movements = &vec![
            Movement {
                ammount: 5,
                dir: Direction::Forward,
            },
            Movement {
                ammount: 5,
                dir: Direction::Depth,
            },
            Movement {
                ammount: -3,
                dir: Direction::Depth,
            },
        ];
        let pos = calculate_position(movements);
        assert_eq!(
            pos,
            Position {
                forward: 5,
                depth: 2
            }
        )
    }
}

fn main() {
    let input = fs::read_to_string("./input/2.txt").unwrap();
    let res = solve(&input);
    println!("{res}");
    let input = fs::read_to_string("./input/2.txt").unwrap();
    let res = solve2(&input);
    println!("{res}");
}
