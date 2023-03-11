fn main() {
    let file = std::fs::read_to_string("lines").unwrap();
    file.lines()
        .forEach(|line| println!("{}", line));
}