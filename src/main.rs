fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let code = match args.split_first() {
        Some((pattern, rest)) => match pattern.as_str() {
            "1" => {
                println!("パターン1は未実装");
                0
            }
            "2" => {
                println!("パターン2は未実装");
                0
            }
            "3" => {
                println!("パターン3は未実装");
                0
            }
            _ => usage(),
        },
        None => usage(),
    };

    std::process::exit(code);
}

fn usage() -> i32 {
    eprintln!("使い方: cargo run -- <1|2|3> <コマンド> [引数...]");
    eprintln!();
    eprintln!("  1 = トランザクションスクリプト");
    eprintln!("  2 = アクティブレコード");
    eprintln!("  3 = ドメインモデル");
    eprintln!();
    eprintln!("コマンド: ");
    eprintln!("  list                       ノートの一覧");
    eprintln!("  create  <ノートID>         ノートを作る");
    eprintln!("  add-tag <ノートID> <タグ>  ノートにタグを追加");
    eprintln!("  archive <ノートID>         ノートをアーカイブする");
    1
}
