use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("rpx2json - Public Edition");

    if args.len() < 2 {
        println!("Usage:");
        println!("  rpx2json <input.rpx>");
        return;
    }

    let input = &args[1];

    println!("Input file: {}", input);
    println!("Note: Full conversion engine is not included in this public repository.");
}
