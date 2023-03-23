const CLEAR: &str = "\x1B[2J\x1B[1;1H";

struct Progress<I> {
    iter: I,
    i: usize
}

impl<I> Progress<I> {
    fn new(iter: I) -> Self {
        Progress { iter, i: 0 }
    }
}

impl<I> Iterator for Progress<I>
    where I: Iterator
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        println!("{}{}", CLEAR, "*".repeat(self.i + 1));
        self.i += 1;
        self.iter.next()
    }
}

trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self>;
}

impl<I> ProgressIteratorExt for I
    where I: Iterator
{
    fn progress(self) -> Progress<Self> {
        Progress::new(self)
    }
}

fn expensive_calculation(_: &i32) {
    std::thread::sleep(std::time::Duration::from_secs(1));
}

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6];

    for n in data.iter().progress() {
        expensive_calculation(n);
    }
}
