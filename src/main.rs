// ライブラリクレートをバイナリクレートに持ってくるため
extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

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
    
    // UPD START Configでの抽出
    // let (query, filename) = parse_config(&args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        // eprintlnを使用すると、標準エラー出力に出力される(> output.txtとしてもoutput.txtに出力されない)
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // UPD END


    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    println!();

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}
