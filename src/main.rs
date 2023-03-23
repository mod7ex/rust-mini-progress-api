const CLEAR: &str = "\x1B[2J\x1B[1;1H";

// States
struct Unbounded;
struct Bounded {
    bound: usize,
    delimiters: (char, char)
}

struct Progress<I: Iterator, B> {
    iter: I,
    i: usize,
    bound: B
}

trait ProgressDisplay: Sized {
    fn display<I: Iterator>(&self, progress: &Progress<I, Self>);
}

impl ProgressDisplay for Unbounded {
    fn display<I: Iterator>(&self, progress: &Progress<I, Self>) {
        println!("{}{}", CLEAR, "*".repeat(progress.i + 1));
    }
}

impl ProgressDisplay for Bounded {
    fn display<I: Iterator>(&self, progress: &Progress<I, Self>) {
        println!(
            "{}{}{}{}{}",
            CLEAR, 
            self.delimiters.0,
            "*".repeat(progress.i), 
            " ".repeat(self.bound - progress.i),
            self.delimiters.1
        );
    }
}

impl<I> Progress<I, Unbounded>
    where I: Iterator
{
    fn new(iter: I) -> Self {
        Progress { iter, i: 0, bound: Unbounded }
    }
}

impl<I> Progress<I, Unbounded>
    where I: ExactSizeIterator
{
    fn with_bound(self) -> Progress<I, Bounded> {
        let bound = self.iter.len();
        Progress {
            i: self.i,
            iter: self.iter,
            bound: Bounded {
                bound,
                delimiters: ('[', ']')
            }
        }
    }
}

impl<I> Progress<I, Bounded>
    where I: Iterator
{
    fn with_delimiters(mut self, delimiters: (char, char)) -> Self {
        self.bound.delimiters = delimiters;
        self
    }
}

impl<I ,B> Iterator for Progress<I, B>
    where I: Iterator, B: ProgressDisplay
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.bound.display(&self);
        self.i += 1;
        self.iter.next()
    }
}

trait ProgressIteratorExt: Sized + Iterator {
    fn progress(self) -> Progress<Self, Unbounded>;
}

impl<I> ProgressIteratorExt for I
    where I: Iterator
{
    fn progress(self) -> Progress<Self, Unbounded> {
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

    // we can use <with_delimiters> only if <with_bound> is used
    for n in data.iter().progress().with_bound().with_delimiters((':', ':')) {
        expensive_calculation(n);
    }
}
