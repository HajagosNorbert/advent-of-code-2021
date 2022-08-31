use aoc2021;

fn main(){
    let input = aoc2021::read_input_file("2.txt");
    let input = input.split_whitespace();    

    let mut aim:i32 = 0;
    let mut input = input.into_iter();
    let mut pos = Position {
        Horizontal: 0,
        Depth: 0
    };
    //go through the commands line by line!
    while let Some(cmd) = input.next() {
        match cmd {
            //if command concerns it, modify the aim 
            "down" => aim += input.next().unwrap().parse::<i32>().unwrap(),
            "up" => aim -= input.next().unwrap().parse::<i32>().unwrap(),
            //move forward and depth based on aim and input
            "forward" => {

                aim = 0;
            },
            _ => unreachable!()
        };
    }
}



//create one position struct where the current location is stored
struct Position {
    Horizontal: i32,
    Depth: i32
}

