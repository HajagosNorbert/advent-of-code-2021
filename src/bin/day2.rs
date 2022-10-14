use aoc2021::read_one_per_line;

fn main() {
    let input: Vec<Dir> = read_one_per_line("./input/2.txt").unwrap();
    let res = calculate_position(&input, simple_calculation)
        .get_result()
        .to_string();
    println!("{res}");
    let res = calculate_position(&input, aim_calculation)
        .get_result()
        .to_string();
    println!("{res}");
}

enum Dir {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl std::str::FromStr for Dir {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, amount) = s.split_once(' ').unwrap();
        let amount = amount.parse()?;
        match direction {
            "forward" => Ok(Dir::Forward(amount)),
            "down" => Ok(Dir::Down(amount)),
            "up" => Ok(Dir::Up(amount)),
            _ => unreachable!("this direction should not exist"),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Default)]
struct Position {
    depth: u32,
    forward: u32,
    aim: u32,
}

impl Position {
    fn get_result(&self) -> u32 {
        self.depth as u32 * self.forward as u32
    }

    fn new() -> Self {
        Default::default()
    }
}

fn aim_calculation(dir: &Dir, pos: &mut Position) {
    match dir {
        Dir::Forward(amount) => {
            pos.forward += amount;
            pos.depth += amount * pos.aim
        }
        Dir::Down(amount) => pos.aim += amount,
        Dir::Up(amount) => pos.aim -= amount,
    }
}

fn simple_calculation(dir: &Dir, pos: &mut Position) {
    match dir {
        Dir::Forward(amount) => pos.forward += amount,
        Dir::Down(amount) => pos.depth += amount,
        Dir::Up(amount) => pos.depth -= amount,
    }
}

fn calculate_position<F>(directions: &[Dir], calculation_method: F) -> Position
where
    F: Fn(&Dir, &mut Position),
{
    let mut pos = Position::new();
    for dir in directions {
        calculation_method(dir, &mut pos);
    }
    pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_calculates_position() {
        let input = [Dir::Down(10), Dir::Up(5), Dir::Forward(10)];
        let res = calculate_position(&input, simple_calculation).get_result();
        assert_eq!(res, 50);
    }

    #[test]
    fn day2_calculates_position() {
        let input = [Dir::Down(10), Dir::Up(5), Dir::Forward(10)];
        let res = calculate_position(&input, aim_calculation).get_result();
        assert_eq!(res, 500);
    }
}
