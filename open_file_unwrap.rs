fn main() {
    let paths = std::fs::read_dir("/home/user").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display());

    }
}
