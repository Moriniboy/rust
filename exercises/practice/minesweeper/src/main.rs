mod solution_with_file_io;

use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Usage: {} <file>", args[0]);
        return;
    }

    let file = args[1].clone();

    let result = solution_with_file_io::handle_minefield(&file);

    println!("{:?}", result);
}