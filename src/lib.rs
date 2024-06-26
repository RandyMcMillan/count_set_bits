///Iterate through each bit position (power of 2)
///
/// ###### count_set_bits
///
/// ```
/// pub fn count_set_bits(x: u128, print: bool) -> u128 {
///     let mut count = 0;
///     let mut temp = x;
///
///     // Iterate through each bit position (power of 2)
///     while temp > 0 {
///         if print {
///             print!("{}", temp & 1);
///         }
///         count += temp & 1;
///         temp >>= 1;
///     }
///
///     count
/// }
/// ```
///
/// ###### main example
///
/// ```
/// use count_set_bits::count_set_bits;
/// fn main() {
///     let num = 13; // Example usage, change x as needed
///     let set_bits = count_set_bits(num.try_into().unwrap(), true);
///
///     println!("\nNumber of set bits in {}: {}", num, set_bits);
/// }
/// ```
///

pub fn count_set_bits(x: u128, print: bool) -> u128 {
    let mut count = 0;
    let mut temp = x;

    // Iterate through each bit position (power of 2)
    while temp > 0 {
        if print {
            print!("{}", temp & 1);
        }
        count += temp & 1;
        temp >>= 1;
    }

    count
}

#[cfg(test)]
/// mod tests
mod tests {
    use super::*;
    #[test]
    fn cs_bits_64() {
        let result: u128 = count_set_bits(111111111111111111111111111111111111111, true);
        print!("\nresult={}\n", result);
        assert_eq!(result, 64);
    }
    #[test]
    fn cs_bits_128() {
        let result: u128 = count_set_bits(340282366920938463463374607431768211455, true);
        print!("\nresult={}\n", result);
        assert_eq!(result, 128);
    }
}
