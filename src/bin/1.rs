use aoc2021;

fn split_into_numbers<'a>(text: &'a str) -> Box<dyn Iterator<Item = i32> + 'a> {
    Box::new(
        text.split_whitespace()
            .map(|num| num.parse::<i32>().expect("not every input is number")),
    )
}
fn task1(input: &mut impl Iterator<Item = i32>) -> u32 {
    let mut prev_num = input.next().unwrap();
    let mut count: u32 = 0;
    for num in input.skip(1) {
        if prev_num < num {
            count += 1;
        };
        prev_num = num;
    }
    count
}

fn task2(input: &mut impl Iterator<Item = i32>) {}

mod tests {
    #[test]
    fn task2_works() {}
}
fn main() {
    let input = aoc2021::read_input();
    let mut input = split_into_numbers(&input);
    let result = task1(&input);
    println!("{result}");
}
