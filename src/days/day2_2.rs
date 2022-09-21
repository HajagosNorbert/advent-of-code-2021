pub fn solve(input: &str) -> String {
    let input = input.split_whitespace();

    let mut aim: i32 = 0;
    let mut input = input.into_iter();
    let mut pos = Position {
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
struct Position {
    horizontal: i32,
    depth: i32,
}
