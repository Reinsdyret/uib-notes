fn main() {
    for i in 0..3 {
        println!("{}", i);
        print_twice(&i.to_string());
    }
}

fn print_twice(word: &str) {
    println!("{}, {}", word, word);
}
