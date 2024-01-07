fn main() {
    if let Err(err) = rust_reader::run(rust_reader::get_args()) {
        eprintln!("An error occurred: {}", err);
        std::process::exit(1);
    }
}
