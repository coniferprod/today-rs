fn main() {
    if let Err(e) = today::run() {
        eprintln!("Error");
    }
}
