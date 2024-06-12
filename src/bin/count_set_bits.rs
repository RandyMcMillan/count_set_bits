use count_set_bits::count_set_bits;
use std::env;
use std::error::Error;
use std::num::ParseIntError;

fn help() {
    use std::process;

    let crate_name = env!("CARGO_CRATE_NAME");
    let version = env!("CARGO_PKG_VERSION");
    print!("\n{} v{}\n\n", crate_name.replace("_", "-"), version);
    print!("{} get\n", crate_name.replace("_", "-"));
    print!("       <csv_relay_list>\n");
    print!("{} json\n", crate_name.replace("_", "-"));
    print!("       <json_relay_list>\n");
    print!("{} stripped\n", crate_name.replace("_", "-"));
    print!("       <string_relay_list> <int_length_last>\n");
    process::exit(0);
}
fn version() {
    use std::process;

    print!("");

    let version = env!("CARGO_PKG_VERSION");
    let crate_name = env!("CARGO_CRATE_NAME");
    //let name = env!("CARGO_PKG_NAME");
    //let author = env!("CARGO_PKG_AUTHORS");

    //println!("Program Name: {}", name);
    //println!("Program Version: {}", version);
    println!("{} v{}", crate_name.replace("_", "-"), version);
    //println!("Program Version: {}", version);
    //println!("Program Author: {}", author);

    process::exit(0);
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Error: Please provide a number as an argument.");
    }

    if args.len() > 1 {
        let arg_str = &args[1];
        let num: u128 = arg_str.parse::<u128>()?;
        let set_bits = count_set_bits(num.try_into().unwrap());
        println!(" {}", set_bits);
    }
    Ok(())
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
