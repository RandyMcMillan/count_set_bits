use count_set_bits::count_set_bits;
fn main() {
    let x = 13; // Example usage, change x as needed
    let set_bits = count_set_bits(x);

    println!(" {}", set_bits);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cs_bits() {
        let result = count_set_bits(13);
        assert_eq!(result, 3);
    }
}
