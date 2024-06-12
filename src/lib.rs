pub fn count_set_bits(x: u128) -> u128 {
    let mut count = 0;
    let mut temp = x;

    // Iterate through each bit position (power of 2)
    while temp > 0 {
        if cfg!(debug_assertions) {
            print!("{}", temp & 1);
        }
        count += temp & 1;
        temp >>= 1;
    }

    count
}

// fn main() {
//   let x = 13; // Example usage, change x as needed
//   let set_bits = count_set_bits(num.try_into().unwrap());
//
//   println!("Number of set bits in {}: {}", x, set_bits);
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cs_bits_64() {
        let result: u128 = count_set_bits(111111111111111111111111111111111111111);
        assert_eq!(result, 64);
    }
    #[test]
    fn cs_bits_128() {
        let result: u128 = count_set_bits(340282366920938463463374607431768211455);
        assert_eq!(result, 128);
    }
}
