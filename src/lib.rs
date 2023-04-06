use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config{
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

/// ファイル操作
/// エラー時はエラーのトレイトオブジェクトを返すよう修正
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    // ファイルのオープンに失敗
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:");
    println!("{}", contents);

    // このように返り値をユニット型で返すのは、
    // 関数が副作用のためだけに呼び出されていることを示唆する慣習的な方法
    Ok(())
}