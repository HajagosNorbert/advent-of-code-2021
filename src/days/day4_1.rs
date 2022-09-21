use core::slice::SlicePattern;

pub fn solve(input: &str) -> String {
    //get the generated numbers into a [u8]
    let (generated_nums, boards) = input.split_once('\n').unwrap();
    let generated_nums: Vec<u8> = generated_nums
        .split(',')
        .map(|num| num.parse::<u8>().unwrap())
        .collect();

    // populate the boards with the rest of the input data. every 25th element is the final cell
    let boards: Vec<Board> = boards
        .split("\n\n")
        .map(|table_input| {
            table_input
                .split('\n')
                .map(|line| {
                    line.split_whitespace()
                        .map(|num| num.parse::<u8>().unwrap())
                        .collect::<Vec<u8>>()
                })
                .collect::<Vec<Vec<u8>>>()
        })
        .map(|table_input| Board::from(&table_input))
        .collect();

    // go through the generated numbers and change the marked property on each Board's each cell.
    for gen_num in generated_nums {}

    // println!("{}", boards[0]);
    boards.last().unwrap().to_string()
    // on each change, check if the rows or columns yield victory by being all marked

    // if yes multiply the current generated number with the sum of unmarked values

    // sum of unmarked values calculated with going through the board once again, excluding either a row or a coulumn (enum)
    // generated_nums.to_string()
}

// create a Board struct with cell-s in it as a 2d matrix
struct Board {
    cells: [[Cell; 5]; 5],
}

impl Board {
    fn from(table: &[Vec<u8>]) -> Board {
        Board {
            cells: [Cell::from("0")],
        }
    }
}

// create a Cell struct with value and is marked in it
struct Cell {
    value: u8,
    marked: bool,
}
