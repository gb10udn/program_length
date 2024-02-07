use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use walkdir::WalkDir;
use tabled::{Table, Tabled, settings::Style};


fn main() {
    // [START] parameters
    let extensions = vec!["rs", "py", "vue", "js", "html", "css"];  // HACK: 240114 config ファイルから選べるようにする？ (or 全テキストファイルを対象とする？)
    let ignore_hidden_directory = true;
    // [END] parameters

    let base_dir = obtain_base_dir_from_user_input();

    let path_info: HashMap<String, Vec<String>> = match retrieve_path_info(&base_dir, &extensions, ignore_hidden_directory) {
        Some(val) => val,
        None => {
            println!("No file exists (user_input: {}, extension: {:?})", base_dir, extensions);
            return
        },
    };

    let each_files = retrieve_each_files(&path_info);

    // EDIT: 240206 each_files, summaries の取得を関数に切り出す。
    let mut summaries = vec![];
    let mut each_files = vec![];
    for ext in extensions {
        if let Some(flist) = path_info.get(ext) {
            let mut total_code_length: usize = 0;
            let total_file_num = flist.len();
            for path in flist {
                if let Ok(code_length) = count_row_num(&path) {
                    total_code_length += code_length;
                    let each_file = EachFile {
                        extension: ext.to_string(),
                        path: path.to_string(),
                        code_length: code_length,
                        code_length_: "".to_string(),
                    };
                    each_files.push(each_file);
                }
            };
            let summary = Summary {
                extension: ext.to_string(),
                total_file_num: total_file_num,
                total_code_length: total_code_length,
            };
            summaries.push(summary)
        }
    }

    let mut table_summary = Table::new(summaries);
    let mut table_each_files = Table::new(each_files);
    table_summary.with(Style::markdown());
    table_each_files.with(Style::markdown());
    print!("\n\n{}\n\n", table_summary.to_string());
    print!("\n\n{}\n\n", table_each_files.to_string());

    stop();
}


#[derive(Tabled)]
struct Summary {
    extension: String,
    total_file_num: usize,
    total_code_length: usize,
}

#[derive(Tabled)]
struct EachFile {
    extension: String,
    path: String,
    code_length: usize,
    code_length_: String,
}


fn obtain_base_dir_from_user_input() -> String {
    println!("Please enter the ''base directory'' to check the code length.");
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("failed to read line ...");

    input_string = _remove_head_and_tail_double_quotation(input_string);
    if input_string.len() == 0 {
        input_string = String::from(".");  // INFO: 240114 if no input (.len() == 0), change to "." (directly below the executable file)
    }
    input_string
}


/// 先頭と末尾のダブルクオーテーションがあれば削除する関数。
/// エクスプローラーでフォルダ右クリックしてパスのコピーすると、先頭と末尾にダブルクオーテーションが付くのだが、それを除去する目的。
fn _remove_head_and_tail_double_quotation(arg: String) -> String {
    let mut result = arg.trim().to_string();  // INFO: 240111 標準入力で取ると末尾に改行コードが付いてるようで、それを除去するために .trim() を実施した。
    if result.starts_with("\"") {
        result.remove(0);
    }
    if result.ends_with("\"") {
        result.pop();
    }
    result.to_string()
}


