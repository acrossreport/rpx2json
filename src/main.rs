use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // ① バージョン表示
    println!("rpx2json v0.1.0");

    // ② help対応
    if args.len() < 2 || args[1] == "--help" {
        println!();
        println!("Usage:");
        println!("  rpx2json <input.rpx>");
        println!();
        println!("Description:");
        println!("  Convert RPX file to JSON format (public edition)");
        return;
    }

    let input = &args[1];

    println!();
    println!("Input file: {}", input);

    // ③ ダミーJSON出力
    println!();
    println!("{{");
    println!("  \"status\": \"public edition\"");
    println!("}}");

    // ④ 内部エンジン非公開メッセージ
    println!();
    println!("Note: Full conversion engine is not included in this public repository.");
}
