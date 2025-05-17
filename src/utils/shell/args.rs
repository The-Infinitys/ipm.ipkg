// args.rs
use colored::Colorize;
use std::env;
use std::fmt;

// 引数の種類を表す列挙型
#[derive(Debug, Clone)]
pub enum ArgumentType {
    Simple,   // プレーンな引数（例: "file.txt"）
    ShortOpt, // 短いオプション（例: "-v"）
    LongOpt,  // 長いオプション（例: "--verbose"）
}

// 引数の情報を保持する構造体
#[derive(Debug, Clone)]
pub struct Argument {
    pub arg_type: ArgumentType,
    pub arg_str: String,         // 引数の生の文字列（例: "--data", "-v", "-i"）
    pub arg_values: Vec<String>, // 引数の値（例: ["data1", "data2", "data3"]）。ShortOptの場合は通常空。
}

// コマンド全体を表す構造体
#[derive(Debug)]
pub struct Command {
    pub cmd_name: String,    // コマンド名（例: "my_program"）
    pub args: Vec<Argument>, // 引数のリスト
}

// DisplayトレイトをCommandに実装（カラー化）
impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // コマンド名を青色で表示
        writeln!(f, "{}: {}", "Command".cyan().bold(), self.cmd_name.blue())?;

        // 引数がなければその旨を赤色で表示
        if self.args.is_empty() {
            return writeln!(f, "{}", "No arguments provided.".red());
        }

        // 引数リストのヘッダーを緑色で表示
        writeln!(f, "{}:", "Arguments".green().bold())?;
        for (i, arg) in self.args.iter().enumerate() {
            let arg_type = match arg.arg_type {
                ArgumentType::Simple => "Simple".purple(),
                ArgumentType::ShortOpt => "Short Option".yellow(),
                ArgumentType::LongOpt => "Long Option".cyan(),
            };
            let values = if arg.arg_values.is_empty() {
                "None".red().to_string()
            } else {
                format!("[{}]", arg.arg_values.join(", ").green())
            };
            writeln!(
                f,
                "  {}. {} ({}: {}): {}: {}",
                (i + 1).to_string().bold(),
                arg.arg_str.magenta(),
                "Type".cyan(),
                arg_type,
                "Values".cyan(),
                values
            )?;
        }
        Ok(())
    }
}

impl Command {
    // 新しいCommandインスタンスを作成
    pub fn new(cmd_name: String) -> Self {
        Command {
            cmd_name,
            args: Vec::new(),
        }
    }

    // 引数を追加
    pub fn add_arg(&mut self, arg: Argument) {
        self.args.push(arg);
    }
}

// コマンドライン引数を取得
fn get_args() -> Vec<String> {
    env::args().collect()
}

// 引数の種類を判定
fn determine_arg_type(arg: &str) -> ArgumentType {
    if arg.starts_with("--") {
        ArgumentType::LongOpt
    } else if arg.starts_with("-") && arg.len() > 1 { // '-' だけは ShortOpt としない
         ArgumentType::ShortOpt
    } else {
        ArgumentType::Simple
    }
}

// カンマ区切りの値を分割（例: "data1,data2,data3" -> ["data1", "data2", "data3"]）
fn parse_values(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

// 引数を初期化してCommandを返す
pub fn init() -> Command {
    let args = get_args();
    let mut command = if let Some(cmd_name) = args.first() {
        // 最初の引数はコマンド名（プログラム名）
        Command::new(cmd_name.clone())
    } else {
        Command::new(String::new())
    };

    let mut i = 1; // コマンド名を除く最初の引数から開始
    while i < args.len() {
        let arg_str = &args[i];
        let arg_type = determine_arg_type(arg_str);

        // 長いオプションで値が付いている場合（例: "--data=data1,data2,data3"）
        if let ArgumentType::LongOpt = arg_type {
            if let Some((key, value)) = arg_str.split_once('=') {
                let arg_values = parse_values(value);
                 command.add_arg(Argument {
                    arg_type,
                    arg_str: key.to_string(), // キー部分だけをarg_strとして保存
                    arg_values,
                });
                i += 1; // この引数は処理済み
                continue; // 次の引数へ
            }
        }

        // 短いオプションが複数連なっている場合（例: "-iv"）
        if let ArgumentType::ShortOpt = arg_type {
             if arg_str.len() > 2 { // 長さが2より大きい場合（例: "-iv" は長さ3）
                 // '-' に続く各文字を個別の短いオプションとして追加
                 for c in arg_str.chars().skip(1) { // 最初の '-' をスキップ
                     let short_opt_str = format!("-{}", c);
                     command.add_arg(Argument {
                         arg_type: ArgumentType::ShortOpt,
                         arg_str: short_opt_str,
                         arg_values: Vec::new(), // 連結された短いオプションは通常値を持たない
                     });
                 }
                 i += 1; // この引数は処理済み
                 continue; // 次の引数へ
             }
        }

        // 上記のどのパターンにも当てはまらない場合（シンプルな引数、単一の短いオプション "-v", 値のない長いオプション "--help"）
        // これらは単一のArgumentとして追加
        let arg_values = Vec::new(); // これらのケースでは値のパースは行わない
        command.add_arg(Argument {
            arg_type,
            arg_str: arg_str.to_string(),
            arg_values,
        });
        i += 1; // この引数は処理済み
    }

    command
}

/*
// main.rs などで以下のように使用できます

mod args;

fn main() {
    let command = args::init();
    println!("{}", command);
}

// 実行例:
// cargo run -- -iv file.txt --data=a,b --verbose
//
// 出力例:
// Command: target/debug/your_program_name
// Arguments:
//   1. -i (Type: Short Option): Values: None
//   2. -v (Type: Short Option): Values: None
//   3. file.txt (Type: Simple): Values: None
//   4. --data (Type: Long Option): Values: [a, b]
//   5. --verbose (Type: Long Option): Values: None
*/