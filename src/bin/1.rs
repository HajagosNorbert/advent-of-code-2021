use aoc2021;

fn split_into_numbers(text: &str) -> Vec<i32> {
    text.split_whitespace()
        .map(|num| num.parse::<i32>().expect("not every input is number"))
        .collect()
}
fn task1(input: &Vec<i32>) -> u32 {
    let mut input = input.iter();
    let mut prev_num = input.next().unwrap();
    let mut count: u32 = 0;
    for num in input {
        if prev_num < num {
            count += 1;
        };
        prev_num = num;
    }
    count
}

fn task2(input: &Vec<i32>) -> u32 {
    if input.len() <= 3 {
        return 0;
    }

    let mut count: u32 = 0;
    let mut prev = input[0] + input[1] + input[2];
    let mut curr;
    for i in 1..(input.len() - 2) {
        curr = prev - input[i - 1] + input[i + 2];
        if prev < curr {
            count += 1;
        }
        prev = curr;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1_works() {
        let input = vec![1, 2, 1, 2, 3, 2, 1];
        assert_eq!(task1(&input), 3);
    }
    #[test]
    fn task2_works() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(task2(&input), 5);
    }
}
fn main() {
    let input = aoc2021::read_input();
    let input = split_into_numbers(&input);
    let result = task2(&input);
    println!("{result}");
}
