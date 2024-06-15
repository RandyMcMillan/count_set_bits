# count\_set\_bits

```
cargo add count_set_bits
```

#### main example
```
use count_set_bits::count_set_bits;
fn main() {
    let num = 13; // Example usage, change x as needed
    let set_bits = count_set_bits(num.try_into().unwrap(), true);

    println!("\nNumber of set bits in {}: {}", num, set_bits);
}
```