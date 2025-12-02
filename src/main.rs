mod storage;
mod tracker;
mod report;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "run" => tracker::run(5),
        "report" => {
            if args.len() < 3 {
                println!("Usage: routine report <daily|weekly|monthly>");
            } else {
                report::generate(&args[2]);
            }
        }
        _ => print_help(),
    }
}

fn print_help() {
    println!("Usage:");
    println!("  routine run");
    println!("  routine report <daily|weekly|monthly>");
}
