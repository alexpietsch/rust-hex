use std::env;
use std::str::from_utf8;
use hex::encode;
use hex::decode;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: No arguments given. Available options:");
        println!("help: shows the help text");
        println!("-d <value>: decode hex to string");
        println!("-e <vaule>: encode string to hex");

        return;
    }

    if args.len() < 3 {
        println!("Error: No value given. Available options:");
        println!("help: shows the help text");
        println!("-d <value>: decode hex to string");
        println!("-e <vaule>: encode string to hex");

        return;
    }

    let option = &args[1];
    let input_value = &args[2];

    match option.as_str() {
        "help" => {
            println!("help: shows this help");
            println!("-d <value>: decode hex to string");
            println!("-e <vaule>: encode string to hex");
        },
        "-d" => {
            let value = decode(input_value).unwrap();
            let hex_to_value = from_utf8(&value).unwrap();
            println!("{}", hex_to_value);
        },
        "-e" => {
            let value_to_hex = encode(input_value);
            println!("{}", value_to_hex);
        },
        _ => {
            println!("Error! Available options:");
            println!("help: shows the help text");
            println!("-d <value>: decode hex to string");
            println!("-e <vaule>: encode string to hex");
        }
    }
}
