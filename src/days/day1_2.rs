
fn split_into_numbers(text: &str) -> Vec<i32> {
    text.split_whitespace()
        .map(|num| num.parse::<i32>().expect("not every input is number"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task2_works() {
        let input = "199 200 208 210 200 207 240 269 260 263";
        assert_eq!(solve(&input), "5");
    }
}

pub fn solve(input: &str) -> String {
    let input = split_into_numbers(&input);
    if input.len() <= 3 {
        return 0.to_string();
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
    count.to_string()
}
