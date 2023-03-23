const CLEAR: &str = "\x1B[2J\x1B[1;1H";

fn expensive_calculation(_: &i32) {
    std::thread::sleep(std::time::Duration::from_secs(1));
}

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6];

    let mut i = 0;

    for n in data.iter() {
        println!("{}{}", CLEAR, "*".repeat(i + 1));
        i += 1;
        expensive_calculation(n);
    }
}
