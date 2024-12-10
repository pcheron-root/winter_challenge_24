fn main() {
    let args: Vec<String> = std::env::args().collect();
    let _n = args
        .get(1)
        .expect("Give one argument")
        .parse::<u32>()
        .expect("Given argument should be a integer.");
}
