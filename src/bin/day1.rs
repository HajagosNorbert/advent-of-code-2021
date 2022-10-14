use aoc2021::read_one_per_line;

pub fn count_depth_increases(input: &[i32], depth_samples: usize) -> String {
    // let input = split_into_numbers(input);
    let res = input
        .windows(depth_samples)
        .filter(|nums| nums[0] < nums[depth_samples - 1])
        .count();
    res.to_string()
}

fn main() {
    let input: Vec<i32> = read_one_per_line("./input/1.txt").unwrap();
    let res = count_depth_increases(&input, 2);
    println!("task2: {res}");
    let input: Vec<i32> = read_one_per_line("./input/1.txt").unwrap();
    let res = count_depth_increases(&input, 4);
    println!("task2: {res}");
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn task1_works() {
        let input = vec![1, 2, 1, 2, 3, 2, 4];
        assert_eq!(count_depth_increases(&input, 2), "4");
    }

    #[test]
    fn task2_works() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_depth_increases(&input, 4), "5");
    }
}
