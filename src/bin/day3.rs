use std::fs;
pub fn solve1(input: &str) -> String {
    // assign an int to each bit position (there are 12 in the input)
    let mut pos_bits: [i32; 12] = [0; 12];
    //for every bitstring increment the position bits on 1, else decrement
    count_bits(input, &mut pos_bits);

    //clamp the pos_bits between 1 and 0. This is the gamma_rate
    let gamma: u16 = get_gamma(&mut pos_bits);

    // invert the bits and get the epsilon_rate
    let all_ones: u16 = (2 << 11) - 1;
    let epsilon: u16 = all_ones - gamma;
    // let epsilon: u16 = !gamma;

    //multiply them to get the result
    let result: u32 = gamma as u32 * epsilon as u32;
    result.to_string()
}

fn get_gamma(pos_bits: &mut [i32; 12]) -> u16 {
    let mut gamma_bits: [u16; 12] = [0; 12];
    for (i, num) in pos_bits.iter_mut().enumerate() {
        gamma_bits[i] = if *num > 0 { 1 } else { 0 }
    }
    binary_to_decimal(&gamma_bits)
}

fn binary_to_decimal(gamma_bits: &[u16])-> u16{
    let mut gamma: u16 = 0;
    for num in gamma_bits.iter() {
        gamma += *num;
        gamma <<= 1;
    }
    gamma >>= 1;
    gamma
}

fn count_bits(input: &str, pos_bits: &mut [i32; 12]) {
    for i in input.split_whitespace() {
        for i in i.char_indices() {
            pos_bits[i.0] = match i {
                (_, '1') => pos_bits[i.0] + 1,
                _ => pos_bits[i.0] - 1,
            };
        }
    }
}

fn main() {
    let input = fs::read_to_string("./input/3.txt").unwrap();
    let res = solve1(&input);
    println!("{res}");
    let input = fs::read_to_string("./input/3.txt").unwrap();
    let res = solve2(&input);
    println!("{res}");
}







// consider 1st, 2nd, i-th, ... bit. oxygen: let c be the most common bit from the list at that position (at tie, be 1). Keep those binary numbers which have c as the i -th position. If you only have 1 number remaining, you have found the oxygen. If not, continue i to the next bit position.
// same for co2 but with the least common bit. (at a tie, be 0)

pub fn solve2(input: &str) -> String {
    let input: Vec<&str> = input.split_whitespace().collect();
    let oxygen: u16 = calc_param(input.clone(), most_common_method);
    let co2: u16 = calc_param(input, least_common_method);
    let result: u32 = oxygen as u32 * co2 as u32;
    result.to_string()
}

fn least_common_method(ones_minus_zeros: i16) -> char {
    if ones_minus_zeros >= 0 {
        '0'
    } else {
        '1'
    }
}

fn most_common_method(ones_minus_zeros: i16) -> char {
    if ones_minus_zeros >= 0 {
        '1'
    } else {
        '0'
    }
}

fn calc_param<F>(mut input: Vec<&str>, method: F) -> u16
where
    F: Fn(i16) -> char,
{
    let mut last_num: &str;
    let mut res: &str = input[0];
    let positions = 0..input[0].len();

    for pos in positions {
        let ones_to_zeros = count_ones_to_zeroes(&input, pos);
        let searched_bit = method(ones_to_zeros);
        last_num = input.last().unwrap();
        input.retain(|num| num.chars().nth(pos).unwrap() == searched_bit);

        if input.is_empty() {
            res = last_num;
            break;
        } else if input.len() == 1 {
            res = input.first().unwrap();
            break;
        }
    }
    binary_to_decimal2(res)
}
fn binary_to_decimal2(binary: &str) -> u16 {
    let mut decimal: u16 = 0;
    for num in binary.chars() {
        decimal += num.to_digit(10).unwrap() as u16; 
        decimal <<= 1;
    }
    decimal >>= 1;
    decimal
}

fn count_ones_to_zeroes(input: &[&str], pos: usize) -> i16 {
    let mut result = 0;
    for num in input {
        result = match num.chars().nth(pos).unwrap() {
            '1' => result + 1,
            '0' => result - 1,
            _ => unreachable!(),
        };
    }
    result
}
