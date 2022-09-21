// consider 1st, 2nd, i-th, ... bit. oxygen: let c be the most common bit from the list at that position (at tie, be 1). Keep those binary numbers which have c as the i -th position. If you only have 1 number remaining, you have found the oxygen. If not, continue i to the next bit position.
// same for co2 but with the least common bit. (at a tie, be 0)

pub fn solve(input: &str) -> String {
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
        // for num in &input {
        //     if input.len() == 1 {
        //         return num.parse::<u16>().unwrap();
        //     }
        //     //filter for that bit on input
        //     if num.chars().nth(pos).unwrap() != searched_bit as char {
        //
        //
        //     }
        //
        // }
    }
    binary_to_decimal(res)
}
fn binary_to_decimal(binary: &str) -> u16 {
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
