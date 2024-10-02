fn main() {
    //cli args
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    dbg!(args);
}
