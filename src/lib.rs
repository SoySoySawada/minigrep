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

    for line in search(&config.query, &contents){
        println!("{}", line);
    }

    // このように返り値をユニット型で返すのは、
    // 関数が副作用のためだけに呼び出されていることを示唆する慣習的な方法
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result:Vec<&str> = Vec::new();

    // lines()を使用し、contentsを行ごとに分割
    for line in contents.lines(){
        if line.contains(query){
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn config_new() {
        let args = vec![
            String::from("target/debug/minigrep"),
            String::from("query"),
            String::from("filename"),
        ];
        let config = Config::new(&args).unwrap();
        assert_eq!(config.query, "query", "Config::new made structure property error (query)");
        assert_eq!(config.filename, "filename", "Config::new made structure property error (filename)");
    }

    #[test]
    fn config_new_args_less() {
        let args = vec![
            String::from("target/debug/minigrep"),
        ];
        let config = Config::new(&args);
        assert!(config.is_err(), "Config::new could not detect args less{:?}", args);

        let args2 = vec![
            String::from("target/debug/minigrep"),
            String::from("query"),
        ];
        let config = Config::new(&args2);
        assert!(config.is_err(), "Config::new could not detect args less{:?}", args2);
    }

    /// まずは空の関数、失敗するテスト(目指す結果)を記述する。
    #[test]
    fn search_one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

}