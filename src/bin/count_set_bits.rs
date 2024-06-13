use count_set_bits::count_set_bits;
use std::env;
use std::error::Error;

fn version() {
    let version = env!("CARGO_PKG_VERSION");
    let crate_name = env!("CARGO_CRATE_NAME");
    println!("{} v{}", crate_name.replace("_", "_"), version);
    std::process::exit(0);
}
fn increase(number: u128) {
    println!("{}", number + 1);
    std::process::exit(0);
}

fn decrease(number: u128) {
    println!("{}", number - 1);
    std::process::exit(0);
}

fn help() {
    let crate_name = env!("CARGO_CRATE_NAME");
    let version = env!("CARGO_PKG_VERSION");
    print!("{} v{}\n", crate_name.replace("_", "_"), version);
    println!(
        "usage:
{} <string>
    Check whether given string is the answer.
{} {{increase|decrease}} <integer>
    Increase or decrease given integer by one.",
        crate_name, crate_name
    );
    std::process::exit(0);
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        // no arguments passed
        // drop into stdin
        1 => {
            let mut arg_str = String::new();
            let stdin = std::io::stdin();
            stdin.read_line(&mut arg_str).unwrap();
            arg_str = arg_str.trim().to_string();
            let s = arg_str.clone();
            if let [arg1, _arg2 @ ..] = &s.split(" ").map(String::from).collect::<Vec<String>>()[..]
            {
                if cfg!(debug_assertions) {
                    //print!("&arg1={}\n",&arg1);
                    //print!("&_arg2={:?}\n",&_arg2);
                }
                match &arg1[..] {
                    "version" => version(),
                    "-V" => version(),
                    "-v" => version(),
                    "--version" => version(),
                    "help" => help(),
                    "-h" => help(),
                    "--help" => help(),
                    //"increase" => increase(number),
                    //"decrease" => decrease(number),
                    _ => {
                        //let num: u128 = arg_str.parse::<u128>()?;
                        let num: u128 = arg1.parse::<u128>()?;
                        let set_bits = count_set_bits(num.try_into().unwrap(), true);
                        //if cfg!(debug_assertions) {
                        //println!(" {}", set_bits);
                        //} else {
                        println!(" {}", set_bits);
                        //}
                    }
                }
            };
        }
        // one argument passed
        2 => {
            //match args[1].parse() {
            if args.len() == 2 {
                let arg_str = &args[1];
                //handle arg as command
                match &arg_str[..] {
                    "version" => version(),
                    "-V" => version(),
                    "-v" => version(),
                    "--version" => version(),
                    "help" => help(),
                    "-h" => help(),
                    "--help" => help(),
                    _ => {
                        let num: u128 = arg_str.parse::<u128>()?;
                        let set_bits = count_set_bits(num.try_into().unwrap(), true);
                        if cfg!(debug_assertions) {
                            println!(" {}", set_bits);
                        } else {
                            println!("{}", set_bits);
                        }
                    }
                }
            }
        }
        // one command and one argument passed
        3 => {
            let cmd = &args[1];
            //TODO more command handling
            let num = &args[2];
            // parse the number
            let number: u128 = match num.parse() {
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
                let set_bits = count_set_bits(num.try_into().unwrap(), true);
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
                let set_bits = count_set_bits(num.try_into().unwrap(), true);
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
