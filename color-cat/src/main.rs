use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use std::io;

struct Ansi {
    step: Uniform<i32>,
    rng: ThreadRng,
}

impl Ansi {
    fn sample(&mut self) -> i32 {
        self.step.sample(&mut self.rng)
    }

    fn rand_truecolor(&mut self, s: String) -> String {
        format!(
            "\x1b[38;2;{};{};{}m{}\x1b[0m",
            self.sample(),
            self.sample(),
            self.sample(),
            s,
        )
    }

    fn new() -> Self {
        Self {
            step: Uniform::new(30, 205),
            rng: rand::thread_rng(),
        }
    }
}

fn main() {
    let mut ansis = Ansi::new();
    let stdin_handle = io::stdin();

    loop {
        let mut buf = String::new();

        if let Ok(len) = stdin_handle.read_line(&mut buf) {
            if len == 0 {
                break;
            }
            let col_str = ansis.rand_truecolor(String::from(buf.trim()));
            println!("{}", col_str);
        }
    }
}
