use std::env;
use std::io::Read;
use std::fs::File;

pub fn get_parameters() -> Vec<String> {
    // png photo
    // mode [encode || decode ] 
    // encode ? message
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!("usage: pngme photo encode|decode (message for encoding/ not required for decoding)");
        std::process::exit(0);
    }


    args 
}

