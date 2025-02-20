use std::io::{self, BufRead, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    
    let binding = lines.next().unwrap().unwrap();
    let s = binding.trim().as_bytes();

    let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..n {
        let items: Vec<usize> = lines.next().unwrap().unwrap().trim().split_whitespace().map(|a| a.parse().unwrap()).collect();
        let (mut a, mut b) = (items[0], items[1]);

        let mut count = 0;
        while b < s.len() && unsafe {*s.get_unchecked(a) == *s.get_unchecked(b)} {
            count += 1;
            a += 1;
            b += 1;
        }

        writeln!(writer, "{}", count).unwrap();
    }

    writer.flush().unwrap();
}