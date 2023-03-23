const CLEAR: &str = "\x1B[2J\x1B[1;1H";

fn expensive_calculation(_: &i32) {
    std::thread::sleep(std::time::Duration::from_secs(1));
}

fn progress<T>(v: Vec<T>, f: fn(&T) -> ()) {
    let mut i = 0;

    for n in v.iter() {
        println!("{}{}", CLEAR, "*".repeat(i + 1));
        i += 1;
        f(n);
    }
}

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6];

    progress(data, expensive_calculation);
}
