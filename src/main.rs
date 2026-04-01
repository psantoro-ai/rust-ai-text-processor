use std::io;

fn main() {
    println!("=== AI Text Processor ===\n");

    println!("Enter a text:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let word_count = input.split_whitespace().count();
    let char_count = input.trim().chars().count();

    println!("\nResults:");
    println!("Words: {}", word_count);
    println!("Characters: {}", char_count);
}
