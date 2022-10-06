pub fn solve(input: &str) -> String {
    //get the generated numbers into a [u8]
    let (generated_nums, boards) = input.split_once('\n').unwrap();
    let generated_nums: Vec<u8> = generated_nums
        .split(',')
        .map(|num| num.parse::<u8>().unwrap())
        .collect();

    // populate the boards with the rest of the input data. every 25th element is the final cell
    let mut boards: Vec<Board> = boards
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
    let mut solution: u32 = 0;
    'outer: for num in generated_nums {
        for board in boards.iter_mut() {
            let res = board.mark(num);
            if let Some(answer) = res {
                solution = answer;
                break 'outer;
            };
        }
    }

    solution.to_string()

    // on each change, check if the rows or columns yield victory by being all marked

    // if yes multiply the current generated number with the sum of unmarked values

    // sum of unmarked values calculated with going through the board once again, excluding either a row or a coulumn (enum)
    // generated_nums.to_string()
}

// create a Board struct with cell-s in it as a 2d matrix
struct Board {
    cells: Vec<Vec<Cell>>,
}

impl Board {
    fn from(table: &[Vec<u8>]) -> Board {
        let cells = table
            .iter()
            .map(|row| {
                row.iter()
                    .map(|num| Cell::from(*num))
                    .collect::<Vec<Cell>>()
            })
            .collect::<Vec<Vec<Cell>>>();
        Board { cells }
    }
// use array of structs to struct of arrays transformation (possibly with a flattened array ) then watch prime how he did it.
    fn mark(&mut self, num: u8) -> Option<u32> {
        for rownum in 0..5 {
            for colnum in 0..5 {
                let cell = self.cells.get(rownum).unwrap().get(colnum).unwrap();
                if cell.value == num && !cell.marked {
                    let res = self.check_if_bingo(rownum, colnum );
                    if let Some(answer) = res {
                        return Some(answer);
                    }

                }
            }
        }
        None
    }

fn check_if_bingo(&mut self, rownum: usize, colnum: usize) -> Option<u32> {
        let mut marked_rows: u8 = 0;
        let mut marked_cols: u8 = 0;
        for i in 0..5 {
            if self.cells.get(rownum).unwrap().get(i).unwrap().marked {
                marked_rows += 1;
            };
            if self.cells.get(i).unwrap().get(colnum).unwrap().marked {
                marked_cols += 1;
            };
        }

        let cell = self.cells.get_mut(rownum).unwrap().get_mut(colnum).unwrap();
        cell.marked = true ;
        if marked_rows == 5 || marked_cols == 5 {
            return Some(1);
        }
        None
    }
}

// create a Cell struct with value and is marked in it
struct Cell {
    value: u8,
    marked: bool,
}

impl Cell {
    fn from(num: u8) -> Cell {
        Cell {
            value: num,
            marked: false,
        }
    }
}