/// base_dir 配下で、target_extensions の拡張子を再帰的に検索し、ファイルパスを取得する関数。
/// 不可視ファイル (Ex. .hoge.py) は対象外。
fn retrieve_path_info<'a>(base_dir: &'a str, target_extensions: &Vec<&'a str>, ignore_hidden_directory: bool) -> Option<HashMap<String, Vec<String>>> {
    let mut result = HashMap::new();
    for entry in WalkDir::new(base_dir) {
        if let Ok(val) = entry {
            if val.file_type().is_file() { // INFO: 240108 .extension() といいながらも、hoge.txt というフォルダでも、txt を取得してしまうため、.is_file() によるチェックを入れた。
                if let Some(extension) = val.path().extension() {
                    let extension = extension.to_str().unwrap();  // FIXME: 240114 unwrap() に失敗するケースを記述しきれていない。
                    if target_extensions.contains(&extension) {
                        let path = val.path().display().to_string();
                        if ignore_hidden_directory == false || (ignore_hidden_directory == true && include_hidden_directory(&path) == false) {
                            result.entry(extension.to_string()).or_insert(Vec::new()).push(path);
                        }
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


// EDIT: 240206 each_files 取得を別関数として独立させる
fn retrieve_each_files(path_info: &HashMap<String, Vec<String>>) -> Vec<EachFile> {
    let mut max_code_length = 0;
    let mut each_files = vec![];
    for ext in path_info.keys() {
        if let Some(flist) = path_info.get(ext) {
            for path in flist {
                if let Ok(code_length) = count_row_num(&path) {
                    let each_file = EachFile {
                        extension: ext.to_string(),
                        path: path.to_string(),
                        code_length: code_length,
                        code_length_: "".to_string(),  // INFO: 240206 "" is temporary value 
                    };
                    each_files.push(each_file);
                    if code_length > max_code_length {
                        max_code_length = code_length;
                    }
                }
            };
        }
    }
    println!("{}", max_code_length);

    // TODO: 240206 以下で、ビジュアル化された、code_length (= code_length_) を再計算してグラフィカルに出力する。
    // for file in each_files {
    //     println!("");
    // }

    each_files
}


// EDIT: 240206 each_files から、summary を作る関数を独立させる。
fn retrieve_summaries_from_each_files(each_files: &EachFile) {
    // let mut summaries = vec![];

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


fn include_hidden_directory(path: &str) -> bool {
    let path = path.replace("/", "\\");
    let list: Vec<&str> = path.split('\\').collect();
    for dir in list {
        if (dir.starts_with(".") == true) && (dir.len() > 1) {
            if dir.starts_with("..") == false {
                return true
            }
            if dir.starts_with("..") == true && dir.len() > 2 {
                return true
            }
        }
    }
    return false;
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
        assert_eq!(open_text_file(TEST_PATH).unwrap(), String::from("あいうえお"));  // HACK: 240112 ./misc にテキストファイルを入れてテストが微妙な気もするので、何か考えるべきかも？ (tokei が参考になるかも？)
    }

    #[test]
    fn test_count_row_num() {
        use crate::count_row_num;
        assert_eq!(count_row_num(TEST_PATH).unwrap(), 1);
    }

    #[test]
    fn test_retrieve_path_info() {
        use crate::retrieve_path_info;
        use std::collections::HashMap;

        let result = retrieve_path_info(TEST_DIR, &vec!["txt", "py"], true).unwrap();

        let mut expect = HashMap::new();
        expect.insert("txt".to_string(), vec![".\\misc\\test1.txt".to_string()]);
        expect.insert("py".to_string(), vec![
            ".\\misc\\piyo1\\test4.py".to_string(),  // FIXME: 240205 Vec 型 は順番違うと別ものとされる。関係無いように修正必要。
            ".\\misc\\test2.py".to_string(),
        ]);
        assert_eq!(result, expect);
    }

    #[test]
    fn test_include_hidden_directory() {
        use crate::include_hidden_directory;

        let false_path_list = vec![
            "C:\\hoge\\fuga\\piyo.rs",
            "./hoge/piyo.rs",
            "../hoge/piyo.rs",
        ];
        for path in false_path_list {
            assert_eq!(include_hidden_directory(path), false);
        }

        let true_path_list = vec![
            "C:\\hoge\\fuga\\.piyo.rs",
            "C:\\hoge\\.fuga\\piyo.rs",
            "./.venv/Scripts/activate",
            "./.gitignore",
            "../.venv/Scripts/activate",
            "../.gitignore",
        ];
        for path in true_path_list {
            assert_eq!(include_hidden_directory(path), true);
        }
    }
}