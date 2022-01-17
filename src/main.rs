fn main() {
    if let Err(e) = cat_rust::get_args().and_then(cat_rust::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
