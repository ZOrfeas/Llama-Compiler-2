use core::panic;

mod cli;
mod scan;

fn main() {
    let mut scanner = scan::Scanner::new()
        .add_file("./tests/input.lla")
        .unwrap()
        .preprocess();
    loop {
        match scanner.read_line() {
            Ok(Some(item)) => print!("{}", item),
            Ok(None) => {
                print!("\nEOF");
                break;
            }
            Err(e) => panic!("{:?}", e),
        }
    }
}
