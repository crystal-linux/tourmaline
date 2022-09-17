use std::env::args;

fn main() {
    let script_path = args().skip(1).next().unwrap();
    tourmaline::test_execute(script_path);
}
