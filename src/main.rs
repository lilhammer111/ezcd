mod path_parser;


use std::env;

fn main() {
    println!("hello, ezcd");
    for arg in env::args().skip(1) {
        println!("Received arg: {}", arg);
    }
}
