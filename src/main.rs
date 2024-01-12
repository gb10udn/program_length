use std::io;
use walkdir::WalkDir;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    let user_input = obtain_user_input();
    let user_input = remove_head_and_tail_double_quotation(user_input);
    println!("user_input: {}", user_input);  // FIXME: 240112 remove me !!!

    let path_list = retrieve_files(&user_input as &str, "rs");  // TODO: 240112 拡張子は外部から指定できるようにせよ。
    println!("path_list: {:?}", path_list);  // FIXME: 240112 remove me !!!

    stop();
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
fn remove_head_and_tail_double_quotation(arg: String)  -> String {
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


fn open_text_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut result = String::new();
    
    f.read_to_string(&mut result)?;
    Ok(result)
}


fn stop() {
    println!("");
    println!("finished !!! Please input enter key");
    let mut a = String::new();
    let _  = io::stdin().read_line(&mut a).expect("");
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_retrieve_files() {
        use crate::retrieve_files;
        let results = retrieve_files(".\\src", "rs").unwrap();  // FIXME: 240112 開発が進み、main.rs 以外にファイルが増えた場合に修正が必要。
        assert_eq!(results, vec!(".\\src\\main.rs"));
    }

    #[test]
    fn test_open_text_file() {
        use crate::open_text_file;
        let result = open_text_file(".\\misc\\test1.txt").unwrap();  // HACK: 240112 ./misc にテキストファイルを入れてテストが微妙な気もするので、何か考えるべきかも？
        assert_eq!(result, String::from("あいうえお"));
    }
}