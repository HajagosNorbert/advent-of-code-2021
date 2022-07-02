use aoc2021;
fn main() {
    let input = aoc2021::read_input();
    println!("{}", aoc2021::read_input());
}

fn formatInput(input: &str) -> Vec<Movement> {
    let input = input.split_ascii_whitespace();
    let mut movements: Vec<Movement>;
    let mut straight_or_opposit = 1; 
    let mut movement: Movement;
    for i in input {
       let dir = match i {
        "forward" => Direction::forward,
        "down" => Direction::depth,
        "up" => {
            straight_or_opposit = -1;
            Direction::depth 
        },
        _ => unreachable!();
       };
       movement = Movement {dir, ammount: input.next().unwrap().parse::<i32>().unwrap()};
       movements.push(movement);
    }
    movements
}

enum Direction {
    forward,
    depth,
}
struct Movement {
    ammount: i32,
    dir: Direction,
}
struct Position {
    depth: i32,
    forward: i32,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1_works() {
        let input = vec![1, 2, 1, 2, 3, 2, 1];
        assert_eq!(task1(&input), 3);
    }
}
