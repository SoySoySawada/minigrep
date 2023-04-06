use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::error::Error;

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
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // UPD END


    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config);
}

struct Config {
    query: String,
    filename: String,
}

impl Config{
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

// DEL START 抽出メソッドをConfig構造体のnew関数に移動
// /// 引数解析器の抽出
// /// 1番目の引数と2番目の引数を抽出する
// fn parse_config(args: &[String]) -> (&str, &str) {
//     let query = &args[1];
//     let filename = &args[2];
//     (query, filename)
// }
// DEL END

// ADD START run関数の抽出
/// ファイル操作
fn run(config: Config) {
    // ファイルのオープンに失敗
    let mut f = File::open(config.filename).expect("File open error");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    println!("{}", contents);
}
// ADD END