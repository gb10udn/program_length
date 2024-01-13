use std::io;
use std::io::prelude::*;
use std::fs::File;
use walkdir::WalkDir;


// tokei という Rust 製のソースコード長を取得するコマンドラインツールが使いやすかったのでそれを使う。(なので、これは練習用。)
// https://github.com/XAMPPRocky/tokei
// .vue ファイルのカウントが弱い issue が 2021 年に上がっているが、2024-01 時点で改善されていないので、その部分は作る価値があるかも？
// https://github.com/XAMPPRocky/tokei/issues/784
// あとは、関数とかクラスの数を数えても面白いかも？

fn main() {
    let user_input = obtain_user_input();
    let user_input = remove_head_and_tail_double_quotation(user_input);
    let extension = "rs";  // TODO: 240112 拡張子は外部から指定できるようにせよ。

    let flist = match retrieve_files(&user_input as &str, extension) {
        Some(val) => val,
        None => {
            println!("Error: No file exists (user_input: {}, extension: {})", user_input, extension);
            panic!();
        },
    };

    // for path in flist {
    //     if let Ok(text) = open_text_file(&path) {
    //         println!("{}", text);
    //     }
    // }

    // stop();  // TODO: 240113 本番では有効にする。
}


/// ユーザーの入力値を取得する関数。処理に使うベースディレクトリが入力される想定。
fn obtain_user_input() -> String {
    println!("Please enter the ''base directory'' to check the code length.");
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("failed to read line ...");
    input_string
}


/// 先頭と末尾のダブルクオーテーションがあれば削除する関数。
/// エクスプローラーでフォルダ右クリックしてパスのコピーすると、先頭と末尾にダブルクオーテーションが付くのだが、それを除去する目的。
fn remove_head_and_tail_double_quotation(arg: String) -> String {
    let mut result = arg.trim().to_string();  // INFO: 240111 標準入力で取ると末尾に改行コードが付いてるようで、それを除去するために .trim() を実施した。
    if result.starts_with("\"") {
        result.remove(0);
    }
    if result.ends_with("\"") {
        result.pop();
    }
    result.to_string()
}


/// base_dir 配下の、拡張子が extension のファイルのリストを取得する。
/// HACK: 240113 ネストが深くて読みにくいので、修正せよ。
fn retrieve_files(base_dir: &str, target_extension: &str) -> Option<Vec<String>> {  // TODO: 240112 複数の拡張子にも対応すること (１つのプロジェクトから、拡張子を複数とかできないのかな？)
    let mut result: Vec<String> = Vec::new();
    for entry in WalkDir::new(base_dir) {  // FIXME: 240112 .max_depth() を設定しない場合、どれくらいの階層まで探すかよくわからない。
        if let Ok(val) = entry {
            if val.file_type().is_file() { // INFO: 240108 .extension() といいながらも、hoge.txt というフォルダでも、txt を取得してしまうため、.is_file() によるチェックを入れた。
                if let Some(extension) = val.path().extension() {
                    if extension == target_extension {  // INFO: 240108 同様にエクセルマクロファイルを取得するようにする。
                        result.push(val.path().display().to_string());
                    }
                }
            }
        }
    }
    if result.len() > 0 {
        return Some(result);
    } else {
        return None;
    }
}


#[allow(dead_code)]  // INFO: 240113 将来用。改行コード等をカウントする用途。
fn open_text_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut result = String::new();
    f.read_to_string(&mut result)?;
    Ok(result)
}


fn count_row_num(path: &str) -> Result<usize, io::Error> {
    let f = File::open(path)?;
    let buf = std::io::BufReader::new(f);  // INFO: 240113 BufReader を使うと処理高速らしい。
    Ok(buf.lines().count())
}


fn _stop() {
    println!("");
    println!("finished !!! Please input enter key");
    let mut a = String::new();
    let _  = io::stdin().read_line(&mut a).expect("");
}


#[cfg(test)]
mod tests {
    const TEST_PATH: &str = "./misc/test1.txt";
    #[test]
    fn test_retrieve_files() {
        use crate::retrieve_files;
        let results = retrieve_files(".\\src", "rs").unwrap();  // FIXME: 240112 開発が進み、main.rs 以外にファイルが増えた場合に修正が必要。
        assert_eq!(results, vec!(".\\src\\main.rs"));
    }

    #[test]
    fn test_open_text_file() {
        use crate::open_text_file;
        assert_eq!(open_text_file(TEST_PATH).unwrap(), String::from("あいうえお"));  // HACK: 240112 ./misc にテキストファイルを入れてテストが微妙な気もするので、何か考えるべきかも？
    }

    #[test]
    fn test_count_row_num() {
        use crate::count_row_num;
        assert_eq!(count_row_num(TEST_PATH).unwrap(), 1);
    }
}