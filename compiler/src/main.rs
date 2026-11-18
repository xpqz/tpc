fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    std::process::exit(tp5c::driver::run(&args));
}
