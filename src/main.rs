use std::env;
use std::fs::File;
use std::io::prelude::*;

/// バイナリプロジェクトの責任の分離
/// プログラムをmain.rsとlib.rsに分け、ロジックをlib.rsに移動。
/// コマンドライン引数の解析ロジックが小規模なら、main.rsに置いてよい。
/// ただし、コマンドライン引数の解析ロジックが複雑化の様相を呈し始めたら、main.rsから抽出してlib.rsに移動する。
/// 
/// main関数に残る責任を以下に限定する
/// ・引数の値でコマンドライン引数の解析ロジックを呼び出す
/// ・他のあらゆる設定を行う
/// ・lib.rsのrun関数を呼び出す
/// ・runがエラーを返したときに処理する
fn main() {
    // env::args()関数で、コマンドライン引数のイテレータを取得
    // collect()にて、それをコレクションに変換
    let args: Vec<String> = env::args().collect();
    
    let (query, filename) = parse_config(&args);

    println!("Searching for {}", query);
    println!("In file {}", filename);

    // ファイルのオープンに失敗
    let mut f = File::open(filename).expect("File open error");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    println!("{}", contents);
}

/// 引数解析器の抽出
/// 1番目の引数と2番目の引数を抽出する
fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}