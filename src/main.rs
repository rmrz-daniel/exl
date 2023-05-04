use std::env::args;

mod exl;

#[allow(dead_code)]
pub struct Arguments {
    file: String,
    options: String,
}

fn main() {

    if args().len() >= 2 {
        exl::run(Some(parse_args()));
    } else {
        exl::run(None);
    }
}

fn parse_args() -> Arguments{
    Arguments { 
        file: args().nth(1).unwrap(),
        options: "s".to_string() 
    }
}

