pub fn count_set_bits(x: u32) -> u32 {
    let mut count = 0;
    let mut temp = x;

    // Iterate through each bit position (power of 2)
    while temp > 0 {
        // Check if the current bit is set (1) using bitwise AND with 1
        print!("{}", temp & 1);
        count += temp & 1;

        // Efficiently move to the next bit position by dividing by 2
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
    fn cs_bits() {
        let result = count_set_bits(13);
        assert_eq!(result, 3);
    }
}
