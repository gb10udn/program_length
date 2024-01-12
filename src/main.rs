use std::io;

fn main() {
    let user_input = obtain_user_input();
    let user_input = remove_head_and_tail_double_quotation(user_input);
    println!("user_input: {}", user_input);
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