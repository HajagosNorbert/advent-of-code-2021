fn split_into_numbers(text: &str) -> Vec<i32> {
    text.split_whitespace()
        .map(|num| num.parse::<i32>().expect("not every input is number"))
        .collect()
}

#[cfg(test)]
mod tests {
   use super::*; 
    #[test]
    fn task1_works() {
        let input = "1 2 1 2 3 2 4";
        assert_eq!(solve(input), "3");
    }
}

pub fn solve(input: &str) -> String{
    let input = split_into_numbers(input);

    let mut input = input.iter();
    let mut prev_num = input.next().unwrap();
    let mut count: u32 = 0;
    for num in input {
        if prev_num < num {
            count += 1;
        };
        prev_num = num;
    }
    count.to_string()
}
