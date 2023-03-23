const CLEAR: &str = "\x1B[2J\x1B[1;1H";

struct Progress<I: Iterator> {
    iter: I,
    i: usize,
    bound: Option<usize>,
    delimiters: (char, char)
}

impl<I> Progress<I>
    where I: Iterator
{
    fn new(iter: I) -> Self {
        Progress { iter, i: 0, bound: None, delimiters: ('<', '>') }
    }

    fn with_delimiters(mut self, delimiters: (char, char)) -> Self {
        self.delimiters = delimiters;
        self
    }
}

impl<I> Progress<I>
    where I: ExactSizeIterator
{
    fn with_bound(mut self) -> Self {
        self.bound = Some(self.iter.len());
        self
    }
}

impl<I> Iterator for Progress<I>
    where I: Iterator
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        print!("{}", CLEAR);

        match &self.bound {
            Some(bound) => {
                println!(
                    "{}{}{}{}", 
                    self.delimiters.0,
                    "*".repeat(self.i), 
                    " ".repeat(bound - self.i),
                    self.delimiters.1
                );
            }
            None => {
                println!("{}", "*".repeat(self.i + 1));
            }
        }
        self.i += 1;
        self.iter.next()
    }
}

trait ProgressIteratorExt: Sized + Iterator {
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
/*
    for n in (0..).progress() {
        expensive_calculation(&n);
    }
*/

    let data = vec![1, 2, 3, 4, 5, 6];

    for n in data.iter().progress() {
        expensive_calculation(n);
    }

    for n in data.iter().progress().with_bound().with_delimiters((':', ':')) {
        expensive_calculation(n);
    }
}
