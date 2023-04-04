use std::env;

fn main() {
    // env::args()関数で、コマンドライン引数のイテレータを取得
    // collect()にて、それをコレクションに変換
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
