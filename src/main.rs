fn main() {
    if let Err(err) = rust_reader::run() {
        eprintln!("An error occurred: {}", err);
        std::process::exit(1);
    }
}
