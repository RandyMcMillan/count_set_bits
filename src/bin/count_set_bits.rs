use count_set_bits::count_set_bits;
use std::env;
use std::error::Error;
use std::num::ParseIntError;

fn version() {
    print!("");

    let version = env!("CARGO_PKG_VERSION");
    let crate_name = env!("CARGO_CRATE_NAME");
    println!("{} v{}", crate_name.replace("_", "-"), version);

    std::process::exit(0);
}
fn increase(number: i32) {
    println!("{}", number + 1);
    std::process::exit(0);
}

fn decrease(number: i32) {
    println!("{}", number - 1);
    std::process::exit(0);
}

fn help() {
    let crate_name = env!("CARGO_CRATE_NAME");
    let version = env!("CARGO_PKG_VERSION");
    print!("\n{} v{}\n\n", crate_name.replace("_", "-"), version);
    print!("{} get\n", crate_name.replace("_", "-"));
    print!("       <csv_relay_list>\n");
    print!("{} json\n", crate_name.replace("_", "-"));
    print!("       <json_relay_list>\n");
    print!("{} stripped\n", crate_name.replace("_", "-"));
    print!("       <string_relay_list> <int_length_last>\n");
    std::process::exit(0);
    println!(
        "usage:
match_args <string>
    Check whether given string is the answer.
match_args {{increase|decrease}} <integer>
    Increase or decrease given integer by one."
    );
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        // no arguments passed
        1 => {
            let mut arg_str = String::new();
            let stdin = std::io::stdin();
            stdin.read_line(&mut arg_str).unwrap();
            arg_str = arg_str.trim().to_string();

            let num = arg_str.parse::<u128>().unwrap();
            let set_bits = count_set_bits(num.try_into().unwrap());
            if cfg!(debug_assertions) {
                println!(" {}", set_bits);
            } else {
                println!("{}", set_bits);
            }
        }
        // one argument passed
        2 => {
            //match args[1].parse() {
            if args.len() == 2 {
                let arg_str = &args[1];
                let num: u128 = arg_str.parse::<u128>()?;
                let set_bits = count_set_bits(num.try_into().unwrap());
                if cfg!(debug_assertions) {
                    println!(" {}", set_bits);
                } else {
                    println!("{}", set_bits);
                }

                //Ok(42) => println!("This is the answer!"),
                //_ => println!("This is not the answer."),
            }
        }
        // one command and one argument passed
        3 => {
            let cmd = &args[1];
            let num = &args[2];
            // parse the number
            let number: i32 = match num.parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("error: second argument not an integer");
                    help();
                    0
                    //Ok(())
                    //return;
                }
            };
            // parse the command
            match &cmd[..] {
                "version" => version(),
                "-V" => version(),
                "-v" => version(),
                "--version" => version(),
                "help" => help(),
                "-h" => help(),
                "--help" => help(),
                "increase" => increase(number),
                "decrease" => decrease(number),
                _ => {
                    eprintln!("error: invalid command");
                    help();
                }
            }
        }
        // all the other cases
        _ => {
            // show a help message
            help();

            if args.len() > 1 {
                let arg_str = &args[1];
                let num: u128 = arg_str.parse::<u128>()?;
                let set_bits = count_set_bits(num.try_into().unwrap());
                if cfg!(debug_assertions) {
                    println!(" {}", set_bits);
                } else {
                    println!("{}", set_bits);
                }
            } else {
                /*drop into stdin*/

                let mut arg_str = String::new();
                let stdin = std::io::stdin();
                stdin.read_line(&mut arg_str).unwrap();
                arg_str = arg_str.trim().to_string();

                let num = arg_str.parse::<u128>().unwrap();
                let set_bits = count_set_bits(num.try_into().unwrap());
                if cfg!(debug_assertions) {
                    println!(" {}", set_bits);
                } else {
                    println!("{}", set_bits);
                }
            }
        }
    }

    Ok(())
}
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
