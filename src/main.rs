use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use walkdir::WalkDir;
use tabled::{Table, Tabled, settings::Style};


// tokei という Rust 製のソースコード長を取得するコマンドラインツールが使いやすかったのでそれを使う。
// https://github.com/XAMPPRocky/tokei
// .vue ファイルのカウントが弱い issue が 2021 年に上がっているが、2024-01 時点で改善されていないので、その部分は作る価値があるかも？
// https://github.com/XAMPPRocky/tokei/issues/784
// あとは、関数とかクラスの数を数えても面白いかも？ 

fn main() {
    let user_input = obtain_user_input();
    let user_input = remove_head_and_tail_double_quotation(user_input);
    let extensions = vec!["rs", "py", "vue", "js"];

    let path_info = match retrieve_path(&user_input, &extensions) {
        Some(val) => val,
        None => {
            println!("No file exists (user_input: {}, extension: {:?})", user_input, extensions);
            return
        },
    };

    let mut summaries = vec![];  // HACK: 240113 以下の塊を別の関数にして、テスト可能な形式にするといいかも？
    for ext in extensions {
        if let Some(flist) = path_info.get(ext) {
            let mut total_code_length: usize = 0;
            let total_file_num = flist.len();
            for path in flist {
                if let Ok(code_length) = count_row_num(&path) {
                    total_code_length += code_length;
                }
            };
            let summary = Summary{
                extension: ext.to_string(),
                total_file_num: total_file_num,
                total_code_length: total_code_length,
            };
            summaries.push(summary)
        }
    }

    let mut table = Table::new(summaries);  // TODO: 240113 tauri の練習にしてもいいかも？
    table.with(Style::markdown());
    print!("\n\n{}\n\n", table.to_string());

    // TODO: 240113 ファイル別の長さも取得できるようにすると便利かも？

    stop();
}


#[derive(Tabled)]
struct Summary {
    extension: String,
    total_file_num: usize,
    total_code_length: usize,
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


fn retrieve_path<'a>(base_dir: &'a str, target_extensions: &Vec<&'a str>) -> Option<HashMap<String, Vec<String>>> {
    let mut result = HashMap::new();
    for entry in WalkDir::new(base_dir) {
        if let Ok(val) = entry {
            if val.file_type().is_file() { // INFO: 240108 .extension() といいながらも、hoge.txt というフォルダでも、txt を取得してしまうため、.is_file() によるチェックを入れた。
                if let Some(extension) = val.path().extension() {
                    let extension = extension.to_str().unwrap();  // FIXME: 240114 unwrap() に失敗するケースを記述しきれていない。
                    if target_extensions.contains(&extension) {
                        result.entry(extension.to_string()).or_insert(Vec::new()).push(val.path().display().to_string());
                    }
                }
            }
        }
    }
    if result.len() > 0 {
        Some(result)
    } else {
        None
    }
}


#[allow(dead_code)]  // TODO: 240113 将来用。改行コード等をカウントする用途。
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


fn stop() {
    println!("");
    println!("finished !!! Please input enter key");
    let mut a = String::new();
    let _  = io::stdin().read_line(&mut a).expect("");
}


#[cfg(test)]
mod tests {
    const TEST_PATH: &str = ".\\misc\\test1.txt";
    const TEST_DIR: &str = ".\\misc";

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

    #[test]
    fn test_retrieve_path() {
        use crate::retrieve_path;
        use std::collections::HashMap;

        let result = retrieve_path(TEST_DIR, &vec!["txt", "py"]).unwrap();

        let mut expect = HashMap::new();
        expect.insert("txt".to_string(), vec![".\\misc\\test1.txt".to_string()]);
        expect.insert("py".to_string(), vec![".\\misc\\test2.py".to_string()]);
        assert_eq!(result, expect);
    }
}