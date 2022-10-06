use std::fs;
pub fn solve(input: &str) -> String {
    // assign an int to each bit position (there are 12 in the input)
    let mut pos_bits: [i32; 12] = [0; 12];
    //for every bitstring increment the position bits on 1, else decrement
    count_bits(&input, &mut pos_bits);

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
    let res = solve(&input);
    println!("{res}");
}
