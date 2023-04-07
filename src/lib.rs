use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config{
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive: bool;
        if args.len() >= 4 {
            if args[3] == "0" {
                case_sensitive = false;
            }else {
                case_sensitive = true;
            }
        }else{
            // 環境変数を取得
            case_sensitive = std::env::var("CASE_INSENSITIVE").is_err();
        }
        
        println!("case_sensitive: {}", case_sensitive);

        Ok(Config { query, filename, case_sensitive })
    }
}

/// ファイル操作
/// エラー時はエラーのトレイトオブジェクトを返すよう修正
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    // ファイルのオープンに失敗
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive == true {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results{
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

/// 大文字小文字を区別しない検索
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result:Vec<&str> = Vec::new();

    // lines()を使用し、contentsを行ごとに分割
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
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

    #[test]
    fn config_new_caseinsensitive(){
        let args = vec![
            String::from("target/debug/minigrep"),
            String::from("query"),
            String::from("filename"),
            String::from("0"),
        ];
        let config = Config::new(&args).unwrap();
        assert_eq!(config.query, "query", "Config::new made structure property error (query)");
        assert_eq!(config.filename, "filename", "Config::new made structure property error (filename)");
        assert_eq!(config.case_sensitive, false, "Config::new made structure property error (case_sensitive)");

        let args = vec![
            String::from("target/debug/minigrep"),
            String::from("query"),
            String::from("filename"),
            String::from("1"),
        ];
        let config = Config::new(&args).unwrap();
        assert_eq!(config.query, "query", "Config::new made structure property error (query)");
        assert_eq!(config.filename, "filename", "Config::new made structure property error (filename)");
        assert_eq!(config.case_sensitive, true, "Config::new made structure property error (case_sensitive)");
    }

    /// まずは空の関数、失敗するテスト(目指す結果)を記述する。
    #[test]
    fn search_case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Duct tape
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    /// 大文字・小文字無視を実装したときのテストを記述
    #[test]
    fn search_test_case_insensitive(){
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Duct tape
Trust me
Pick three.";

        assert_eq!(
            vec!["Rust:", "Trust me"],
            search_case_insensitive(query, contents)
        );
    }

}